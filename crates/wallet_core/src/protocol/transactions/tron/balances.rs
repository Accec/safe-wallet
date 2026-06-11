use crate::error::WalletError;
use alloy_primitives::U256;
use reqwest::blocking::Client;
use serde_json::json;

use super::super::amount::decimal_amount_to_be_bytes;
use super::super::encoding::{encode_tron_balance_of_parameter, hex_string_to_u256};
use super::rpc;

pub(super) const MIN_TRC20_FEE_RESERVE_SUN: u64 = 1_000_000;

pub(super) fn ensure_native_balance(
    client: &Client,
    rpc_url: &str,
    owner_address: &str,
    required_sun: u64,
) -> Result<(), WalletError> {
    let account = rpc::post(
        client,
        rpc_url,
        "/wallet/getaccount",
        json!({
            "address": owner_address,
            "visible": false,
        }),
    )?;
    let balance = account
        .get("balance")
        .and_then(|value| value.as_u64())
        .unwrap_or(0);
    if balance < required_sun {
        return Err(WalletError::InsufficientFunds);
    }
    Ok(())
}

pub(super) fn ensure_token_balance(
    client: &Client,
    rpc_url: &str,
    owner_address: &str,
    contract_address: &str,
    amount: &str,
    decimals: u8,
) -> Result<(), WalletError> {
    let balance = token_balance(client, rpc_url, owner_address, contract_address)?;
    let required = U256::from_be_slice(&decimal_amount_to_be_bytes(amount, decimals)?);
    if balance < required {
        return Err(WalletError::InsufficientFunds);
    }
    Ok(())
}

fn token_balance(
    client: &Client,
    rpc_url: &str,
    owner_address: &str,
    contract_address: &str,
) -> Result<U256, WalletError> {
    let response = rpc::post(
        client,
        rpc_url,
        "/wallet/triggerconstantcontract",
        json!({
            "owner_address": owner_address,
            "contract_address": contract_address,
            "function_selector": "balanceOf(address)",
            "parameter": encode_tron_balance_of_parameter(owner_address)?,
            "visible": false,
        }),
    )?;
    let balance = response
        .get("constant_result")
        .and_then(|value| value.as_array())
        .and_then(|values| values.first())
        .and_then(|value| value.as_str())
        .ok_or(WalletError::NetworkUnavailable)?;
    let balance = if balance.starts_with("0x") {
        balance.to_string()
    } else {
        format!("0x{balance}")
    };
    hex_string_to_u256(&balance)
}
