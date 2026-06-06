use crate::chains::{LEGACY_TRONGRID_RPC_URL, TRON_PUBLICNODE_RPC_URL};
use crate::error::WalletError;
use chrono::Utc;
use rusqlite::params;
use rusqlite::Connection;

pub(super) fn ensure_legacy_columns(connection: &Connection) -> Result<(), WalletError> {
    ensure_columns(
        connection,
        "app_security",
        &[
            ("duress_salt", "duress_salt text"),
            ("duress_verifier", "duress_verifier text"),
            ("duress_kdf_name", "duress_kdf_name text"),
            ("duress_kdf_params_json", "duress_kdf_params_json text"),
            ("duress_security_version", "duress_security_version integer"),
            (
                "biometric_enabled",
                "biometric_enabled integer not null default 0",
            ),
        ],
    )?;
    ensure_columns(
        connection,
        "keystore_items",
        &[(
            "secret_kind",
            "secret_kind text not null default 'mnemonic'",
        )],
    )?;
    ensure_columns(
        connection,
        "chain_settings",
        &[
            ("network_name", "network_name text not null default ''"),
            ("chain_id", "chain_id text"),
        ],
    )?;
    update_legacy_tron_rpc_url(connection)
}

fn update_legacy_tron_rpc_url(connection: &Connection) -> Result<(), WalletError> {
    let now = Utc::now().to_rfc3339();
    connection
        .execute(
            "update chain_settings
            set default_rpc_url = ?1, updated_at = ?2
            where chain = 'tron' and default_rpc_url = ?3",
            params![TRON_PUBLICNODE_RPC_URL, now, LEGACY_TRONGRID_RPC_URL],
        )
        .map_err(|_| WalletError::Storage)?;
    connection
        .execute(
            "update chain_settings
            set user_rpc_url = ?1, updated_at = ?2
            where chain = 'tron' and user_rpc_url = ?3",
            params![TRON_PUBLICNODE_RPC_URL, now, LEGACY_TRONGRID_RPC_URL],
        )
        .map_err(|_| WalletError::Storage)?;
    Ok(())
}

fn ensure_columns(
    connection: &Connection,
    table: &str,
    required_columns: &[(&str, &str)],
) -> Result<(), WalletError> {
    let columns = table_columns(connection, table)?;
    for (column, definition) in required_columns {
        if !columns.iter().any(|existing| existing == column) {
            connection
                .execute(&format!("alter table {table} add column {definition}"), [])
                .map_err(|_| WalletError::Storage)?;
        }
    }
    Ok(())
}

fn table_columns(connection: &Connection, table: &str) -> Result<Vec<String>, WalletError> {
    let mut statement = connection
        .prepare(&format!("pragma table_info({table})"))
        .map_err(|_| WalletError::Storage)?;
    let columns = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|_| WalletError::Storage)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| WalletError::Storage)?;
    Ok(columns)
}
