use crate::error::WalletError;
use crate::models::AssetKind;
use alloy_primitives::U256;
use reqwest::blocking::Client;
use serde_json::json;

use super::super::amount::decimal_amount_to_be_bytes;
use super::super::encoding::{hex_string_to_u256, left_pad_32};
use super::super::TransferBroadcastDraft;
use super::rpc::{eth_string, eth_u128};

#[allow(clippy::too_many_arguments)]
pub(super) fn ensure_sendable_balance(
    client: &Client,
    rpc_url: &str,
    from_address: &[u8],
    from_address_hex: &str,
    transfer_to: &[u8],
    native_value: &[u8],
    draft: &TransferBroadcastDraft<'_>,
    gas_limit: u128,
    max_fee_per_gas: u128,
) -> Result<(), WalletError> {
    let balance = eth_u128(
        client,
        rpc_url,
        "eth_getBalance",
        json!([format!("0x{from_address_hex}"), "pending"]),
    )?;
    let gas_cost = U256::from(gas_limit)
        .checked_mul(U256::from(max_fee_per_gas))
        .ok_or(WalletError::InsufficientFunds)?;
    let required_native = match draft.asset.kind {
        AssetKind::Native => gas_cost
            .checked_add(U256::from_be_slice(native_value))
            .ok_or(WalletError::InsufficientFunds)?,
        AssetKind::Erc20 => gas_cost,
        AssetKind::Trc20 => return Err(WalletError::InvalidTokenContract),
    };
    if U256::from(balance) < required_native {
        return Err(WalletError::InsufficientFunds);
    }
    if draft.asset.kind == AssetKind::Erc20 {
        let token_balance = token_balance(client, rpc_url, transfer_to, from_address)?;
        let token_amount = U256::from_be_slice(&decimal_amount_to_be_bytes(
            draft.amount,
            draft.asset.decimals,
        )?);
        if token_balance < token_amount {
            return Err(WalletError::InsufficientFunds);
        }
    }
    Ok(())
}

fn token_balance(
    client: &Client,
    rpc_url: &str,
    contract_address: &[u8],
    owner_address: &[u8],
) -> Result<U256, WalletError> {
    let mut data = hex::decode("70a08231").map_err(|_| WalletError::Crypto)?;
    data.extend_from_slice(&left_pad_32(owner_address));
    let result = eth_string(
        client,
        rpc_url,
        "eth_call",
        json!([
            {
                "to": format!("0x{}", hex::encode(contract_address)),
                "data": format!("0x{}", hex::encode(data))
            },
            "pending"
        ]),
    )?;
    hex_string_to_u256(&result)
}
