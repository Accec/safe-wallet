use super::super::super::WalletRepository;
use crate::error::WalletError;
use rusqlite::params;
use uuid::Uuid;

impl WalletRepository {
    pub fn delete_wallet(&self, wallet_id: Uuid) -> Result<(), WalletError> {
        if !self.wallet_exists(wallet_id)? {
            return Err(WalletError::WalletNotFound);
        }
        let mut connection = self.database.connect()?;
        let transaction = connection.transaction().map_err(|_| WalletError::Storage)?;
        let wallet_id = wallet_id.to_string();
        for statement in [
            "delete from transactions where wallet_id = ?1",
            "delete from activities where wallet_id = ?1",
            "delete from asset_balances where wallet_id = ?1",
            "delete from accounts where wallet_id = ?1",
            "delete from wallets where id = ?1",
            "delete from keystore_items where id = ?1",
        ] {
            transaction
                .execute(statement, params![wallet_id])
                .map_err(|_| WalletError::Storage)?;
        }
        transaction.commit().map_err(|_| WalletError::Storage)
    }
}
