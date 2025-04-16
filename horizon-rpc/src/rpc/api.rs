use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::error::ErrorObject;
use serde_json::Value;

use crate::horizon::HorizonClient;
use crate::models::rpc::{GetEventsResult, GetLedgerEntriesResult, Health, Ledger, Network};

#[rpc(server)]
pub trait StellarRpcApi {
    #[method(name = "getHealth")]
    async fn get_health(&self) -> RpcResult<Health>;

    #[method(name = "getNetwork")]
    async fn get_network(&self) -> RpcResult<Network>;

    #[method(name = "getLatestLedger")]
    async fn get_latest_ledger(&self) -> RpcResult<Ledger>;

    #[method(name = "getLedgers")]
    async fn get_ledgers(
        &self,
        cursor: Option<String>,
        limit: Option<u32>,
    ) -> RpcResult<Vec<Ledger>>;

    #[method(name = "getLedgerEntries")]
    async fn get_ledger_entries(
        &self,
        keys: Vec<String>,
    ) -> RpcResult<GetLedgerEntriesResult>;

    #[method(name = "getEvents")]
    async fn get_events(
        &self,
        start_ledger: Option<u32>,
        end_ledger: Option<u32>,
        filters: Option<Vec<Value>>,
        pagination: Option<Value>,
        xdr_format: Option<String>,
    ) -> RpcResult<GetEventsResult>;
}

pub struct StellarRpcServer {
    horizon_client: HorizonClient,
}

impl StellarRpcServer {
    pub fn new(horizon_url: String) -> anyhow::Result<Self> {
        let horizon_client = HorizonClient::new(horizon_url)?;
        Ok(Self { horizon_client })
    }

    fn rpc_error(msg: String) -> ErrorObject<'static> {
        ErrorObject::owned(
            2001, // Custom error code
            msg,
            None::<()>,
        )
    }
}

#[async_trait]
impl StellarRpcApiServer for StellarRpcServer {
    async fn get_health(&self) -> RpcResult<Health> {
        match self.horizon_client.get_root().await {
            Ok(root) => Ok(Health {
                status: "healthy".to_string(),
                horizon_status: format!("Horizon {}, Core {}", root.horizon_version, root.core_version),
            }),
            Err(e) => Ok(Health {
                status: "error".to_string(),
                horizon_status: format!("Error connecting to Horizon: {}", e),
            }),
        }
    }

    async fn get_network(&self) -> RpcResult<Network> {
        match self.horizon_client.get_root().await {
            Ok(root) => Ok(Network {
                passphrase: root.network_passphrase,
                protocol_version: root.ingest_latest_ledger, // This is an approximation
            }),
            Err(e) => Err(Self::rpc_error(format!("Failed to get network info: {}", e))),
        }
    }

    async fn get_latest_ledger(&self) -> RpcResult<Ledger> {
        match self.horizon_client.get_latest_ledger().await {
            Ok(ledger) => Ok(Ledger {
                hash: ledger.hash,
                sequence: ledger.sequence,
                closed_at: ledger.closed_at,
                successful_transaction_count: ledger.successful_transaction_count,
                failed_transaction_count: ledger.failed_transaction_count,
                operation_count: ledger.operation_count,
                tx_set_operation_count: ledger.tx_set_operation_count.unwrap_or(0),
                protocol_version: ledger.protocol_version,
            }),
            Err(e) => Err(Self::rpc_error(format!("Failed to get latest ledger: {}", e))),
        }
    }

    async fn get_ledgers(
        &self,
        cursor: Option<String>,
        limit: Option<u32>,
    ) -> RpcResult<Vec<Ledger>> {
        match self.horizon_client.get_ledgers(cursor, limit, Some("desc")).await {
            Ok(response) => {
                if let Some(embedded) = response._embedded {
                    let ledgers = embedded.records.into_iter().map(|l| Ledger {
                        hash: l.hash,
                        sequence: l.sequence,
                        closed_at: l.closed_at,
                        successful_transaction_count: l.successful_transaction_count,
                        failed_transaction_count: l.failed_transaction_count,
                        operation_count: l.operation_count,
                        tx_set_operation_count: l.tx_set_operation_count.unwrap_or(0),
                        protocol_version: l.protocol_version,
                    }).collect();
                    Ok(ledgers)
                } else {
                    Ok(Vec::new())
                }
            },
            Err(e) => Err(Self::rpc_error(format!("Failed to get ledgers: {}", e))),
        }
    }

    async fn get_ledger_entries(
        &self,
        _keys: Vec<String>,
    ) -> RpcResult<GetLedgerEntriesResult> {
        // This is a placeholder implementation
        // Horizon doesn't directly support getting arbitrary ledger entries
        // This would need to be implemented based on the specific entries needed
        Err(Self::rpc_error("Not implemented".to_string()))
    }

    async fn get_events(
        &self,
        _start_ledger: Option<u32>,
        _end_ledger: Option<u32>,
        _filters: Option<Vec<Value>>,
        _pagination: Option<Value>,
        _xdr_format: Option<String>,
    ) -> RpcResult<GetEventsResult> {
        // This is a placeholder implementation
        // Horizon has effects endpoint, but mapping to events needs careful implementation
        Err(Self::rpc_error("Not implemented".to_string()))
    }
}