use super::common::deduplicate_contracts;
use crate::models::{AssetKind, ChainId, DiscoveredAsset};
use serde_json::Value;

pub(super) fn parse_token_transfer_html(
    chain: ChainId,
    body: &str,
) -> Option<Vec<DiscoveredAsset>> {
    if !is_evm_chain(chain) || !body.contains("/token/") {
        return None;
    }
    let contracts = token_contracts(body);
    if contracts.is_empty() {
        return None;
    }

    let export_tokens = quick_export_tokens(body);
    let mut discovered = Vec::new();
    for (index, contract_address) in contracts.into_iter().enumerate() {
        let export = export_tokens.get(index);
        let title = token_title_after_contract(body, &contract_address);
        let token_text = export
            .and_then(|token| token.token.as_deref())
            .or(title.as_deref());
        let (name, symbol) = token_text
            .and_then(split_token_name_symbol)
            .unwrap_or_else(|| ("TOKEN".to_string(), "TOKEN".to_string()));
        let balance = export
            .and_then(|token| token.amount.as_deref())
            .map(normalize_display_amount)
            .unwrap_or_else(|| "0".to_string());

        discovered.push(DiscoveredAsset {
            chain,
            kind: AssetKind::Erc20,
            contract_address,
            symbol,
            name,
            decimals: 18,
            balance,
        });
    }

    Some(deduplicate_contracts(discovered))
}

fn is_evm_chain(chain: ChainId) -> bool {
    matches!(
        chain,
        ChainId::Ethereum | ChainId::Bsc | ChainId::Polygon | ChainId::Arbitrum | ChainId::Optimism
    )
}

fn token_contracts(body: &str) -> Vec<String> {
    let mut contracts = Vec::new();
    let mut remaining = body;
    while let Some(position) = remaining.find("/token/0x") {
        let start = position + "/token/".len();
        let candidate = &remaining[start..];
        let contract = candidate
            .chars()
            .take_while(|ch| ch.is_ascii_hexdigit() || *ch == 'x')
            .collect::<String>();
        if contract.len() == 42 && contract.starts_with("0x") {
            contracts.push(contract.to_lowercase());
        }
        remaining = &candidate[contract.len().min(candidate.len())..];
    }
    contracts
}

#[derive(Debug)]
struct QuickExportToken {
    token: Option<String>,
    amount: Option<String>,
}

fn quick_export_tokens(body: &str) -> Vec<QuickExportToken> {
    let Some(json_text) = extract_js_string(body, "quickExportTokentxnsData") else {
        return Vec::new();
    };
    let Ok(Value::Array(rows)) = serde_json::from_str::<Value>(&json_text) else {
        return Vec::new();
    };
    rows.into_iter()
        .filter_map(|row| {
            let token = string_field(&row, "Token");
            let amount = string_field(&row, "Amount");
            if token.is_none() && amount.is_none() {
                None
            } else {
                Some(QuickExportToken { token, amount })
            }
        })
        .collect()
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

fn string_field(row: &Value, key: &str) -> Option<String> {
    row.get(key)
        .and_then(Value::as_str)
        .map(|value| value.to_string())
}

fn token_title_after_contract(body: &str, contract: &str) -> Option<String> {
    let position = body.find(contract)?;
    let snippet = &body[position..body.len().min(position + 800)];
    extract_attr(snippet, "title")
        .or_else(|| extract_attr(snippet, "data-bs-title"))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn extract_attr(snippet: &str, attr: &str) -> Option<String> {
    let pattern = format!("{attr}=");
    let position = snippet.find(&pattern)?;
    let after_attr = &snippet[position + pattern.len()..];
    let quote = after_attr.chars().next()?;
    if quote != '\'' && quote != '"' {
        return None;
    }
    let value = after_attr[quote.len_utf8()..].split(quote).next()?;
    Some(html_unescape(value))
}

fn split_token_name_symbol(token: &str) -> Option<(String, String)> {
    let token = token
        .trim()
        .strip_prefix("BEP-20:")
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

fn normalize_display_amount(amount: &str) -> String {
    let amount = amount.trim().replace(',', "");
    if amount.is_empty() {
        "0".to_string()
    } else {
        amount
    }
}

fn html_unescape(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}
