// Models for both the JSON-RPC interface and Horizon API responses

use serde::{Deserialize, Serialize};

// JSON-RPC Response Models
pub mod rpc {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Health {
        pub status: String,
        pub horizon_status: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Ledger {
        pub hash: String,
        pub sequence: u32,
        pub closed_at: String,
        pub successful_transaction_count: u32,
        pub failed_transaction_count: u32,
        pub operation_count: u32,
        pub tx_set_operation_count: u32,
        pub protocol_version: u32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Network {
        pub passphrase: String,
        pub protocol_version: u32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Event {
        pub type_: String,
        pub ledger: u32,
        pub ledger_closed_at: String,
        pub contract_id: String,
        pub id: String,
        pub paging_token: String,
        pub in_successful_contract_call: bool,
        pub topic: Vec<String>,
        pub value: serde_json::Value,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetEventsResult {
        pub latest_ledger: u32,
        pub events: Vec<Event>,
        pub cursor: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LedgerEntry {
        pub key: String,
        pub xdr: String,
        pub last_modified_ledger: u32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetLedgerEntriesResult {
        pub entries: Vec<LedgerEntry>,
        pub latest_ledger: u32,
    }
}

// Horizon API Response Models
pub mod horizon {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct Response<T> {
        pub _embedded: Option<Embedded<T>>,
        pub _links: Option<Links>,
        #[serde(flatten)]
        pub data: Option<T>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Embedded<T> {
        pub records: Vec<T>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Links {
        pub next: Option<Link>,
        pub prev: Option<Link>,
        pub self_: Option<Link>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Link {
        pub href: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct LedgerResponse {
        pub id: String,
        pub paging_token: String,
        pub hash: String,
        pub sequence: u32,
        pub successful_transaction_count: u32,
        pub failed_transaction_count: u32,
        pub operation_count: u32,
        pub tx_set_operation_count: Option<u32>,
        pub closed_at: String,
        pub total_coins: String,
        pub fee_pool: String,
        pub base_fee_in_stroops: u32,
        pub base_reserve_in_stroops: u32,
        pub max_tx_set_size: u32,
        pub protocol_version: u32,
    }

    #[derive(Debug, Deserialize)]
    pub struct EffectResponse {
        pub id: String,
        pub paging_token: String,
        pub account: String,
        pub type_: String,
        pub type_i: u32,
        pub created_at: String,
        // Additional fields can be added as needed
    }

    #[derive(Debug, Deserialize)]
    pub struct AccountResponse {
        pub id: String,
        pub account_id: String,
        pub sequence: String,
        pub subentry_count: u32,
        pub balances: Vec<Balance>,
        // Additional fields can be added as needed
    }

    #[derive(Debug, Deserialize)]
    pub struct Balance {
        pub balance: String,
        pub asset_type: String,
        pub asset_code: Option<String>,
        pub asset_issuer: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct RootResponse {
        pub horizon_version: String,
        pub core_version: String,
        pub ingest_latest_ledger: u32,
        pub history_latest_ledger: u32,
        pub history_latest_ledger_closed_at: String,
        pub network_passphrase: String,
    }
}