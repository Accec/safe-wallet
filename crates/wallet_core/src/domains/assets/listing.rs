use super::AssetsDomain;
use crate::error::WalletError;
use crate::models::Asset;
use uuid::Uuid;

impl AssetsDomain {
    pub fn list_assets(&self, wallet_id: Uuid) -> Result<Vec<Asset>, WalletError> {
        self.database.assets().list_assets(wallet_id)
    }
}
