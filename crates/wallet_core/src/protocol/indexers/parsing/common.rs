use crate::error::WalletError;
use crate::models::{ActivityKind, ActivityRecord, ActivityStatus, ChainId};
use serde_json::Value;

pub(super) fn activity_rows(body: &Value) -> Result<&Vec<Value>, WalletError> {
    if let Some(rows) = body.as_array() {
        return Ok(rows);
    }
    for key in ["result", "data", "token_transfers"] {
        if let Some(rows) = body.get(key).and_then(Value::as_array) {
            return Ok(rows);
        }
    }
    Err(WalletError::NetworkUnavailable)
}

pub(super) fn parse_summary_row(chain: ChainId, row: &Value) -> Option<ActivityRecord> {
    let summary = string_field(row, "summary")?;
    Some(ActivityRecord {
        chain,
        tx_hash: string_field(row, "tx_hash").or_else(|| string_field(row, "hash"))?,
        kind: parse_kind(string_field(row, "kind").as_deref()),
        status: parse_status(string_field(row, "status").as_deref()),
        summary,
    })
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

pub(super) fn row_status(row: &Value) -> ActivityStatus {
    if string_field(row, "isError").as_deref() == Some("1")
        || string_field(row, "contractRet")
            .as_deref()
            .is_some_and(|ret| ret != "SUCCESS")
        || string_field(row, "result")
            .as_deref()
            .is_some_and(|ret| ret != "SUCCESS")
        || row.get("revert").and_then(Value::as_bool) == Some(true)
        || row.get("confirmed").and_then(Value::as_bool) == Some(false)
    {
        ActivityStatus::Failed
    } else {
        ActivityStatus::Confirmed
    }
}

pub(super) fn format_amount(value: &str, decimals: u8) -> String {
    if value.is_empty() || value == "0" {
        return "0".to_string();
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

pub(super) fn native_symbol(chain: ChainId) -> &'static str {
    match chain {
        ChainId::Btc => "BTC",
        ChainId::Ethereum | ChainId::Arbitrum | ChainId::Optimism => "ETH",
        ChainId::Bsc => "BNB",
        ChainId::Polygon => "POL",
        ChainId::Tron => "TRX",
    }
}

fn parse_kind(value: Option<&str>) -> ActivityKind {
    match value {
        Some("token_transfer") => ActivityKind::TokenTransfer,
        Some("approval") => ActivityKind::Approval,
        Some("swap") => ActivityKind::Swap,
        Some("contract_call") => ActivityKind::ContractCall,
        _ => ActivityKind::NativeTransfer,
    }
}

fn parse_status(value: Option<&str>) -> ActivityStatus {
    match value {
        Some("pending") => ActivityStatus::Pending,
        Some("failed") => ActivityStatus::Failed,
        _ => ActivityStatus::Confirmed,
    }
}
