use super::super::AppSecurityRepository;
use crate::error::WalletError;
use chrono::Utc;
use rusqlite::{params, OptionalExtension};

impl AppSecurityRepository {
    pub fn biometric_enabled(&self) -> Result<bool, WalletError> {
        let connection = self.database.connect()?;
        connection
            .query_row(
                "select biometric_enabled from app_security where id = 1",
                [],
                |row| row.get::<_, i64>(0),
            )
            .optional()
            .map(|value| value.unwrap_or(0) != 0)
            .map_err(|_| WalletError::Storage)
    }

    pub fn update_biometric_enabled(&self, enabled: bool) -> Result<(), WalletError> {
        let connection = self.database.connect()?;
        let updated = connection
            .execute(
                "update app_security
                set biometric_enabled = ?1, updated_at = ?2
                where id = 1",
                params![if enabled { 1 } else { 0 }, Utc::now().to_rfc3339()],
            )
            .map_err(|_| WalletError::Storage)?;
        if updated == 0 {
            return Err(WalletError::InvalidPassword);
        }
        Ok(())
    }
}
