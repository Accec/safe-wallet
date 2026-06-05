use crate::error::WalletError;
use rusqlite::Connection;

pub(super) fn execute_schema(connection: &Connection, blocks: &[&str]) -> Result<(), WalletError> {
    for block in blocks {
        connection
            .execute_batch(block)
            .map_err(|_| WalletError::Storage)?;
    }
    Ok(())
}
