use crate::error::WalletError;
use crate::models::ChainId;

mod query;
mod validation;

pub use crate::models::ParsedPayment;

pub fn parse_payment_uri(payload: &str) -> Result<ParsedPayment, WalletError> {
    if payload.is_empty() {
        return Err(WalletError::InvalidAddress);
    }

    if let Some(uri) = payload.strip_prefix("bitcoin:") {
        return parse_uri(ChainId::Btc, uri, "amount");
    }

    if let Some(uri) = payload.strip_prefix("ethereum:") {
        return parse_uri(ChainId::Ethereum, uri, "value");
    }

    parse_bare_address(payload)
}

fn parse_uri(chain: ChainId, uri: &str, amount_key: &str) -> Result<ParsedPayment, WalletError> {
    let (address, query) = uri.split_once('?').unwrap_or((uri, ""));
    validation::validate_chain_address(chain, address)?;

    let reject_required_keys = chain == ChainId::Btc;
    let (amount, note) = query::parse_query(query, amount_key, reject_required_keys)?;
    Ok(ParsedPayment {
        chain: Some(chain),
        address: address.to_string(),
        amount,
        note,
    })
}

fn parse_bare_address(payload: &str) -> Result<ParsedPayment, WalletError> {
    if validation::validate_evm_address(payload).is_ok() {
        return Ok(ParsedPayment {
            chain: None,
            address: payload.to_string(),
            amount: None,
            note: None,
        });
    }

    if validation::validate_tron_address(payload).is_ok() {
        return Ok(ParsedPayment {
            chain: Some(ChainId::Tron),
            address: payload.to_string(),
            amount: None,
            note: None,
        });
    }

    if validation::validate_btc_address(payload).is_ok() {
        return Ok(ParsedPayment {
            chain: Some(ChainId::Btc),
            address: payload.to_string(),
            amount: None,
            note: None,
        });
    }

    Err(WalletError::InvalidAddress)
}

#[cfg(test)]
mod tests;
