use super::{migrations, WalletDatabase};
use crate::error::WalletError;

mod bootstrap;
mod compatibility;

impl WalletDatabase {
    pub fn initialize(&self) -> Result<(), WalletError> {
        let connection = self.connect()?;
        let had_schema_migrations = migrations::table_exists(&connection, "schema_migrations")?;
        let had_legacy_wallet_data =
            !had_schema_migrations && migrations::table_exists(&connection, "wallets")?;
        if had_legacy_wallet_data {
            migrations::backup_database(&self.path, migrations::SCHEMA_VERSION)?;
        }

        bootstrap::create_schema(&connection)?;
        compatibility::ensure_legacy_columns(&connection)?;
        migrations::record_current_schema_version(&connection)?;
        Ok(())
    }
}
