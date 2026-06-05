mod balances;
mod fees;
mod rpc;
mod signing;

use crate::error::WalletError;
use crate::models::AssetKind;
use alloy_consensus::{TxEip1559, TxLegacy};
use alloy_primitives::{Address as AlloyAddress, Bytes as AlloyBytes, TxKind, U256};
use reqwest::blocking::Client;
use serde_json::json;

use super::amount::decimal_amount_to_be_bytes;
use super::encoding::{
    encode_erc20_transfer, evm_hex_quantity, expected_evm_chain_id, normalize_evm_address,
};
use super::{BroadcastedTransaction, TransferBroadcastDraft};
use balances::ensure_sendable_balance;
use fees::fee_settings;
use rpc::{eth_string, eth_u128};
use signing::{sign_eip1559_transaction, sign_legacy_transaction};

pub(super) fn broadcast_transfer(
    client: &Client,
    draft: &TransferBroadcastDraft<'_>,
) -> Result<BroadcastedTransaction, WalletError> {
    let from_address = normalize_evm_address(draft.from_address)?;
    let from_address_hex = hex::encode(&from_address);
    let (to, value, data) = match draft.asset.kind {
        AssetKind::Native => {
            let to = normalize_evm_address(draft.to_address)?;
            let value = decimal_amount_to_be_bytes(draft.amount, draft.asset.decimals)?;
            (to, value, Vec::new())
        }
        AssetKind::Erc20 => {
            let contract = draft
                .asset
                .contract_address
                .as_deref()
                .ok_or(WalletError::InvalidTokenContract)?;
            let to = normalize_evm_address(contract)?;
            let data = encode_erc20_transfer(draft.to_address, draft.amount, draft.asset.decimals)?;
            (to, Vec::new(), data)
        }
        AssetKind::Trc20 => return Err(WalletError::InvalidTokenContract),
    };

    let chain_id = eth_u128(client, draft.rpc_url, "eth_chainId", json!([]))?;
    let expected_chain_id = expected_evm_chain_id(draft.chain)?;
    if chain_id != expected_chain_id as u128 {
        return Err(WalletError::InvalidNetworkSettings);
    }
    let nonce = eth_u128(
        client,
        draft.rpc_url,
        "eth_getTransactionCount",
        json!([format!("0x{from_address_hex}"), "pending"]),
    )?;
    let mut estimate = json!({
        "from": format!("0x{from_address_hex}"),
        "to": format!("0x{}", hex::encode(&to)),
        "value": evm_hex_quantity(&value),
    });
    if !data.is_empty() {
        estimate["data"] = json!(format!("0x{}", hex::encode(&data)));
    }
    let gas_limit = eth_u128(client, draft.rpc_url, "eth_estimateGas", json!([estimate]))?;
    let fee = fee_settings(client, draft.chain, draft.rpc_url)?;
    ensure_sendable_balance(
        client,
        draft.rpc_url,
        &from_address,
        &from_address_hex,
        &to,
        &value,
        draft,
        gas_limit,
        fee.max_native_fee_per_gas(),
    )?;

    let gas_limit = u64::try_from(gas_limit).map_err(|_| WalletError::NetworkUnavailable)?;
    let nonce = u64::try_from(nonce).map_err(|_| WalletError::NetworkUnavailable)?;
    let value = U256::from_be_slice(&value);
    let to = TxKind::Call(AlloyAddress::from_slice(&to));
    let input = AlloyBytes::from(data);

    let raw_tx = match fee {
        fees::EvmFeeSettings::Eip1559 {
            max_fee_per_gas,
            max_priority_fee_per_gas,
        } => {
            let transaction = TxEip1559 {
                chain_id: expected_chain_id,
                nonce,
                gas_limit,
                max_fee_per_gas,
                max_priority_fee_per_gas,
                to,
                value,
                access_list: Default::default(),
                input,
            };
            sign_eip1559_transaction(transaction, draft.signing_key)?
        }
        fees::EvmFeeSettings::Legacy { gas_price } => {
            let transaction = TxLegacy {
                chain_id: Some(expected_chain_id),
                nonce,
                gas_price,
                gas_limit,
                to,
                value,
                input,
            };
            sign_legacy_transaction(transaction, draft.signing_key)?
        }
    };
    let tx_hash = eth_string(
        client,
        draft.rpc_url,
        "eth_sendRawTransaction",
        json!([format!("0x{}", hex::encode(raw_tx))]),
    )?;
    Ok(BroadcastedTransaction { tx_hash })
}
