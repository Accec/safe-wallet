mod address;
mod builder;
mod rpc;
mod signing;
mod utxos;

use crate::error::WalletError;
use crate::models::AssetKind;
use reqwest::blocking::Client;

use super::amount::decimal_amount_to_u64;
use super::{BroadcastedTransaction, TransferBroadcastDraft};
use address::address;
use builder::build_signed_transaction;
use rpc::{broadcast, fee_rate_sat_vb};
use utxos::address_utxos;

pub(super) fn broadcast_transfer(
    client: &Client,
    draft: &TransferBroadcastDraft<'_>,
) -> Result<BroadcastedTransaction, WalletError> {
    if draft.asset.kind != AssetKind::Native {
        return Err(WalletError::InvalidTokenContract);
    }
    let from_address = address(draft.from_address)?;
    let to_address = address(draft.to_address)?;
    let amount_sats = decimal_amount_to_u64(draft.amount, draft.asset.decimals)?;
    let fee_rate = fee_rate_sat_vb(client, draft.rpc_url)?;
    let utxos = address_utxos(client, draft.rpc_url, draft.from_address)?;
    let transaction = build_signed_transaction(
        &from_address,
        &to_address,
        amount_sats,
        fee_rate,
        &utxos,
        draft.signing_key,
    )?;
    let txid = transaction.compute_txid().to_string();
    broadcast(client, draft.rpc_url, &transaction)?;
    Ok(BroadcastedTransaction { tx_hash: txid })
}
