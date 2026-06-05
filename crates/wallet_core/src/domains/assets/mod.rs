mod balances;
mod discovery;
mod listing;
mod tokens;

use crate::storage::WalletDatabase;

pub struct AssetsDomain {
    pub(super) database: WalletDatabase,
}

impl AssetsDomain {
    pub(crate) fn new(database: WalletDatabase) -> Self {
        Self { database }
    }
}
