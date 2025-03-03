use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::{
    env,
    fs::{self, File},
    io::{Cursor, Write},
    path::Path,
};
use stellar_xdr::curr::{
    BucketEntry, Frame, LedgerEntryData, LedgerKey, Limited, Limits, ReadXdr, WriteXdr,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: bucket-slimmer <bucket-file> [bucket-file...]");
        std::process::exit(1);
    }

    let files = args.par_iter().skip(1);
    let files_len = files.len();
    files.progress_count(files_len as u64).for_each(|path| {
        if let Err(e) = process_file(Path::new(path)) {
            eprintln!("Error processing {path}: {e}");
        }
    });
}

fn process_file(path: &Path) -> Result<()> {
    let contents = fs::read(path).map_err(|e| format!("Failed to read input file: {e}"))?;
    let mut limited_contents = Limited::new(Cursor::new(contents), Limits::none());
    let iter = Frame::<BucketEntry>::read_xdr_iter(&mut limited_contents);

    let slim_path = path.with_extension("slim");
    let mut slim_file =
        File::create(&slim_path).map_err(|e| format!("Failed to create output file: {e}"))?;

    for result_entry in iter {
        let framed_entry = match &result_entry {
            Ok(framed_entry) => framed_entry,
            Err(e) => {
                return Err(format!("Failed to read entry: {e:?}").into());
            }
        };
        let Frame(entry) = framed_entry;
        let keep = match entry {
            BucketEntry::Initentry(entry) | BucketEntry::Liveentry(entry) => match &entry.data {
                LedgerEntryData::ContractData(_)
                | LedgerEntryData::ContractCode(_)
                | LedgerEntryData::ConfigSetting(_)
                | LedgerEntryData::Trustline(_)
                | LedgerEntryData::Account(_) => true,
                LedgerEntryData::Ttl(_)
                | LedgerEntryData::Offer(_)
                | LedgerEntryData::Data(_)
                | LedgerEntryData::ClaimableBalance(_)
                | LedgerEntryData::LiquidityPool(_) => false,
            },
            BucketEntry::Deadentry(ledger_key) => match ledger_key {
                LedgerKey::Account(_)
                | LedgerKey::Trustline(_)
                | LedgerKey::ContractData(_)
                | LedgerKey::ContractCode(_)
                | LedgerKey::ConfigSetting(_) => true,
                LedgerKey::Offer(_)
                | LedgerKey::Data(_)
                | LedgerKey::ClaimableBalance(_)
                | LedgerKey::LiquidityPool(_)
                | LedgerKey::Ttl(_) => false,
            },
            BucketEntry::Metaentry(_) => true,
        };

        if keep {
            slim_file
                .write_all(
                    &entry
                        .to_xdr(Limits::none())
                        .map_err(|e| format!("Failed to encode entry as XDR: {e}"))?,
                )
                .map_err(|e| format!("Failed to write entry: {e}"))?;
        }
    }

    slim_file
        .flush()
        .map_err(|e| format!("Failed to flush output file: {e}"))?;

    Ok(())
}
