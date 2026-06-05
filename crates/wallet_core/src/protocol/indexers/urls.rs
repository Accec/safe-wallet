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
    let path = endpoint.path().trim_end_matches('/').to_string();
    if path.ends_with("/transfer") {
        let mut url = endpoint;
        ensure_query_pair(&mut url, "address", address);
        ensure_query_pair(&mut url, "sort", "-timestamp");
        ensure_query_pair(&mut url, "start", "0");
        ensure_query_pair(&mut url, "limit", "100");
        return Ok(vec![url]);
    }
    if path.ends_with("/token_trc20/transfers") {
        let mut url = endpoint;
        ensure_query_pair(&mut url, "relatedAddress", address);
        ensure_query_pair(&mut url, "sort", "-timestamp");
        ensure_query_pair(&mut url, "start", "0");
        ensure_query_pair(&mut url, "limit", "100");
        return Ok(vec![url]);
    }
    let mut native = endpoint.clone();
    native.set_path(&format!("{path}/transfer"));
    ensure_query_pair(&mut native, "address", address);
    ensure_query_pair(&mut native, "sort", "-timestamp");
    ensure_query_pair(&mut native, "start", "0");
    ensure_query_pair(&mut native, "limit", "100");

    let mut trc20 = endpoint;
    trc20.set_path(&format!("{path}/token_trc20/transfers"));
    ensure_query_pair(&mut trc20, "relatedAddress", address);
    ensure_query_pair(&mut trc20, "sort", "-timestamp");
    ensure_query_pair(&mut trc20, "start", "0");
    ensure_query_pair(&mut trc20, "limit", "100");
    Ok(vec![native, trc20])
}

fn has_query_key(url: &Url, key: &str) -> bool {
    url.query_pairs().any(|(existing, _)| existing == key)
}

fn ensure_query_pair(url: &mut Url, key: &str, value: &str) {
    if !has_query_key(url, key) {
        url.query_pairs_mut().append_pair(key, value);
    }
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
    fn tronscan_urls_include_native_and_trc20_transfer_endpoints() {
        let urls = activity_urls(
            "https://apilist.tronscanapi.com/api",
            ChainId::Tron,
            "TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7",
        )
        .unwrap();
        let native = urls[0].as_str();
        let trc20 = urls[1].as_str();

        assert_eq!(urls.len(), 2);
        assert!(native.contains("/api/transfer?"));
        assert!(native.contains("address=TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7"));
        assert!(trc20.contains("/api/token_trc20/transfers?"));
        assert!(trc20.contains("relatedAddress=TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7"));
    }
}
