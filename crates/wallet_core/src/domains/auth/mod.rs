use crate::chains;
use crate::domains::wallets::{accounts, keystore, mnemonic};
use crate::error::WalletError;
use crate::models::WalletSummary;
use crate::storage::WalletDatabase;
use chrono::Utc;
use uuid::Uuid;

pub(crate) mod security;

pub struct AuthDomain {
    database: WalletDatabase,
}

impl AuthDomain {
    pub(crate) fn new(database: WalletDatabase) -> Self {
        Self { database }
    }

    pub fn set_master_password(&self, password: &str) -> Result<(), WalletError> {
        let verifier = security::create_master_password_verifier(password)?;
        self.database
            .app_security()
            .save_master_password_verifier(&verifier)
    }

    pub fn unlock_app(&self, password: &str) -> Result<(), WalletError> {
        let verifier = self
            .database
            .app_security()
            .load_master_password_verifier()?
            .ok_or(WalletError::InvalidPassword)?;
        match security::verify_master_password(password, &verifier) {
            Ok(()) => Ok(()),
            Err(_) if self.duress_password_matches(password)? => {
                self.replace_wallet_data_with_decoy(password)
            }
            Err(error) => Err(error),
        }
    }

    pub fn set_duress_password(
        &self,
        master_password: &str,
        duress_password: &str,
    ) -> Result<(), WalletError> {
        if duress_password.is_empty() {
            return Err(WalletError::InvalidPassword);
        }
        let verifier = self
            .database
            .app_security()
            .load_master_password_verifier()?
            .ok_or(WalletError::InvalidPassword)?;
        security::verify_master_password(master_password, &verifier)?;
        if security::verify_master_password(duress_password, &verifier).is_ok() {
            return Err(WalletError::InvalidPassword);
        }
        let duress_verifier = security::create_duress_password_verifier(duress_password)?;
        self.database
            .app_security()
            .save_duress_password_verifier(&duress_verifier)
    }

    pub fn set_biometric_unlock(
        &self,
        master_password: &str,
        enabled: bool,
    ) -> Result<(), WalletError> {
        let verifier = self
            .database
            .app_security()
            .load_master_password_verifier()?
            .ok_or(WalletError::InvalidPassword)?;
        security::verify_master_password(master_password, &verifier)?;
        self.database
            .app_security()
            .update_biometric_enabled(enabled)
    }

    fn duress_password_matches(&self, password: &str) -> Result<bool, WalletError> {
        let Some(verifier) = self
            .database
            .app_security()
            .load_duress_password_verifier()?
        else {
            return Ok(false);
        };
        Ok(security::verify_duress_password(password, &verifier).is_ok())
    }

    fn replace_wallet_data_with_decoy(&self, password: &str) -> Result<(), WalletError> {
        self.database.app_security().wipe_all_wallet_data()?;
        self.database.initialize()?;
        self.set_master_password(password)?;
        let mnemonic = mnemonic::generate_mnemonic()?;
        self.create_decoy_wallet("Primary", &mnemonic, password)?;
        Ok(())
    }

    fn create_decoy_wallet(
        &self,
        label: &str,
        mnemonic: &str,
        password: &str,
    ) -> Result<WalletSummary, WalletError> {
        mnemonic::validate_mnemonic(mnemonic)?;
        let keystore = keystore::encrypt_mnemonic(mnemonic, password)?;
        let wallet = WalletSummary {
            id: Uuid::new_v4(),
            label: label.to_string(),
            created_at: Utc::now(),
        };
        let accounts = accounts::derive_default_accounts(wallet.id, mnemonic)?;
        self.database.wallets().save_wallet(
            &wallet,
            &keystore,
            &accounts,
            &chains::default_chain_settings(),
        )?;
        Ok(wallet)
    }
}
