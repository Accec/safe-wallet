use super::support::{
    initialized_connection, insert_contract_token, insert_keystore, insert_native_token,
    insert_wallet, insert_wallet_with_account,
};
use rusqlite::params;

#[test]
fn duplicate_native_token_for_same_chain_fails() {
    let (_dir, connection) = initialized_connection();

    insert_native_token(&connection, "eth-native-1", "eth");
    let result = connection.execute(
        "insert into tokens (
            id, chain, contract_address, kind, symbol, name, decimals, source, visible, updated_at
        ) values (?1, ?2, null, 'native', 'ETH', 'Ether', 18, 'system', 1, '2026-06-03')",
        params!["eth-native-2", "eth"],
    );

    assert!(result.is_err());
}

#[test]
fn duplicate_contract_token_for_same_chain_and_contract_fails() {
    let (_dir, connection) = initialized_connection();

    insert_contract_token(&connection, "usdt-1", "eth", "0x1234");
    let result = connection.execute(
        "insert into tokens (
            id, chain, contract_address, kind, symbol, name, decimals, source, visible, updated_at
        ) values (?1, ?2, ?3, 'erc20', 'USDT', 'Tether', 6, 'user', 1, '2026-06-03')",
        params!["usdt-2", "eth", "0x1234"],
    );

    assert!(result.is_err());
}

#[test]
fn asset_balance_rejects_mismatched_wallet_and_account() {
    let (_dir, connection) = initialized_connection();
    insert_wallet_with_account(&connection, "wallet-1", "keystore-1", "account-1");
    insert_wallet_with_account(&connection, "wallet-2", "keystore-2", "account-2");
    insert_native_token(&connection, "eth-native", "eth");

    let result = connection.execute(
        "insert into asset_balances (
            id, wallet_id, account_id, asset_id, balance, block_height, source, refreshed_at
        ) values (
            'balance-1', 'wallet-2', 'account-1', 'eth-native', '100', 1, 'indexer', '2026-06-03'
        )",
        [],
    );

    assert!(result.is_err());
}

#[test]
fn wallet_keystore_id_must_be_unique() {
    let (_dir, connection) = initialized_connection();
    insert_keystore(&connection, "keystore-1");
    insert_wallet(&connection, "wallet-1", "keystore-1");

    let result = connection.execute(
        "insert into wallets (
            id, label, keystore_id, hidden, created_at, updated_at
        ) values ('wallet-2', 'Wallet 2', 'keystore-1', 0, '2026-06-03', '2026-06-03')",
        [],
    );

    assert!(result.is_err());
}
