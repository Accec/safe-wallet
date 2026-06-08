use crate::error::WalletError;
use crate::models::{ActivityRecord, ChainId};
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use reqwest::Url;
use serde_json::Value;
use std::time::Duration;

mod parsing;
mod urls;

const ACTIVITY_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) SafeWallet/0.1";
const MAX_PAGINATED_ACTIVITY_PAGES: usize = 25;

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
            let mut next_url = Some(url);
            for _ in 0..MAX_PAGINATED_ACTIVITY_PAGES {
                let Some(url) = next_url.take() else {
                    break;
                };
                let body = self
                    .client
                    .get(url.clone())
                    .header(USER_AGENT, ACTIVITY_USER_AGENT)
                    .send()
                    .and_then(|response| response.error_for_status())
                    .map_err(|_| WalletError::NetworkUnavailable)?
                    .text()
                    .map_err(|_| WalletError::NetworkUnavailable)?;
                records.extend(parsing::parse_activity_response(chain, &body)?);
                next_url = next_activity_page_url(&url, &body);
            }
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

fn next_activity_page_url(url: &Url, body: &str) -> Option<Url> {
    let start = query_value(url, "start")?.parse::<usize>().ok()?;
    let limit = query_value(url, "limit")?.parse::<usize>().ok()?;
    if limit == 0 {
        return None;
    }
    let body = serde_json::from_str::<Value>(body).ok()?;
    let row_count = activity_row_count(&body)?;
    if row_count < limit {
        return None;
    }
    let next_start = start.checked_add(limit)?;
    if activity_total(&body).is_some_and(|total| next_start >= total) {
        return None;
    }

    let mut next = url.clone();
    set_query_pair(&mut next, "start", &next_start.to_string());
    Some(next)
}

fn query_value(url: &Url, key: &str) -> Option<String> {
    url.query_pairs()
        .find_map(|(existing, value)| (existing == key).then(|| value.into_owned()))
}

fn activity_total(body: &Value) -> Option<usize> {
    body.get("rangeTotal")
        .or_else(|| body.get("total"))
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
}

fn activity_row_count(body: &Value) -> Option<usize> {
    if let Some(rows) = body.as_array() {
        return Some(rows.len());
    }
    ["data", "token_transfers", "result"]
        .into_iter()
        .find_map(|key| body.get(key).and_then(Value::as_array).map(Vec::len))
}

fn set_query_pair(url: &mut Url, key: &str, value: &str) {
    let pairs = url
        .query_pairs()
        .filter(|(existing, _)| existing != key)
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect::<Vec<_>>();
    url.set_query(None);
    {
        let mut query = url.query_pairs_mut();
        for (key, value) in pairs {
            query.append_pair(&key, &value);
        }
        query.append_pair(key, value);
    }
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

    #[test]
    fn tronscan_activity_pages_continue_until_total_rows_are_fetched() {
        let url = reqwest::Url::parse(
            "https://apilist.tronscan.org/api/transaction?start=0&limit=20&address=T",
        )
        .unwrap();
        let body = serde_json::json!({
            "total": 45,
            "data": vec![serde_json::json!({"hash": "tx"}); 20]
        })
        .to_string();

        let next = next_activity_page_url(&url, &body).unwrap();

        assert!(next.as_str().contains("start=20"));
        assert!(next.as_str().contains("limit=20"));
    }

    #[test]
    fn tronscan_activity_pages_stop_on_short_final_page() {
        let url = reqwest::Url::parse(
            "https://apilist.tronscan.org/api/filter/trc20/transfers?start=40&limit=20&relatedAddress=T",
        )
        .unwrap();
        let body = serde_json::json!({
            "total": 45,
            "token_transfers": vec![serde_json::json!({"transaction_id": "tx"}); 5]
        })
        .to_string();

        assert!(next_activity_page_url(&url, &body).is_none());
    }
}
