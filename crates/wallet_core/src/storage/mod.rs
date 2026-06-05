pub mod migrations;
pub mod repositories;

mod connection;
mod mappers;
mod schema;

pub use mappers::chain_to_db;

use std::path::PathBuf;

#[derive(Clone)]
pub struct WalletDatabase {
    pub(crate) path: PathBuf,
}

impl WalletDatabase {
    pub(crate) fn activity(&self) -> repositories::ActivityRepository {
        repositories::ActivityRepository::new(self.clone())
    }

    pub(crate) fn app_security(&self) -> repositories::AppSecurityRepository {
        repositories::AppSecurityRepository::new(self.clone())
    }

    pub(crate) fn assets(&self) -> repositories::AssetRepository {
        repositories::AssetRepository::new(self.clone())
    }

    pub(crate) fn multisig(&self) -> repositories::MultisigRepository {
        repositories::MultisigRepository::new(self.clone())
    }

    pub(crate) fn network(&self) -> repositories::NetworkRepository {
        repositories::NetworkRepository::new(self.clone())
    }

    pub(crate) fn transfers(&self) -> repositories::TransferRepository {
        repositories::TransferRepository::new(self.clone())
    }

    pub(crate) fn wallets(&self) -> repositories::WalletRepository {
        repositories::WalletRepository::new(self.clone())
    }
}

#[cfg(test)]
mod tests;
