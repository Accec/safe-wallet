use super::super::AssetRepository;
use crate::error::WalletError;
use crate::models::ChainId;
use chrono::Utc;
use rusqlite::params;
use uuid::Uuid;

impl AssetRepository {
    pub fn save_asset_balance(
        &self,
        wallet_id: Uuid,
        chain: ChainId,
        asset_id: Uuid,
        balance: &str,
        source: &str,
    ) -> Result<(), WalletError> {
        if !self.database.wallets().wallet_exists(wallet_id)? {
            return Err(WalletError::WalletNotFound);
        }
        let connection = self.database.connect()?;
        let account_id =
            self.database
                .wallets()
                .account_id_for_chain(&connection, wallet_id, chain)?;
        let now = Utc::now().to_rfc3339();
        connection
            .execute(
                "insert into asset_balances (
                    id, wallet_id, account_id, asset_id, balance, source, refreshed_at
                ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                on conflict(wallet_id, account_id, asset_id)
                do update set
                    balance = excluded.balance,
                    source = excluded.source,
                    refreshed_at = excluded.refreshed_at",
                params![
                    Uuid::new_v4().to_string(),
                    wallet_id.to_string(),
                    account_id,
                    asset_id.to_string(),
                    balance,
                    source,
                    now,
                ],
            )
            .map_err(|_| WalletError::Storage)?;
        Ok(())
    }
}
