use crate::error::WalletError;
use crate::models::{Account, KeystoreExport, WalletSummary};
use crate::storage::WalletDatabase;
use uuid::Uuid;

pub(crate) mod accounts;
mod creation;
mod imports;
pub(crate) mod keystore;
mod listing;
pub(crate) mod mnemonic;
mod secrets;

pub fn generate_mnemonic() -> Result<String, WalletError> {
    mnemonic::generate_mnemonic()
}

pub struct WalletsDomain {
    database: WalletDatabase,
}

impl WalletsDomain {
    pub(crate) fn new(database: WalletDatabase) -> Self {
        Self { database }
    }

    pub fn create_wallet(
        &self,
        label: &str,
        mnemonic: &str,
        password: &str,
    ) -> Result<WalletSummary, WalletError> {
        creation::create_wallet(&self.database, label, mnemonic, password)
    }

    pub fn import_private_key(
        &self,
        label: &str,
        private_key: &str,
        password: &str,
    ) -> Result<WalletSummary, WalletError> {
        imports::import_private_key(&self.database, label, private_key, password)
    }

    pub fn import_keystore(
        &self,
        label: &str,
        keystore_json: &str,
        keystore_password: &str,
        password: &str,
    ) -> Result<WalletSummary, WalletError> {
        imports::import_keystore(
            &self.database,
            label,
            keystore_json,
            keystore_password,
            password,
        )
    }

    pub fn list_wallets(&self) -> Result<Vec<WalletSummary>, WalletError> {
        listing::list_wallets(&self.database)
    }

    pub fn list_accounts(&self, wallet_id: Uuid) -> Result<Vec<Account>, WalletError> {
        listing::list_accounts(&self.database, wallet_id)
    }

    pub fn export_keystore(
        &self,
        wallet_id: Uuid,
        password: &str,
    ) -> Result<KeystoreExport, WalletError> {
        secrets::export_keystore(&self.database, wallet_id, password)
    }

    pub fn delete_wallet(&self, wallet_id: Uuid, password: &str) -> Result<(), WalletError> {
        secrets::delete_wallet(&self.database, wallet_id, password)
    }

    pub fn reveal_mnemonic(&self, wallet_id: Uuid, password: &str) -> Result<String, WalletError> {
        secrets::reveal_mnemonic(&self.database, wallet_id, password)
    }

    pub(crate) fn verify_wallet_password(
        &self,
        wallet_id: Uuid,
        password: &str,
    ) -> Result<(), WalletError> {
        secrets::verify_wallet_password(&self.database, wallet_id, password)
    }
}
