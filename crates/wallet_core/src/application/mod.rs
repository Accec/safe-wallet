use crate::domains;
use crate::error::WalletError;
use crate::storage::WalletDatabase;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AppStatus {
    pub initialized: bool,
    pub locked: bool,
    pub biometric_enabled: bool,
}

pub struct WalletEngine {
    database: WalletDatabase,
}

impl WalletEngine {
    pub fn new(database: WalletDatabase) -> Self {
        Self { database }
    }

    pub fn initialize(&self) -> Result<(), WalletError> {
        self.database.initialize()
    }

    pub fn app_status(&self) -> Result<AppStatus, WalletError> {
        self.database.initialize()?;
        let initialized = self
            .database
            .app_security()
            .load_master_password_verifier()?
            .is_some();
        let biometric_enabled = if initialized {
            self.database.app_security().biometric_enabled()?
        } else {
            false
        };
        Ok(AppStatus {
            initialized,
            locked: initialized,
            biometric_enabled,
        })
    }

    pub fn auth(&self) -> domains::auth::AuthDomain {
        domains::auth::AuthDomain::new(self.database.clone())
    }

    pub fn wallets(&self) -> domains::wallets::WalletsDomain {
        domains::wallets::WalletsDomain::new(self.database.clone())
    }

    pub fn assets(&self) -> domains::assets::AssetsDomain {
        domains::assets::AssetsDomain::new(self.database.clone())
    }

    pub fn transfers(&self) -> domains::transfers::TransfersDomain {
        domains::transfers::TransfersDomain::new(self.database.clone())
    }

    pub fn activity(&self) -> domains::activity::ActivityDomain {
        domains::activity::ActivityDomain::new(self.database.clone())
    }

    pub fn multisig(&self) -> domains::multisig::MultisigDomain {
        domains::multisig::MultisigDomain::new(self.database.clone())
    }

    pub fn network(&self) -> domains::network::NetworkDomain {
        domains::network::NetworkDomain::new(self.database.clone())
    }
}

#[cfg(test)]
mod tests;
