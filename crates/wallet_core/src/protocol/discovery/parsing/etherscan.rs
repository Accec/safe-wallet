use super::common::{first_string, format_units};
use crate::models::{AssetKind, ChainId, DiscoveredAsset};
use serde_json::Value;

pub(super) fn parse_discovery_row(chain: ChainId, row: &Value) -> Option<DiscoveredAsset> {
    if chain == ChainId::Btc {
        return None;
    }
    let contract_address = first_string(
        row,
        &[
            "TokenAddress",
            "contractAddress",
            "contract_address",
            "tokenAddress",
            "id",
        ],
    )?;
    let symbol = first_string(row, &["TokenSymbol", "tokenSymbol", "symbol", "tokenAbbr"])
        .unwrap_or_else(|| "TOKEN".to_string());
    let name =
        first_string(row, &["TokenName", "tokenName", "name"]).unwrap_or_else(|| symbol.clone());
    let decimals = first_string(row, &["TokenDivisor", "tokenDecimal", "decimals"])
        .and_then(|value| value.parse::<u8>().ok())
        .unwrap_or(18);
    let raw_balance = first_string(
        row,
        &["TokenQuantity", "balance", "quantity", "amount", "value"],
    )
    .unwrap_or_else(|| "0".to_string());

    Some(DiscoveredAsset {
        chain,
        kind: AssetKind::Erc20,
        contract_address,
        symbol,
        name,
        decimals,
        balance: format_units(&raw_balance, decimals),
    })
}
