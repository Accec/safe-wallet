use crate::error::WalletError;
use crate::models::{TransferRequest, TransferResult};
use crate::protocol::transactions::{TransactionBroadcastClient, TransferBroadcastDraft};
use crate::storage::WalletDatabase;

use super::preview;
use super::signing;

pub(super) fn send_transfer_with<C: TransactionBroadcastClient>(
    database: &WalletDatabase,
    request: &TransferRequest,
    password: &str,
    client: &C,
) -> Result<TransferResult, WalletError> {
    signing::verify_wallet_password(database, request.wallet_id, password)?;
    let preview = preview::preview_transfer(database, request)?;
    let account = database
        .wallets()
        .account_for_chain(request.wallet_id, request.chain)?;
    let asset = database.assets().find_asset(request.asset_id)?;
    let signing_key = signing::signing_key_for_account(database, &account, password)?;
    let broadcasted = client.broadcast_transfer(&TransferBroadcastDraft {
        chain: request.chain,
        rpc_url: &preview.rpc_url,
        from_address: &preview.from_address,
        to_address: &preview.to_address,
        amount: &preview.amount,
        asset: &asset,
        signing_key: &signing_key,
    })?;
    let status = "broadcasted";
    database
        .transfers()
        .save_local_transfer(request, &preview, &broadcasted.tx_hash, status)?;
    Ok(TransferResult {
        chain: request.chain,
        tx_hash: broadcasted.tx_hash,
        status: status.to_string(),
    })
}
