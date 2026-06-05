use super::common::{format_amount, native_symbol, row_status, string_field};
use crate::models::{ActivityKind, ActivityRecord, ChainId};
use serde_json::Value;

pub(super) fn parse_activity_row(chain: ChainId, row: &Value) -> Option<ActivityRecord> {
    let tx_hash = string_field(row, "hash").or_else(|| string_field(row, "transactionHash"))?;
    let token_symbol = string_field(row, "tokenSymbol");
    let kind = if token_symbol.is_some() {
        ActivityKind::TokenTransfer
    } else {
        ActivityKind::NativeTransfer
    };
    let amount = string_field(row, "value")
        .map(|value| format_amount(&value, token_decimals(row, kind)))
        .unwrap_or_else(|| "0".to_string());
    let symbol = token_symbol.unwrap_or_else(|| native_symbol(chain).to_string());
    let to = string_field(row, "to").unwrap_or_else(|| "unknown".to_string());

    Some(ActivityRecord {
        chain,
        tx_hash,
        kind,
        status: row_status(row),
        summary: format!("Transfer {amount} {symbol} to {to}"),
    })
}

fn token_decimals(row: &Value, kind: ActivityKind) -> u8 {
    string_field(row, "tokenDecimal")
        .and_then(|value| value.parse::<u8>().ok())
        .unwrap_or(if kind == ActivityKind::TokenTransfer {
            18
        } else {
            6
        })
}
