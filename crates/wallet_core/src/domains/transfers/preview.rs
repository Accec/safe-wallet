use crate::domains::validation::{fee_estimate_for_chain, validate_chain_address};
use crate::error::WalletError;
use crate::models::{TransferPreview, TransferRequest};
use crate::storage::WalletDatabase;

pub(super) fn preview_transfer(
    database: &WalletDatabase,
    request: &TransferRequest,
) -> Result<TransferPreview, WalletError> {
    let amount = request.amount.trim();
    if amount.is_empty()
        || amount.starts_with('-')
        || !amount.chars().any(|ch| ch.is_ascii_digit() && ch != '0')
    {
        return Err(WalletError::InsufficientFunds);
    }
    validate_chain_address(request.chain, &request.to_address)?;
    let account = database
        .wallets()
        .account_for_chain(request.wallet_id, request.chain)?;
    let asset = database.assets().find_asset(request.asset_id)?;
    if asset.chain != request.chain {
        return Err(WalletError::InvalidAddress);
    }
    let settings = database.network().chain_settings(request.chain)?;
    Ok(TransferPreview {
        chain: request.chain,
        from_address: account.address,
        to_address: request.to_address.clone(),
        asset_symbol: asset.symbol,
        amount: request.amount.clone(),
        fee_estimate: fee_estimate_for_chain(request.chain).to_string(),
        rpc_url: settings
            .user_rpc_url
            .unwrap_or_else(|| settings.default_rpc_url),
    })
}
