use super::WalletDatabase;
use crate::error::WalletError;
use rusqlite::Connection;
use std::path::Path;

impl WalletDatabase {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn connect(&self) -> Result<Connection, WalletError> {
        let connection = Connection::open(&self.path).map_err(|_| WalletError::Storage)?;
        connection
            .execute_batch("pragma foreign_keys = on;")
            .map_err(|_| WalletError::Storage)?;
        Ok(connection)
    }
}
