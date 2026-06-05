use super::common::{first_string, format_units};
use crate::models::{AssetKind, ChainId, DiscoveredAsset};
use serde_json::Value;

pub(super) fn parse_discovery_row(chain: ChainId, row: &Value) -> Option<DiscoveredAsset> {
    let contract_address = first_string(
        row,
        &[
            "tokenId",
            "tokenID",
            "token_id",
            "id",
            "contractAddress",
            "contract_address",
            "tokenAddress",
            "TokenAddress",
        ],
    )?;
    let symbol = first_string(row, &["tokenAbbr", "symbol", "tokenSymbol", "TokenSymbol"])
        .unwrap_or_else(|| "TOKEN".to_string());
    let name =
        first_string(row, &["tokenName", "name", "TokenName"]).unwrap_or_else(|| symbol.clone());
    let decimals = first_string(row, &["tokenDecimal", "decimals", "TokenDivisor"])
        .and_then(|value| value.parse::<u8>().ok())
        .unwrap_or(6);
    let raw_balance = first_string(
        row,
        &["balance", "quantity", "amount", "value", "TokenQuantity"],
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
