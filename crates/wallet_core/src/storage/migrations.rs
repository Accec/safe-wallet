use crate::error::WalletError;
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use std::fs;
use std::path::{Path, PathBuf};

pub const SCHEMA_VERSION: u32 = 1;

pub fn table_exists(connection: &Connection, table_name: &str) -> Result<bool, WalletError> {
    connection
        .query_row(
            "select 1 from sqlite_master where type = 'table' and name = ?1 limit 1",
            params![table_name],
            |_| Ok(()),
        )
        .optional()
        .map(|row| row.is_some())
        .map_err(|_| WalletError::Storage)
}

pub fn backup_path_for(path: &Path, target_version: u32) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("wallet.sqlite");
    path.with_file_name(format!("{file_name}.pre-v{target_version}.bak"))
}

pub fn backup_database(path: &Path, target_version: u32) -> Result<(), WalletError> {
    if path.as_os_str().is_empty() || !path.exists() {
        return Ok(());
    }
    let backup_path = backup_path_for(path, target_version);
    if backup_path.exists() {
        return Ok(());
    }
    fs::copy(path, backup_path)
        .map(|_| ())
        .map_err(|_| WalletError::Storage)
}

pub fn record_current_schema_version(connection: &Connection) -> Result<(), WalletError> {
    connection
        .execute(
            "insert or ignore into schema_migrations (version, applied_at) values (?1, ?2)",
            params![SCHEMA_VERSION, Utc::now().to_rfc3339()],
        )
        .map(|_| ())
        .map_err(|_| WalletError::Storage)
}
