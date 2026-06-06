use crate::error::WalletError;
use crate::models::{ChainId, DiscoveredAsset};
use serde_json::Value;

mod common;
mod etherscan;
mod scan_pages;
mod tronscan;

pub(super) fn parse_discovery_response(
    chain: ChainId,
    body: &str,
) -> Result<Vec<DiscoveredAsset>, WalletError> {
    if let Ok(body) = serde_json::from_str::<Value>(body) {
        return parse_discovery_body(chain, &body);
    }
    scan_pages::parse_token_transfer_html(chain, body).ok_or(WalletError::NetworkUnavailable)
}

pub(super) fn parse_discovery_body(
    chain: ChainId,
    body: &Value,
) -> Result<Vec<DiscoveredAsset>, WalletError> {
    let rows = common::discovery_rows(body)?;
    let mut discovered = Vec::new();
    for row in rows {
        let asset = if chain == ChainId::Tron {
            tronscan::parse_discovery_row(chain, row)
        } else {
            etherscan::parse_discovery_row(chain, row)
        };
        if let Some(asset) = asset {
            discovered.push(asset);
        }
    }
    Ok(common::deduplicate_contracts(discovered))
}

pub(super) fn deduplicate_contracts(discovered: Vec<DiscoveredAsset>) -> Vec<DiscoveredAsset> {
    common::deduplicate_contracts(discovered)
}

#[cfg(test)]
mod tests;
