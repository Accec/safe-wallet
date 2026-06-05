use crate::error::WalletError;
use crate::models::ChainId;
use reqwest::blocking::Client;
use serde_json::json;

use super::super::encoding::{evm_prefers_eip1559, hex_quantity_to_u128};
use super::rpc::{eth_u128, eth_value};

pub(super) fn fee_settings(
    client: &Client,
    chain: ChainId,
    rpc_url: &str,
) -> Result<EvmFeeSettings, WalletError> {
    if evm_prefers_eip1559(chain) {
        if let Ok(fee) = eip1559_fee_settings(client, rpc_url) {
            return Ok(fee);
        }
    }
    let gas_price = eth_u128(client, rpc_url, "eth_gasPrice", json!([]))?;
    Ok(EvmFeeSettings::Legacy { gas_price })
}

fn eip1559_fee_settings(client: &Client, rpc_url: &str) -> Result<EvmFeeSettings, WalletError> {
    let max_priority_fee_per_gas =
        eth_u128(client, rpc_url, "eth_maxPriorityFeePerGas", json!([]))?;
    let latest_block = eth_value(
        client,
        rpc_url,
        "eth_getBlockByNumber",
        json!(["latest", false]),
    )?;
    let base_fee_per_gas = latest_block
        .get("baseFeePerGas")
        .and_then(|value| value.as_str())
        .ok_or(WalletError::NetworkUnavailable)
        .and_then(hex_quantity_to_u128)?;
    let max_fee_per_gas = base_fee_per_gas
        .checked_mul(2)
        .and_then(|value| value.checked_add(max_priority_fee_per_gas))
        .ok_or(WalletError::NetworkUnavailable)?;
    Ok(EvmFeeSettings::Eip1559 {
        max_fee_per_gas,
        max_priority_fee_per_gas,
    })
}

#[derive(Debug, Clone, Copy)]
pub(super) enum EvmFeeSettings {
    Legacy {
        gas_price: u128,
    },
    Eip1559 {
        max_fee_per_gas: u128,
        max_priority_fee_per_gas: u128,
    },
}

impl EvmFeeSettings {
    pub(super) fn max_native_fee_per_gas(self) -> u128 {
        match self {
            EvmFeeSettings::Legacy { gas_price } => gas_price,
            EvmFeeSettings::Eip1559 {
                max_fee_per_gas, ..
            } => max_fee_per_gas,
        }
    }
}
