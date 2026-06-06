use crate::storage::WalletDatabase;
use rusqlite::Connection;

#[test]
fn initialize_creates_required_tables() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let database = WalletDatabase::new(&db_path);

    database.initialize().unwrap();

    let connection = Connection::open(db_path).unwrap();
    let mut statement = connection
        .prepare("select name from sqlite_master where type = 'table' order by name")
        .unwrap();
    let tables = statement
        .query_map([], |row| row.get::<_, String>(0))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert!(tables.contains(&"app_security".to_string()));
    assert!(tables.contains(&"wallets".to_string()));
    assert!(tables.contains(&"keystore_items".to_string()));
    assert!(tables.contains(&"accounts".to_string()));
    assert!(tables.contains(&"chain_settings".to_string()));
    assert!(tables.contains(&"indexer_settings".to_string()));
    assert!(tables.contains(&"network_privacy_settings".to_string()));
    assert!(tables.contains(&"tokens".to_string()));
    assert!(tables.contains(&"asset_balances".to_string()));
    assert!(tables.contains(&"activities".to_string()));
    assert!(tables.contains(&"transactions".to_string()));
    assert!(tables.contains(&"ui_preferences".to_string()));
    assert!(tables.contains(&"schema_migrations".to_string()));

    let version: i64 = connection
        .query_row(
            "select version from schema_migrations order by version desc limit 1",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(version, crate::storage::migrations::SCHEMA_VERSION as i64);
}

#[test]
fn legacy_database_is_baselined_and_backed_up_before_schema_version_is_recorded() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let connection = Connection::open(&db_path).unwrap();
    connection
        .execute_batch(
            r#"
            create table wallets (
                id text primary key,
                label text not null,
                keystore_id text not null unique,
                hidden integer not null default 0,
                created_at text not null,
                updated_at text not null
            );
            insert into wallets (id, label, keystore_id, created_at, updated_at)
            values ('legacy-wallet', 'Legacy', 'legacy-keystore', '2026-06-05T00:00:00Z', '2026-06-05T00:00:00Z');
            "#,
        )
        .unwrap();
    drop(connection);

    let database = WalletDatabase::new(&db_path);
    database.initialize().unwrap();

    let backup_path = crate::storage::migrations::backup_path_for(&db_path, 1);
    assert!(backup_path.exists());

    let connection = Connection::open(&db_path).unwrap();
    let label: String = connection
        .query_row(
            "select label from wallets where id = 'legacy-wallet'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let version: i64 = connection
        .query_row("select max(version) from schema_migrations", [], |row| {
            row.get(0)
        })
        .unwrap();

    assert_eq!(label, "Legacy");
    assert_eq!(version, crate::storage::migrations::SCHEMA_VERSION as i64);
}

#[test]
fn initialize_updates_legacy_tron_grid_default_rpc_url() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let connection = Connection::open(&db_path).unwrap();
    connection
        .execute_batch(
            r#"
            create table schema_migrations (
                version integer primary key,
                applied_at text not null
            );
            insert into schema_migrations (version, applied_at)
            values (1, '2026-06-06T00:00:00Z');
            create table chain_settings (
                chain text primary key,
                network_name text not null default '',
                chain_id text,
                enabled integer not null,
                default_rpc_url text not null,
                user_rpc_url text,
                explorer_url text,
                native_symbol text not null,
                native_decimals integer not null,
                updated_at text not null
            );
            insert into chain_settings (
                chain, network_name, chain_id, enabled, default_rpc_url,
                user_rpc_url, explorer_url, native_symbol, native_decimals,
                updated_at
            ) values (
                'tron', 'Tron', '728126428', 1, 'https://api.trongrid.io',
                null, 'https://tronscan.org', 'TRX', 6, '2026-06-06T00:00:00Z'
            );
            "#,
        )
        .unwrap();
    drop(connection);

    let database = WalletDatabase::new(&db_path);
    database.initialize().unwrap();

    let connection = Connection::open(&db_path).unwrap();
    let (default_rpc_url, user_rpc_url): (String, Option<String>) = connection
        .query_row(
            "select default_rpc_url, user_rpc_url from chain_settings where chain = 'tron'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();

    assert_eq!(default_rpc_url, "https://tron-rpc.publicnode.com");
    assert_eq!(user_rpc_url, None);
}

#[test]
fn initialize_updates_legacy_tron_grid_user_rpc_url_without_overwriting_custom_urls() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let connection = Connection::open(&db_path).unwrap();
    connection
        .execute_batch(
            r#"
            create table schema_migrations (
                version integer primary key,
                applied_at text not null
            );
            insert into schema_migrations (version, applied_at)
            values (1, '2026-06-06T00:00:00Z');
            create table chain_settings (
                chain text primary key,
                network_name text not null default '',
                chain_id text,
                enabled integer not null,
                default_rpc_url text not null,
                user_rpc_url text,
                explorer_url text,
                native_symbol text not null,
                native_decimals integer not null,
                updated_at text not null
            );
            insert into chain_settings (
                chain, network_name, chain_id, enabled, default_rpc_url,
                user_rpc_url, explorer_url, native_symbol, native_decimals,
                updated_at
            ) values (
                'tron', 'Tron', '728126428', 1, 'https://api.trongrid.io',
                'https://api.trongrid.io', 'https://tronscan.org', 'TRX', 6,
                '2026-06-06T00:00:00Z'
            );
            insert into chain_settings (
                chain, network_name, chain_id, enabled, default_rpc_url,
                user_rpc_url, explorer_url, native_symbol, native_decimals,
                updated_at
            ) values (
                'ethereum', 'Ethereum', '1', 1, 'https://ethereum-rpc.publicnode.com',
                'https://custom.example.invalid/rpc', 'https://etherscan.io', 'ETH', 18,
                '2026-06-06T00:00:00Z'
            );
            "#,
        )
        .unwrap();
    drop(connection);

    let database = WalletDatabase::new(&db_path);
    database.initialize().unwrap();

    let connection = Connection::open(&db_path).unwrap();
    let tron_user_rpc_url: String = connection
        .query_row(
            "select user_rpc_url from chain_settings where chain = 'tron'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let ethereum_user_rpc_url: String = connection
        .query_row(
            "select user_rpc_url from chain_settings where chain = 'ethereum'",
            [],
            |row| row.get(0),
        )
        .unwrap();

    assert_eq!(tron_user_rpc_url, "https://tron-rpc.publicnode.com");
    assert_eq!(ethereum_user_rpc_url, "https://custom.example.invalid/rpc");
}

#[test]
fn connect_enables_foreign_keys() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let database = WalletDatabase::new(&db_path);

    database.initialize().unwrap();
    let connection = database.connect().unwrap();
    let enabled: i64 = connection
        .query_row("pragma foreign_keys", [], |row| row.get(0))
        .unwrap();

    assert_eq!(enabled, 1);
}
