use crate::error::WalletError;
use crate::models::{ActivityRecord, ChainId};
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use std::time::Duration;

mod parsing;
mod urls;

const ACTIVITY_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) SafeWallet/0.1";

pub trait ActivityIndexer {
    fn fetch_activity(
        &self,
        chain: ChainId,
        endpoint: &str,
        address: &str,
    ) -> Result<Vec<ActivityRecord>, WalletError>;
}

pub struct HttpActivityIndexer {
    client: Client,
}

impl HttpActivityIndexer {
    pub fn new(settings: &crate::models::NetworkPrivacySettings) -> Result<Self, WalletError> {
        let client =
            crate::protocol::network::build_http_client(settings, Duration::from_secs(20))?;
        Ok(Self { client })
    }
}

impl ActivityIndexer for HttpActivityIndexer {
    fn fetch_activity(
        &self,
        chain: ChainId,
        endpoint: &str,
        address: &str,
    ) -> Result<Vec<ActivityRecord>, WalletError> {
        let mut records = Vec::new();
        for url in urls::activity_urls(endpoint, chain, address)? {
            let body = self
                .client
                .get(url)
                .header(USER_AGENT, ACTIVITY_USER_AGENT)
                .send()
                .and_then(|response| response.error_for_status())
                .map_err(|_| WalletError::NetworkUnavailable)?
                .text()
                .map_err(|_| WalletError::NetworkUnavailable)?;
            records.extend(parsing::parse_activity_response(chain, &body)?);
        }
        Ok(records)
    }
}

pub fn default_activity_endpoint(chain: ChainId) -> Option<&'static str> {
    match chain {
        ChainId::Ethereum => Some("https://etherscan.io"),
        ChainId::Bsc => Some("https://bscscan.com"),
        ChainId::Polygon => Some("https://polygonscan.com"),
        ChainId::Arbitrum => Some("https://arbiscan.io"),
        ChainId::Optimism => Some("https://optimistic.etherscan.io"),
        ChainId::Tron => Some("https://apilist.tronscan.org/api"),
        ChainId::Btc => None,
    }
}

pub fn fetch_indexed_activity<I: ActivityIndexer>(
    indexer: &I,
    chain: ChainId,
    endpoint: &str,
    address: &str,
) -> Result<Vec<ActivityRecord>, WalletError> {
    indexer.fetch_activity(chain, endpoint, address)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ActivityKind, ActivityStatus};

    struct FakeIndexer;

    impl ActivityIndexer for FakeIndexer {
        fn fetch_activity(
            &self,
            chain: ChainId,
            _endpoint: &str,
            address: &str,
        ) -> Result<Vec<ActivityRecord>, WalletError> {
            Ok(vec![ActivityRecord {
                chain,
                tx_hash: format!("fake-tx-for-{address}"),
                kind: ActivityKind::TokenTransfer,
                status: ActivityStatus::Confirmed,
                summary: "Received USDC".to_string(),
            }])
        }
    }

    #[test]
    fn indexed_activity_is_loaded_from_indexer() {
        let records = fetch_indexed_activity(
            &FakeIndexer,
            ChainId::Ethereum,
            "https://example.invalid/indexer",
            "0x0000000000000000000000000000000000000000",
        )
        .unwrap();

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].chain, ChainId::Ethereum);
        assert_eq!(records[0].kind, ActivityKind::TokenTransfer);
        assert_eq!(records[0].status, ActivityStatus::Confirmed);
    }

    #[test]
    fn activity_indexer_accepts_socks_proxy_settings() {
        let indexer = HttpActivityIndexer::new(&crate::models::NetworkPrivacySettings {
            proxy_enabled: true,
            proxy_mode: crate::models::ProxyMode::Custom,
            proxy_url: Some("socks5://127.0.0.1:9050".to_string()),
        });

        assert!(indexer.is_ok());
    }

    #[test]
    fn default_activity_endpoints_use_scan_pages_and_tronscan_transaction_api() {
        assert_eq!(
            default_activity_endpoint(ChainId::Ethereum),
            Some("https://etherscan.io")
        );
        assert_eq!(
            default_activity_endpoint(ChainId::Bsc),
            Some("https://bscscan.com")
        );
        assert_eq!(
            default_activity_endpoint(ChainId::Tron),
            Some("https://apilist.tronscan.org/api")
        );
        assert_eq!(default_activity_endpoint(ChainId::Btc), None);
    }
}
