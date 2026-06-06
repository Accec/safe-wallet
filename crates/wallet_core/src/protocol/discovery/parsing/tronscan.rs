use super::common::{first_string, format_units};
use crate::models::{AssetKind, ChainId, DiscoveredAsset};
use serde_json::Value;

pub(super) fn parse_discovery_row(chain: ChainId, row: &Value) -> Option<DiscoveredAsset> {
    let contract_address = first_string_deep(
        row,
        &[
            "contract_address",
            "contractAddress",
            "tokenAddress",
            "TokenAddress",
            "tokenId",
            "tokenID",
            "token_id",
            "id",
        ],
    )?;
    let symbol = first_string_deep(row, &["tokenAbbr", "symbol", "tokenSymbol", "TokenSymbol"])
        .unwrap_or_else(|| "TOKEN".to_string());
    let name = first_string_deep(row, &["tokenName", "name", "TokenName"])
        .unwrap_or_else(|| symbol.clone());
    let decimals = first_string_deep(row, &["tokenDecimal", "decimals", "TokenDivisor"])
        .and_then(|value| value.parse::<u8>().ok())
        .unwrap_or(6);
    let raw_balance = first_string(
        row,
        &[
            "balance",
            "quantity",
            "quant",
            "amount",
            "value",
            "TokenQuantity",
        ],
    )
    .unwrap_or_else(|| "0".to_string());

    Some(DiscoveredAsset {
        chain,
        kind: AssetKind::Trc20,
        contract_address,
        symbol,
        name,
        decimals,
        balance: format_units(&raw_balance, decimals),
    })
}

fn first_string_deep(row: &Value, keys: &[&str]) -> Option<String> {
    first_string(row, keys)
        .or_else(|| nested_first_string(row, "tokenInfo", keys))
        .or_else(|| nested_first_string(row, "token_info", keys))
}

fn nested_first_string(row: &Value, parent: &str, keys: &[&str]) -> Option<String> {
    row.get(parent).and_then(|value| first_string(value, keys))
}
