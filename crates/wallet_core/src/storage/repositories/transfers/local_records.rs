use super::super::super::mappers::*;
use super::super::TransferRepository;
use crate::error::WalletError;
use crate::models::{ActivityKind, ActivityStatus, AssetKind, TransferPreview, TransferRequest};
use chrono::Utc;
use rusqlite::params;
use uuid::Uuid;

impl TransferRepository {
    pub fn save_local_transfer(
        &self,
        request: &TransferRequest,
        preview: &TransferPreview,
        tx_hash: &str,
        status: &str,
    ) -> Result<(), WalletError> {
        let connection = self.database.connect()?;
        let account_id = self.database.wallets().account_id_for_chain(
            &connection,
            request.wallet_id,
            request.chain,
        )?;
        let asset = self.database.assets().find_asset(request.asset_id)?;
        let activity_kind = match asset.kind {
            AssetKind::Native => ActivityKind::NativeTransfer,
            AssetKind::Erc20 | AssetKind::Trc20 => ActivityKind::TokenTransfer,
        };
        let now = Utc::now().to_rfc3339();
        let tx_id = Uuid::new_v4().to_string();
        let activity_id = Uuid::new_v4().to_string();
        connection
            .execute(
                "insert into transactions (
                    id, wallet_id, account_id, chain, asset_id, to_address, amount,
                    fee_estimate, status, tx_hash, created_at, updated_at
                ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?11)",
                params![
                    tx_id,
                    request.wallet_id.to_string(),
                    account_id,
                    chain_to_db(request.chain),
                    request.asset_id.to_string(),
                    request.to_address,
                    request.amount,
                    preview.fee_estimate,
                    status,
                    tx_hash,
                    now,
                ],
            )
            .map_err(|_| WalletError::Storage)?;
        connection
            .execute(
                "insert into activities (
                    id, wallet_id, account_id, chain, tx_hash, kind, status,
                    from_address, to_address, asset_symbol, amount, fee,
                    happened_at, decoded_summary
                ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
                params![
                    activity_id,
                    request.wallet_id.to_string(),
                    account_id,
                    chain_to_db(request.chain),
                    tx_hash,
                    activity_kind_to_db(activity_kind),
                    activity_status_to_db(ActivityStatus::Pending),
                    preview.from_address,
                    preview.to_address,
                    preview.asset_symbol,
                    preview.amount,
                    preview.fee_estimate,
                    now,
                    format!("Sent {} {}", preview.amount, preview.asset_symbol),
                ],
            )
            .map_err(|_| WalletError::Storage)?;
        Ok(())
    }
}
