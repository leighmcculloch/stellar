use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::{
    collections::HashSet,
    env,
    fs::{self},
    io::Cursor,
    path::Path,
};
use stellar_xdr::curr::{
    BucketEntry, ConfigSettingEntry, ConfigSettingId, Frame, LedgerEntry, LedgerEntryData,
    LedgerKey, LedgerKeyAccount, LedgerKeyClaimableBalance, LedgerKeyConfigSetting,
    LedgerKeyContractCode, LedgerKeyContractData, LedgerKeyData, LedgerKeyLiquidityPool,
    LedgerKeyOffer, LedgerKeyTrustLine, LedgerKeyTtl, Limited, Limits, ReadXdr, WriteXdr,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: bucket-sponsored-search <sponsor> <bucket-file> [bucket-file...]");
        std::process::exit(1);
    }

    let sponsor = args
        .get(2)
        .expect("the first argument should be the sponsor G address");
    let files = args.par_iter().skip(2);
    let files_len = files.len();
    files.progress_count(files_len as u64).for_each(|path| {
        if let Err(e) = process_file(Path::new(path), sponsor) {
            eprintln!("Error processing {}: {}", path, e);
        }
    });

    Ok(())
}

fn process_file(path: &Path, sponsor: &str) -> Result<()> {
    let contents = fs::read(path).map_err(|e| format!("Failed to read input file: {}", e))?;
    let mut limited_contents = Limited::new(Cursor::new(contents), Limits::none());
    let iter = Frame::<BucketEntry>::read_xdr_iter(&mut limited_contents);

    let mut seen = HashSet::<LedgerKey>::new();

    for entry in iter {
        let Frame(entry) = match entry {
            Ok(entry) => entry,
            Err(e) => {
                return Err(format!("Failed to read entry: {:?}", e).into());
            }
        };
        let (key, sponsored) = match &entry {
            BucketEntry::Initentry(entry) | BucketEntry::Liveentry(entry) => (
                data_into_key(entry),
                match &entry.ext {
                    stellar_xdr::curr::LedgerEntryExt::V0 => false,
                    stellar_xdr::curr::LedgerEntryExt::V1(ext) => match &ext.sponsoring_id.0 {
                        Some(sponsoring_id) => sponsoring_id.to_string() == sponsor,
                        None => false,
                    },
                },
            ),
            BucketEntry::Deadentry(key) => (key.clone(), false),
            BucketEntry::Metaentry(_) => continue,
        };
        if seen.contains(&key) {
            continue;
        }
        seen.insert(key);
        if sponsored {
            println!("{}", entry.to_xdr_base64(Limits::none())?);
        }
    }
    Ok(())
}

fn data_into_key(d: &LedgerEntry) -> LedgerKey {
    // TODO: Remove when
    match &d.data {
        LedgerEntryData::Account(e) => LedgerKey::Account(LedgerKeyAccount {
            account_id: e.account_id.clone(),
        }),
        LedgerEntryData::Trustline(e) => LedgerKey::Trustline(LedgerKeyTrustLine {
            account_id: e.account_id.clone(),
            asset: e.asset.clone(),
        }),
        LedgerEntryData::Offer(e) => LedgerKey::Offer(LedgerKeyOffer {
            seller_id: e.seller_id.clone(),
            offer_id: e.offer_id,
        }),
        LedgerEntryData::Data(e) => LedgerKey::Data(LedgerKeyData {
            account_id: e.account_id.clone(),
            data_name: e.data_name.clone(),
        }),
        LedgerEntryData::ClaimableBalance(e) => {
            LedgerKey::ClaimableBalance(LedgerKeyClaimableBalance {
                balance_id: e.balance_id.clone(),
            })
        }
        LedgerEntryData::LiquidityPool(e) => LedgerKey::LiquidityPool(LedgerKeyLiquidityPool {
            liquidity_pool_id: e.liquidity_pool_id.clone(),
        }),
        LedgerEntryData::ContractData(e) => LedgerKey::ContractData(LedgerKeyContractData {
            contract: e.contract.clone(),
            key: e.key.clone(),
            durability: e.durability,
        }),
        LedgerEntryData::ContractCode(e) => LedgerKey::ContractCode(LedgerKeyContractCode {
            hash: e.hash.clone(),
        }),
        LedgerEntryData::ConfigSetting(e) => LedgerKey::ConfigSetting(LedgerKeyConfigSetting {
            config_setting_id: match e {
                ConfigSettingEntry::ContractMaxSizeBytes(_) => {
                    ConfigSettingId::ContractMaxSizeBytes
                }
                ConfigSettingEntry::ContractComputeV0(_) => ConfigSettingId::ContractComputeV0,
                ConfigSettingEntry::ContractLedgerCostV0(_) => {
                    ConfigSettingId::ContractLedgerCostV0
                }
                ConfigSettingEntry::ContractHistoricalDataV0(_) => {
                    ConfigSettingId::ContractHistoricalDataV0
                }
                ConfigSettingEntry::ContractEventsV0(_) => ConfigSettingId::ContractEventsV0,
                ConfigSettingEntry::ContractBandwidthV0(_) => ConfigSettingId::ContractBandwidthV0,
                ConfigSettingEntry::ContractCostParamsCpuInstructions(_) => {
                    ConfigSettingId::ContractCostParamsCpuInstructions
                }
                ConfigSettingEntry::ContractCostParamsMemoryBytes(_) => {
                    ConfigSettingId::ContractCostParamsMemoryBytes
                }
                ConfigSettingEntry::ContractDataKeySizeBytes(_) => {
                    ConfigSettingId::ContractDataKeySizeBytes
                }
                ConfigSettingEntry::ContractDataEntrySizeBytes(_) => {
                    ConfigSettingId::ContractDataEntrySizeBytes
                }
                ConfigSettingEntry::StateArchival(_) => ConfigSettingId::StateArchival,
                ConfigSettingEntry::ContractExecutionLanes(_) => {
                    ConfigSettingId::ContractExecutionLanes
                }
                ConfigSettingEntry::BucketlistSizeWindow(_) => {
                    ConfigSettingId::BucketlistSizeWindow
                }
                ConfigSettingEntry::EvictionIterator(_) => ConfigSettingId::EvictionIterator,
            },
        }),
        LedgerEntryData::Ttl(e) => LedgerKey::Ttl(LedgerKeyTtl {
            key_hash: e.key_hash.clone(),
        }),
    }
}
