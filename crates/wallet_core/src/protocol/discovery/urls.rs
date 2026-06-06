use crate::error::WalletError;
use crate::models::ChainId;
use reqwest::Url;

pub(super) fn discovery_urls(
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
    if chain == ChainId::Tron || is_tronscan_api(&url) {
        return tron_discovery_urls(url, address);
    }
    Ok(evm_discovery_urls(url, chain, address))
}

fn evm_discovery_urls(endpoint: Url, chain: ChainId, address: &str) -> Vec<Url> {
    let mut holding = endpoint.clone();
    ensure_query_pair(&mut holding, "module", "account");
    ensure_query_pair(&mut holding, "action", "addresstokenbalance");
    if let Some(chain_id) = etherscan_chain_id(chain) {
        ensure_query_pair(&mut holding, "chainid", chain_id);
    }
    ensure_query_pair(&mut holding, "address", address);
    ensure_query_pair(&mut holding, "page", "1");
    ensure_query_pair(&mut holding, "offset", "100");

    let mut transfers = endpoint;
    ensure_query_pair(&mut transfers, "module", "account");
    ensure_query_pair(&mut transfers, "action", "tokentx");
    if let Some(chain_id) = etherscan_chain_id(chain) {
        ensure_query_pair(&mut transfers, "chainid", chain_id);
    }
    ensure_query_pair(&mut transfers, "address", address);
    ensure_query_pair(&mut transfers, "sort", "desc");
    ensure_query_pair(&mut transfers, "page", "1");
    ensure_query_pair(&mut transfers, "offset", "100");

    vec![holding, transfers]
}

fn tron_discovery_urls(endpoint: Url, address: &str) -> Result<Vec<Url>, WalletError> {
    let mut url = endpoint;
    let path = url.path().trim_end_matches('/').to_string();
    if !path.ends_with("/filter/trc20/transfers") {
        let base_path = path
            .strip_suffix("/account/tokens")
            .or_else(|| path.strip_suffix("/token_trc20/transfers"))
            .or_else(|| path.strip_suffix("/transfer"))
            .unwrap_or(path.as_str());
        let base_path = if base_path.is_empty() {
            "/api"
        } else {
            base_path
        };
        url.set_path(&format!("{base_path}/filter/trc20/transfers"));
    }
    ensure_query_pair(&mut url, "relatedAddress", address);
    ensure_query_pair(&mut url, "start", "0");
    ensure_query_pair(&mut url, "limit", "100");
    ensure_query_pair(&mut url, "sort", "-timestamp");
    ensure_query_pair(&mut url, "count", "true");
    ensure_query_pair(&mut url, "filterTokenValue", "0");
    Ok(vec![url])
}

fn has_query_key(url: &Url, key: &str) -> bool {
    url.query_pairs().any(|(existing, _)| existing == key)
}

fn ensure_query_pair(url: &mut Url, key: &str, value: &str) {
    if !has_query_key(url, key) {
        url.query_pairs_mut().append_pair(key, value);
    }
}

fn is_tronscan_api(url: &Url) -> bool {
    url.host_str()
        .is_some_and(|host| host.contains("tronscanapi.com") || host.contains("tronscan.org"))
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
    fn tronscan_discovery_uses_tor_accessible_trc20_transfer_endpoint() {
        let urls = discovery_urls(
            "https://apilist.tronscan.org/api",
            ChainId::Tron,
            "TUxJcEDX8Srz3kYsv7oC4h3RWERhBk4QjJ",
        )
        .unwrap();

        assert_eq!(urls.len(), 1);
        let url = urls[0].as_str();
        assert!(url.contains("/api/filter/trc20/transfers?"));
        assert!(url.contains("relatedAddress=TUxJcEDX8Srz3kYsv7oC4h3RWERhBk4QjJ"));
        assert!(url.contains("filterTokenValue=0"));
        assert!(!url.contains("/account/tokens"));
    }
}
