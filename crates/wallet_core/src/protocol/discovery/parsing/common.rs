use crate::error::WalletError;
use crate::models::{ChainId, DiscoveredAsset};
use serde_json::Value;
use std::collections::HashSet;

pub(super) fn discovery_rows(body: &Value) -> Result<&Vec<Value>, WalletError> {
    if let Some(rows) = body.as_array() {
        return Ok(rows);
    }
    for key in [
        "result",
        "data",
        "tokens",
        "token_transfers",
        "trc20token_balances",
        "withPriceTokens",
        "trc20_tokens",
    ] {
        if let Some(rows) = body.get(key).and_then(Value::as_array) {
            return Ok(rows);
        }
    }
    Err(WalletError::NetworkUnavailable)
}

pub(super) fn deduplicate_contracts(discovered: Vec<DiscoveredAsset>) -> Vec<DiscoveredAsset> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for asset in discovered {
        let key = format!(
            "{}:{}",
            chain_slug(asset.chain),
            asset.contract_address.to_lowercase()
        );
        if seen.insert(key) {
            deduped.push(asset);
        }
    }
    deduped
}

pub(super) fn first_string(row: &Value, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| string_field(row, key))
}

pub(super) fn string_field(row: &Value, key: &str) -> Option<String> {
    row.get(key).and_then(|value| {
        if let Some(value) = value.as_str() {
            return Some(value.to_string());
        }
        if value.is_number() {
            return Some(value.to_string());
        }
        None
    })
}

pub(super) fn format_units(value: &str, decimals: u8) -> String {
    let value = value.trim();
    if value.is_empty() || value == "0" {
        return "0".to_string();
    }
    if value.contains('.') {
        return value.to_string();
    }
    let decimals = decimals as usize;
    if decimals == 0 {
        return value.to_string();
    }
    if value.len() <= decimals {
        let zeros = "0".repeat(decimals - value.len());
        let trimmed = format!("0.{zeros}{value}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string();
        return if trimmed.is_empty() {
            "0".to_string()
        } else {
            trimmed
        };
    }
    let split = value.len() - decimals;
    let whole = &value[..split];
    let fraction = value[split..].trim_end_matches('0');
    if fraction.is_empty() {
        whole.to_string()
    } else {
        format!("{whole}.{fraction}")
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
