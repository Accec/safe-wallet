use crate::error::WalletError;
use crate::models::ChainId;
use reqwest::Url;

pub(super) fn activity_urls(
    endpoint: &str,
    chain: ChainId,
    address: &str,
) -> Result<Vec<Url>, WalletError> {
    let endpoint = endpoint.trim();
    if endpoint.is_empty() {
        return Err(WalletError::NetworkUnavailable);
    }
    if endpoint.contains("{address}") || endpoint.contains("{chain}") {
        let url = endpoint
            .replace("{address}", address)
            .replace("{chain}", chain_slug(chain));
        return Url::parse(&url)
            .map(|url| vec![url])
            .map_err(|_| WalletError::NetworkUnavailable);
    }
    let url = Url::parse(endpoint).map_err(|_| WalletError::NetworkUnavailable)?;
    if chain == ChainId::Tron && is_tronscan_api(&url) {
        return tronscan_activity_urls(url, address);
    }
    if is_evm_scan_web_url(chain, &url) {
        if url.path().trim_end_matches('/').ends_with("/tokentxns") {
            return Ok(vec![token_transfer_page_url(url, address)]);
        }
        if url.path().trim_end_matches('/').ends_with("/txs") {
            return Ok(vec![transaction_page_url(url, address)]);
        }
        return Ok(vec![
            transaction_page_url(url.clone(), address),
            token_transfer_page_url(url, address),
        ]);
    }
    if is_account_api(&url) && !has_query_key(&url, "action") {
        return ["txlist", "tokentx"]
            .into_iter()
            .map(|action| {
                let mut url = url.clone();
                if let Some(chain_id) = etherscan_chain_id(chain) {
                    ensure_query_pair(&mut url, "chainid", chain_id);
                }
                ensure_query_pair(&mut url, "module", "account");
                ensure_query_pair(&mut url, "action", action);
                ensure_query_pair(&mut url, "address", address);
                ensure_query_pair(&mut url, "sort", "desc");
                ensure_query_pair(&mut url, "page", "1");
                ensure_query_pair(&mut url, "offset", "100");
                Ok(url)
            })
            .collect();
    }
    let mut url = url;
    if is_account_api(&url) {
        if let Some(chain_id) = etherscan_chain_id(chain) {
            ensure_query_pair(&mut url, "chainid", chain_id);
        }
        ensure_query_pair(&mut url, "module", "account");
    } else {
        ensure_query_pair(&mut url, "chain", chain_slug(chain));
    }
    ensure_query_pair(&mut url, "address", address);
    Ok(vec![url])
}

fn is_account_api(url: &Url) -> bool {
    has_query_key(url, "module")
        || url.path().ends_with("/api")
        || url
            .host_str()
            .is_some_and(|host| host.contains("etherscan") || host.contains("bscscan"))
}

fn is_tronscan_api(url: &Url) -> bool {
    url.host_str()
        .is_some_and(|host| host.contains("tronscanapi.com") || host.contains("tronscan.org"))
}

fn tronscan_activity_urls(endpoint: Url, address: &str) -> Result<Vec<Url>, WalletError> {
    let mut url = endpoint;
    let path = url.path().trim_end_matches('/').to_string();
    if !path.ends_with("/transaction") {
        let base_path = path
            .strip_suffix("/transfer")
            .or_else(|| path.strip_suffix("/token_trc20/transfers"))
            .unwrap_or(path.as_str());
        let base_path = if base_path.is_empty() {
            "/api"
        } else {
            base_path
        };
        url.set_path(&format!("{base_path}/transaction"));
    }
    ensure_query_pair(&mut url, "address", address);
    ensure_query_pair(&mut url, "sort", "-timestamp");
    ensure_query_pair(&mut url, "count", "true");
    ensure_query_pair(&mut url, "start", "0");
    ensure_query_pair(&mut url, "limit", "20");
    Ok(vec![url])
}

fn token_transfer_page_url(mut url: Url, address: &str) -> Url {
    if !url.path().trim_end_matches('/').ends_with("/tokentxns") {
        url.set_path("/tokentxns");
    }
    set_query_pair(&mut url, "a", address);
    ensure_query_pair(&mut url, "p", "1");
    url
}

fn transaction_page_url(mut url: Url, address: &str) -> Url {
    if !url.path().trim_end_matches('/').ends_with("/txs") {
        url.set_path("/txs");
    }
    set_query_pair(&mut url, "a", address);
    ensure_query_pair(&mut url, "p", "1");
    url
}

fn has_query_key(url: &Url, key: &str) -> bool {
    url.query_pairs().any(|(existing, _)| existing == key)
}

fn ensure_query_pair(url: &mut Url, key: &str, value: &str) {
    if !has_query_key(url, key) {
        url.query_pairs_mut().append_pair(key, value);
    }
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

fn is_evm_scan_web_url(chain: ChainId, url: &Url) -> bool {
    let Some(host) = url.host_str() else {
        return false;
    };
    let expected_host = match chain {
        ChainId::Ethereum => "etherscan.io",
        ChainId::Bsc => "bscscan.com",
        ChainId::Polygon => "polygonscan.com",
        ChainId::Arbitrum => "arbiscan.io",
        ChainId::Optimism => "optimistic.etherscan.io",
        ChainId::Btc | ChainId::Tron => return false,
    };
    host == expected_host && !url.path().trim_end_matches('/').ends_with("/api")
}

fn chain_slug(chain: ChainId) -> &'static str {
    match chain {
        ChainId::Btc => "btc",
        ChainId::Ethereum => "ethereum",
        ChainId::Bsc => "bsc",
        ChainId::Polygon => "polygon",
        ChainId::Arbitrum => "arbitrum",
        ChainId::Optimism => "optimism",
        ChainId::Tron => "tron",
    }
}

fn etherscan_chain_id(chain: ChainId) -> Option<&'static str> {
    match chain {
        ChainId::Ethereum => Some("1"),
        ChainId::Bsc => Some("56"),
        ChainId::Polygon => Some("137"),
        ChainId::Arbitrum => Some("42161"),
        ChainId::Optimism => Some("10"),
        ChainId::Btc | ChainId::Tron => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn etherscan_v2_urls_include_chain_id_and_account_actions() {
        let urls = activity_urls(
            "https://api.etherscan.io/v2/api?apikey=key",
            ChainId::Polygon,
            "0xabc",
        )
        .unwrap();
        let first = urls[0].as_str();
        let second = urls[1].as_str();

        assert_eq!(urls.len(), 2);
        assert!(first.contains("chainid=137"));
        assert!(first.contains("action=txlist"));
        assert!(first.contains("address=0xabc"));
        assert!(second.contains("chainid=137"));
        assert!(second.contains("action=tokentx"));
    }

    #[test]
    fn evm_scan_web_urls_use_transaction_and_token_transfer_pages_for_activity() {
        let urls = activity_urls(
            "https://bscscan.com",
            ChainId::Bsc,
            "0x12b17178502c5b24d01d9a2089d2625f165acb2c",
        )
        .unwrap();

        assert_eq!(urls.len(), 2);
        let native = urls[0].as_str();
        let token = urls[1].as_str();
        assert!(native.contains("https://bscscan.com/txs?"), "{native}");
        assert!(native.contains("a=0x12b17178502c5b24d01d9a2089d2625f165acb2c"));
        assert!(native.contains("p=1"));
        assert!(token.contains("https://bscscan.com/tokentxns?"), "{token}");
        assert!(token.contains("a=0x12b17178502c5b24d01d9a2089d2625f165acb2c"));
        assert!(token.contains("p=1"));
        assert!(!native.contains("module=account"));
        assert!(!token.contains("action=tokentx"));
    }

    #[test]
    fn explicit_evm_token_transfer_page_stays_single_activity_url() {
        let urls = activity_urls(
            "https://bscscan.com/tokentxns?p=2",
            ChainId::Bsc,
            "0x12b17178502c5b24d01d9a2089d2625f165acb2c",
        )
        .unwrap();

        assert_eq!(urls.len(), 1);
        let url = urls[0].as_str();
        assert!(url.contains("https://bscscan.com/tokentxns?"), "{url}");
        assert!(url.contains("a=0x12b17178502c5b24d01d9a2089d2625f165acb2c"));
        assert!(url.contains("p=2"));
    }

    #[test]
    fn tronscan_activity_uses_transaction_api_shape() {
        let urls = activity_urls(
            "https://apilist.tronscan.org/api",
            ChainId::Tron,
            "TUxJcEDX8Srz3kYsv7oC4h3RWERhBk4QjJ",
        )
        .unwrap();

        assert_eq!(urls.len(), 1);
        let url = urls[0].as_str();
        assert!(url.contains("/api/transaction?"), "{url}");
        assert!(url.contains("address=TUxJcEDX8Srz3kYsv7oC4h3RWERhBk4QjJ"));
        assert!(url.contains("sort=-timestamp"));
        assert!(url.contains("count=true"));
        assert!(url.contains("limit=20"));
    }
}
