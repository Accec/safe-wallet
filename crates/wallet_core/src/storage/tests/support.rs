use crate::storage::WalletDatabase;
use rusqlite::{params, Connection};

pub(super) fn initialized_connection() -> (tempfile::TempDir, Connection) {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let database = WalletDatabase::new(&db_path);

    database.initialize().unwrap();
    let connection = database.connect().unwrap();

    (dir, connection)
}

pub(super) fn insert_wallet_with_account(
    connection: &Connection,
    wallet_id: &str,
    keystore_id: &str,
    account_id: &str,
) {
    insert_keystore(connection, keystore_id);
    insert_wallet(connection, wallet_id, keystore_id);
    insert_account(connection, account_id, wallet_id);
}

pub(super) fn insert_keystore(connection: &Connection, id: &str) {
    connection
        .execute(
            "insert into keystore_items (
                id, ciphertext, nonce, salt, kdf_name, kdf_params_json,
                cipher_name, version, created_at, updated_at
            ) values (?1, 'ciphertext', 'nonce', 'salt', 'scrypt', '{}', 'aes-gcm', 1, '2026-06-03', '2026-06-03')",
            params![id],
        )
        .unwrap();
}

pub(super) fn insert_wallet(connection: &Connection, id: &str, keystore_id: &str) {
    connection
        .execute(
            "insert into wallets (
                id, label, keystore_id, hidden, created_at, updated_at
            ) values (?1, 'Wallet', ?2, 0, '2026-06-03', '2026-06-03')",
            params![id, keystore_id],
        )
        .unwrap();
}

fn insert_account(connection: &Connection, id: &str, wallet_id: &str) {
    connection
        .execute(
            "insert into accounts (
                id, wallet_id, chain, address, derivation_path, account_index, created_at
            ) values (?1, ?2, 'eth', '0xabc', \"m/44'/60'/0'/0/0\", 0, '2026-06-03')",
            params![id, wallet_id],
        )
        .unwrap();
}

pub(super) fn insert_native_token(connection: &Connection, id: &str, chain: &str) {
    connection
        .execute(
            "insert into tokens (
                id, chain, contract_address, kind, symbol, name, decimals, source, visible, updated_at
            ) values (?1, ?2, null, 'native', 'ETH', 'Ether', 18, 'system', 1, '2026-06-03')",
            params![id, chain],
        )
        .unwrap();
}

pub(super) fn insert_contract_token(
    connection: &Connection,
    id: &str,
    chain: &str,
    contract: &str,
) {
    connection
        .execute(
            "insert into tokens (
                id, chain, contract_address, kind, symbol, name, decimals, source, visible, updated_at
            ) values (?1, ?2, ?3, 'erc20', 'USDT', 'Tether', 6, 'user', 1, '2026-06-03')",
            params![id, chain, contract],
        )
        .unwrap();
}
