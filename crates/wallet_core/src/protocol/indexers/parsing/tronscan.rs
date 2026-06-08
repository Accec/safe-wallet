use super::common::{format_amount, native_symbol, row_status, string_field};
use crate::models::{ActivityKind, ActivityRecord, ChainId};
use serde_json::Value;

pub(super) fn parse_activity_row(chain: ChainId, row: &Value) -> Option<ActivityRecord> {
    let tx_hash = string_field(row, "transactionHash")
        .or_else(|| string_field(row, "transaction_id"))
        .or_else(|| string_field(row, "hash"))?;
    let token_symbol = token_info_field(row, "tokenAbbr");
    let kind = if is_native_symbol(chain, token_symbol.as_deref()) {
        ActivityKind::NativeTransfer
    } else if token_symbol.is_some() {
        ActivityKind::TokenTransfer
    } else {
        ActivityKind::NativeTransfer
    };
    let raw_amount = string_field(row, "amount")
        .or_else(|| string_field(row, "quant"))
        .or_else(|| string_field(row, "value"))
        .or_else(|| {
            row.get("contractData")
                .and_then(|contract_data| string_field(contract_data, "amount"))
        });
    if raw_amount.as_deref().map_or(true, is_zero_amount) {
        return None;
    }
    let amount = raw_amount
        .map(|value| format_amount(&value, token_decimals(row, kind)))
        .unwrap_or_else(|| "0".to_string());
    let symbol = if kind == ActivityKind::NativeTransfer {
        native_symbol(chain).to_string()
    } else {
        token_symbol.unwrap_or_else(|| native_symbol(chain).to_string())
    };
    let to = string_field(row, "transferToAddress")
        .or_else(|| string_field(row, "to_address"))
        .or_else(|| string_field(row, "toAddress"))
        .or_else(|| string_field(row, "to"))
        .or_else(|| {
            row.get("contractData")
                .and_then(|contract_data| string_field(contract_data, "to_address"))
        })
        .unwrap_or_else(|| "unknown".to_string());

    Some(ActivityRecord {
        chain,
        tx_hash,
        kind,
        status: row_status(row),
        summary: format!("Transfer {amount} {symbol} to {to}"),
    })
}

fn is_zero_amount(value: &str) -> bool {
    value.trim().trim_start_matches('0').is_empty()
}

fn is_native_symbol(chain: ChainId, symbol: Option<&str>) -> bool {
    chain == ChainId::Tron && symbol.is_some_and(|symbol| symbol.eq_ignore_ascii_case("TRX"))
}

fn token_info_field(row: &Value, key: &str) -> Option<String> {
    row.get("tokenInfo")
        .and_then(|token_info| string_field(token_info, key))
        .or_else(|| {
            row.get("token_info")
                .and_then(|token_info| string_field(token_info, key))
        })
}

fn token_decimals(row: &Value, kind: ActivityKind) -> u8 {
    string_field(row, "tokenDecimal")
        .or_else(|| token_info_field(row, "tokenDecimal"))
        .and_then(|value| value.parse::<u8>().ok())
        .unwrap_or(if kind == ActivityKind::TokenTransfer {
            18
        } else {
            6
        })
}
