use crate::models::{ActivityKind, ActivityRecord, ActivityStatus, ChainId};
use serde_json::Value;

pub(super) fn parse_token_transfer_html(chain: ChainId, body: &str) -> Option<Vec<ActivityRecord>> {
    if !is_evm_chain(chain) {
        return None;
    }
    let mut records = Vec::new();
    records.extend(parse_quick_export_rows(
        body,
        "quickExportTransactionListData",
        |row| parse_transaction_row(chain, row),
    ));
    records.extend(parse_quick_export_rows(
        body,
        "quickExportTokentxnsData",
        |row| parse_token_transfer_row(chain, row),
    ));
    if records.is_empty() {
        None
    } else {
        Some(records)
    }
}

fn parse_quick_export_rows(
    body: &str,
    variable: &str,
    parse_row: impl Fn(&Value) -> Option<ActivityRecord>,
) -> Vec<ActivityRecord> {
    let Some(json_text) = extract_js_string(body, variable) else {
        return Vec::new();
    };
    let Ok(Value::Array(rows)) = serde_json::from_str::<Value>(&json_text) else {
        return Vec::new();
    };
    rows.iter().filter_map(parse_row).collect()
}

fn parse_token_transfer_row(chain: ChainId, row: &Value) -> Option<ActivityRecord> {
    let tx_hash = first_string(row, &["Txhash", "Transaction Hash", "Hash"])?;
    let amount = first_string(row, &["Amount"]).map(|amount| normalize_amount(&amount))?;
    let token = first_string(row, &["Token"]).unwrap_or_else(|| "TOKEN".to_string());
    let symbol = split_token_name_symbol(&token)
        .map(|(_, symbol)| symbol)
        .unwrap_or(token);
    let receiver = first_string(row, &["Receiver", "To"]).unwrap_or_else(|| "unknown".to_string());

    Some(ActivityRecord {
        chain,
        tx_hash,
        kind: ActivityKind::TokenTransfer,
        status: parse_status(first_string(row, &["Status"]).as_deref()),
        summary: format!("Transfer {amount} {symbol} to {receiver}"),
    })
}

fn parse_transaction_row(chain: ChainId, row: &Value) -> Option<ActivityRecord> {
    let tx_hash = first_string(row, &["Txhash", "Transaction Hash", "Hash"])?;
    let method = first_string(row, &["Method"]).unwrap_or_else(|| "Transaction".to_string());
    let receiver = first_string(row, &["Receiver", "To"]).unwrap_or_else(|| "unknown".to_string());
    let status = parse_status(first_string(row, &["Status"]).as_deref());
    let (amount, symbol) = first_string(row, &["Amount"])
        .and_then(|amount| split_display_amount(&amount))
        .unwrap_or_else(|| ("0".to_string(), native_symbol(chain).to_string()));
    let kind = if amount == "0" {
        if method.to_ascii_lowercase().contains("approve") {
            ActivityKind::Approval
        } else {
            ActivityKind::ContractCall
        }
    } else {
        ActivityKind::NativeTransfer
    };
    let summary = if amount == "0" {
        format!("{method} to {receiver}")
    } else {
        format!("Transfer {amount} {symbol} to {receiver}")
    };

    Some(ActivityRecord {
        chain,
        tx_hash,
        kind,
        status,
        summary,
    })
}

fn is_evm_chain(chain: ChainId) -> bool {
    matches!(
        chain,
        ChainId::Ethereum | ChainId::Bsc | ChainId::Polygon | ChainId::Arbitrum | ChainId::Optimism
    )
}

fn native_symbol(chain: ChainId) -> &'static str {
    match chain {
        ChainId::Ethereum | ChainId::Arbitrum | ChainId::Optimism => "ETH",
        ChainId::Bsc => "BNB",
        ChainId::Polygon => "POL",
        ChainId::Btc => "BTC",
        ChainId::Tron => "TRX",
    }
}

fn first_string(row: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| row.get(key).and_then(Value::as_str))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn parse_status(value: Option<&str>) -> ActivityStatus {
    match value.map(|value| value.to_ascii_lowercase()) {
        Some(value) if value.contains("fail") || value.contains("error") => ActivityStatus::Failed,
        _ => ActivityStatus::Confirmed,
    }
}

fn extract_js_string(body: &str, variable: &str) -> Option<String> {
    let start = body.find(variable)?;
    let after_variable = &body[start + variable.len()..];
    let assignment = after_variable.find('=')?;
    let after_assignment = after_variable[assignment + 1..].trim_start();
    let quote = after_assignment.chars().next()?;
    if quote != '\'' && quote != '"' {
        return None;
    }
    let mut value = String::new();
    let mut escaped = false;
    for ch in after_assignment[quote.len_utf8()..].chars() {
        if escaped {
            value.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == quote {
            return Some(value);
        }
        value.push(ch);
    }
    None
}

fn split_token_name_symbol(token: &str) -> Option<(String, String)> {
    let token = token
        .trim()
        .strip_prefix("BEP-20:")
        .or_else(|| token.trim().strip_prefix("ERC-20:"))
        .unwrap_or(token.trim())
        .trim();
    let open = token.rfind('(')?;
    let close = token.rfind(')')?;
    if close <= open {
        return None;
    }
    let name = token[..open].trim();
    let symbol = token[open + 1..close].trim();
    if name.is_empty() || symbol.is_empty() {
        None
    } else {
        Some((name.to_string(), symbol.to_string()))
    }
}

fn normalize_amount(amount: &str) -> String {
    let amount = amount.trim().replace(',', "");
    if amount.is_empty() {
        "0".to_string()
    } else {
        amount
    }
}

fn split_display_amount(amount: &str) -> Option<(String, String)> {
    let amount = amount.trim();
    let split = amount.rfind(' ')?;
    let value = normalize_amount(&amount[..split]);
    let symbol = amount[split + 1..].trim();
    if value.is_empty() || symbol.is_empty() {
        None
    } else {
        Some((value, symbol.to_string()))
    }
}
