pub mod activity;
pub mod app_security;
pub mod assets;
pub mod multisig;
pub mod network;
pub mod transfers;
pub mod wallets;

use super::WalletDatabase;

macro_rules! repository {
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name {
            pub(super) database: WalletDatabase,
        }

        impl $name {
            pub(crate) fn new(database: WalletDatabase) -> Self {
                Self { database }
            }
        }
    };
}

repository!(ActivityRepository);
repository!(AppSecurityRepository);
repository!(AssetRepository);
repository!(MultisigRepository);
repository!(NetworkRepository);
repository!(TransferRepository);
repository!(WalletRepository);
