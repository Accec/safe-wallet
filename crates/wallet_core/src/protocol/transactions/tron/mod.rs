use crate::error::WalletError;
use crate::models::AssetKind;
use reqwest::blocking::Client;

use super::amount::decimal_amount_to_u64;
use super::encoding::tron_base58_to_hex;
use super::{BroadcastedTransaction, TransferBroadcastDraft};

mod balances;
mod builder;
mod rpc;
mod signing;

pub(super) fn broadcast_transfer(
    client: &Client,
    draft: &TransferBroadcastDraft<'_>,
) -> Result<BroadcastedTransaction, WalletError> {
    let owner_address = tron_base58_to_hex(draft.from_address)?;
    let unsigned = match draft.asset.kind {
        AssetKind::Native => {
            let amount = decimal_amount_to_u64(draft.amount, draft.asset.decimals)?;
            balances::ensure_native_balance(client, draft.rpc_url, &owner_address, amount)?;
            builder::create_native_transfer(
                client,
                draft.rpc_url,
                &owner_address,
                draft.to_address,
                amount,
            )?
        }
        AssetKind::Trc20 => {
            let contract = draft
                .asset
                .contract_address
                .as_deref()
                .ok_or(WalletError::InvalidTokenContract)?;
            let contract_address = tron_base58_to_hex(contract)?;
            balances::ensure_native_balance(client, draft.rpc_url, &owner_address, 1)?;
            balances::ensure_token_balance(
                client,
                draft.rpc_url,
                &owner_address,
                &contract_address,
                draft.amount,
                draft.asset.decimals,
            )?;
            builder::create_trc20_transfer(
                client,
                draft.rpc_url,
                &owner_address,
                &contract_address,
                draft.to_address,
                draft.amount,
                draft.asset.decimals,
            )?
        }
        AssetKind::Erc20 => return Err(WalletError::InvalidTokenContract),
    };
    let signed = signing::sign_transaction(unsigned, draft.signing_key)?;
    let tx_hash = signed
        .get("txID")
        .and_then(|value| value.as_str())
        .ok_or(WalletError::NetworkUnavailable)?
        .to_string();
    let broadcast = rpc::post(
        client,
        draft.rpc_url,
        "/wallet/broadcasttransaction",
        signed,
    )?;
    if broadcast
        .get("result")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
    {
        Ok(BroadcastedTransaction { tx_hash })
    } else {
        Err(WalletError::NetworkUnavailable)
    }
}

#[cfg(test)]
mod tests;
