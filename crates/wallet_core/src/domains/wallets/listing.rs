use uuid::Uuid;

use crate::error::WalletError;
use crate::models::{Account, WalletSummary};
use crate::storage::WalletDatabase;

pub(super) fn list_wallets(database: &WalletDatabase) -> Result<Vec<WalletSummary>, WalletError> {
    database.wallets().list_wallets()
}

pub(super) fn list_accounts(
    database: &WalletDatabase,
    wallet_id: Uuid,
) -> Result<Vec<Account>, WalletError> {
    database.wallets().list_accounts(wallet_id)
}
