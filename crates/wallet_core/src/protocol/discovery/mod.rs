use crate::error::WalletError;
use crate::models::{ChainId, DiscoveredAsset, NetworkPrivacySettings};
use reqwest::blocking::Client;
use serde_json::Value;
use std::time::Duration;

mod parsing;
mod urls;

pub trait AssetDiscoveryProvider {
    fn discover_assets(
        &self,
        chain: ChainId,
        endpoint: &str,
        address: &str,
    ) -> Result<Vec<DiscoveredAsset>, WalletError>;
}

pub struct HttpAssetDiscoveryProvider {
    client: Client,
}

impl HttpAssetDiscoveryProvider {
    pub fn new(settings: &NetworkPrivacySettings) -> Result<Self, WalletError> {
        let client =
            crate::protocol::network::build_http_client(settings, Duration::from_secs(20))?;
        Ok(Self { client })
    }
}

impl AssetDiscoveryProvider for HttpAssetDiscoveryProvider {
    fn discover_assets(
        &self,
        chain: ChainId,
        endpoint: &str,
        address: &str,
    ) -> Result<Vec<DiscoveredAsset>, WalletError> {
        let mut discovered = Vec::new();
        let mut saw_success = false;
        for url in urls::discovery_urls(endpoint, chain, address)? {
            let response = self.client.get(url).send();
            let Ok(response) = response else {
                continue;
            };
            let Ok(response) = response.error_for_status() else {
                continue;
            };
            let Ok(body) = response.json::<Value>() else {
                continue;
            };
            let Ok(mut parsed) = parsing::parse_discovery_body(chain, &body) else {
                continue;
            };
            saw_success = true;
            discovered.append(&mut parsed);
        }
        if !saw_success {
            return Err(WalletError::NetworkUnavailable);
        }
        Ok(parsing::deduplicate_contracts(discovered))
    }
}

pub fn default_discovery_endpoint(chain: ChainId) -> Option<&'static str> {
    match chain {
        ChainId::Ethereum
        | ChainId::Bsc
        | ChainId::Polygon
        | ChainId::Arbitrum
        | ChainId::Optimism => Some("https://api.etherscan.io/v2/api"),
        ChainId::Tron => Some("https://apilist.tronscanapi.com/api"),
        ChainId::Btc => None,
    }
}
