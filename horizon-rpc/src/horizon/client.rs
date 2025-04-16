use anyhow::Result;
use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;
use url::Url;

use crate::models::horizon::{LedgerResponse, Response, RootResponse};

#[derive(Debug, Clone)]
pub struct HorizonClient {
    base_url: Url,
    client: ReqwestClient,
}

impl HorizonClient {
    pub fn new(base_url: String) -> Result<Self> {
        let base_url = Url::parse(&base_url)?;
        let client = ReqwestClient::new();
        Ok(Self { base_url, client })
    }

    pub async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.base_url.join(path)?;
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            anyhow::bail!("HTTP error {}: {}", status, error_text);
        }
        
        let data = response.json::<T>().await?;
        Ok(data)
    }

    pub async fn get_root(&self) -> Result<RootResponse> {
        self.get::<RootResponse>("").await
    }

    pub async fn get_latest_ledger(&self) -> Result<LedgerResponse> {
        let response: Response<LedgerResponse> = self.get("ledgers?order=desc&limit=1").await?;
        if let Some(embedded) = response._embedded {
            if let Some(record) = embedded.records.into_iter().next() {
                return Ok(record);
            }
        }
        anyhow::bail!("No ledger found in response")
    }

    pub async fn get_ledger(&self, sequence: u32) -> Result<LedgerResponse> {
        let path = format!("ledgers/{}", sequence);
        self.get::<LedgerResponse>(&path).await
    }

    pub async fn get_ledgers(&self, cursor: Option<String>, limit: Option<u32>, order: Option<&str>) -> Result<Response<LedgerResponse>> {
        let mut params = Vec::new();
        
        if let Some(cursor) = cursor {
            params.push(format!("cursor={}", cursor));
        }
        
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        
        if let Some(order) = order {
            params.push(format!("order={}", order));
        }
        
        let query = if params.is_empty() {
            "ledgers".to_string()
        } else {
            format!("ledgers?{}", params.join("&"))
        };
        
        self.get::<Response<LedgerResponse>>(&query).await
    }

    // Add more methods to interact with other Horizon endpoints as needed
    // For example, for accounts, transactions, operations, effects, etc.
}