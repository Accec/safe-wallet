use super::super::AppSecurityRepository;
use crate::error::WalletError;

impl AppSecurityRepository {
    pub fn wipe_all_wallet_data(&self) -> Result<(), WalletError> {
        let mut connection = self.database.connect()?;
        let transaction = connection.transaction().map_err(|_| WalletError::Storage)?;
        for statement in [
            "delete from transactions",
            "delete from activities",
            "delete from asset_balances",
            "delete from accounts",
            "delete from wallets",
            "delete from keystore_items",
            "delete from tokens",
            "delete from chain_settings",
            "delete from indexer_settings",
            "delete from ui_preferences",
            "delete from app_security",
        ] {
            transaction
                .execute(statement, [])
                .map_err(|_| WalletError::Storage)?;
        }
        transaction.commit().map_err(|_| WalletError::Storage)
    }
}
