use serde_json::json;
use wallet_core::domains::wallets as wallet_domain;

use super::support::{engine_for_path, parse_wallet_id};
use crate::response::{ok_json, wallet_error_response, WalletResponse};

pub(super) fn generate_mnemonic() -> WalletResponse {
    match wallet_domain::generate_mnemonic() {
        Ok(mnemonic) => ok_json(json!({ "mnemonic": mnemonic })),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn list_wallets(db_path: String) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match engine
        .initialize()
        .and_then(|_| engine.wallets().list_wallets())
    {
        Ok(wallets) => ok_json(wallets),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn list_accounts(db_path: String, wallet_id: String) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_wallet_id(&wallet_id).and_then(|wallet_id| {
        engine
            .initialize()
            .and_then(|_| engine.wallets().list_accounts(wallet_id))
    }) {
        Ok(accounts) => ok_json(accounts),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn create_wallet(
    db_path: String,
    label: String,
    mnemonic: String,
    password: String,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match engine
        .initialize()
        .and_then(|_| engine.wallets().create_wallet(&label, &mnemonic, &password))
    {
        Ok(wallet) => ok_json(wallet),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn import_private_key(
    db_path: String,
    label: String,
    private_key: String,
    password: String,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match engine.initialize().and_then(|_| {
        engine
            .wallets()
            .import_private_key(&label, &private_key, &password)
    }) {
        Ok(wallet) => ok_json(wallet),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn import_keystore(
    db_path: String,
    label: String,
    keystore_json: String,
    keystore_password: String,
    password: String,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match engine.initialize().and_then(|_| {
        engine
            .wallets()
            .import_keystore(&label, &keystore_json, &keystore_password, &password)
    }) {
        Ok(wallet) => ok_json(wallet),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn reveal_mnemonic(
    db_path: String,
    wallet_id: String,
    password: String,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_wallet_id(&wallet_id).and_then(|wallet_id| {
        engine
            .initialize()
            .and_then(|_| engine.wallets().reveal_mnemonic(wallet_id, &password))
    }) {
        Ok(mnemonic) => ok_json(json!({ "mnemonic": mnemonic })),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn export_keystore(
    db_path: String,
    wallet_id: String,
    password: String,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_wallet_id(&wallet_id).and_then(|wallet_id| {
        engine
            .initialize()
            .and_then(|_| engine.wallets().export_keystore(wallet_id, &password))
    }) {
        Ok(keystore) => ok_json(keystore),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn delete_wallet(
    db_path: String,
    wallet_id: String,
    password: String,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_wallet_id(&wallet_id).and_then(|wallet_id| {
        engine
            .initialize()
            .and_then(|_| engine.wallets().delete_wallet(wallet_id, &password))
    }) {
        Ok(()) => ok_json(json!({ "deleted": true })),
        Err(error) => wallet_error_response(error),
    }
}
