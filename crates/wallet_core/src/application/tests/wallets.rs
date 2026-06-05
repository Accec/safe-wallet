#![allow(unused_imports)]

use super::super::WalletEngine;
use super::support::*;
use crate::domains::auth::security;
use crate::domains::wallets::keystore;
use crate::error::WalletError;
use crate::models::*;
use crate::protocol::rpc::AssetBalanceClient;
use crate::storage::WalletDatabase;
use std::fs;
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use uuid::Uuid;

#[test]
fn creates_multiple_wallets() {
    let fixture = engine_fixture();

    fixture
        .engine
        .auth()
        .set_master_password("master-password")
        .unwrap();
    let first = fixture
        .engine
        .wallets()
        .create_wallet("Primary", MNEMONIC, "master-password")
        .unwrap();
    let second = fixture
        .engine
        .wallets()
        .create_wallet("Trading", MNEMONIC, "master-password")
        .unwrap();
    let wallets = fixture.engine.wallets().list_wallets().unwrap();

    assert_ne!(first.id, Uuid::nil());
    assert_ne!(first.id, second.id);
    assert_eq!(wallets.len(), 2);
}

#[test]
fn delete_wallet_requires_password_and_removes_local_wallet_data() {
    let fixture = engine_fixture();

    fixture
        .engine
        .auth()
        .set_master_password("master-password")
        .unwrap();
    let first = fixture
        .engine
        .wallets()
        .create_wallet("Primary", MNEMONIC, "master-password")
        .unwrap();
    let second = fixture
        .engine
        .wallets()
        .create_wallet("Trading", MNEMONIC, "master-password")
        .unwrap();

    let wrong_password = fixture
        .engine
        .wallets()
        .delete_wallet(first.id, "wrong-password")
        .unwrap_err();
    assert_eq!(wrong_password, WalletError::InvalidPassword);
    assert_eq!(fixture.engine.wallets().list_wallets().unwrap().len(), 2);

    fixture
        .engine
        .wallets()
        .delete_wallet(first.id, "master-password")
        .unwrap();
    let wallets = fixture.engine.wallets().list_wallets().unwrap();

    assert_eq!(wallets, vec![second.clone()]);
    assert_eq!(
        fixture
            .engine
            .wallets()
            .list_accounts(first.id)
            .unwrap_err(),
        WalletError::WalletNotFound
    );
    assert_eq!(
        fixture
            .engine
            .wallets()
            .export_keystore(first.id, "master-password")
            .unwrap_err(),
        WalletError::WalletNotFound
    );
    assert!(!fixture
        .engine
        .wallets()
        .list_accounts(second.id)
        .unwrap()
        .is_empty());
}

#[test]
fn private_key_import_persists_accounts_without_plaintext_and_exports_keystore() {
    let fixture = engine_fixture();
    let private_key = "0000000000000000000000000000000000000000000000000000000000000001";

    fixture
        .engine
        .auth()
        .set_master_password("master-password")
        .unwrap();
    let wallet = fixture
        .engine
        .wallets()
        .import_private_key("Key Wallet", private_key, "master-password")
        .unwrap();
    let accounts = fixture.engine.wallets().list_accounts(wallet.id).unwrap();
    let export = fixture
        .engine
        .wallets()
        .export_keystore(wallet.id, "master-password")
        .unwrap();
    let wrong_password = fixture
        .engine
        .wallets()
        .export_keystore(wallet.id, "wrong-password")
        .unwrap_err();
    let database_bytes = fs::read(&fixture.db_path).unwrap();
    let database_text = String::from_utf8_lossy(&database_bytes);

    assert_eq!(wallet.label, "Key Wallet");
    assert_eq!(accounts.len(), 7);
    assert_eq!(
        accounts
            .iter()
            .find(|account| account.chain == ChainId::Ethereum)
            .unwrap()
            .address,
        "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf"
    );
    assert_eq!(export.wallet_id, wallet.id);
    assert_eq!(export.label, "Key Wallet");
    assert_eq!(export.secret_kind, "private_key");
    assert_eq!(wrong_password, WalletError::InvalidPassword);
    assert!(!export.ciphertext_b64.contains(private_key));
    assert!(!database_text.contains(private_key));
    assert!(!database_text.contains("master-password"));
}

#[test]
fn keystore_import_decrypts_export_and_reencrypts_with_current_master_password() {
    let source = engine_fixture();
    source
        .engine
        .auth()
        .set_master_password("source-password")
        .unwrap();
    let source_wallet = source
        .engine
        .wallets()
        .create_wallet("Source", MNEMONIC, "source-password")
        .unwrap();
    let exported = source
        .engine
        .wallets()
        .export_keystore(source_wallet.id, "source-password")
        .unwrap();
    let exported_json = serde_json::to_string(&exported).unwrap();

    let target = engine_fixture();
    target
        .engine
        .auth()
        .set_master_password("target-password")
        .unwrap();
    let imported = target
        .engine
        .wallets()
        .import_keystore(
            "Imported",
            &exported_json,
            "source-password",
            "target-password",
        )
        .unwrap();
    let wallets = target.engine.wallets().list_wallets().unwrap();
    let accounts = target.engine.wallets().list_accounts(imported.id).unwrap();

    assert_eq!(wallets.len(), 1);
    assert_eq!(wallets[0].label, "Imported");
    assert_eq!(accounts.len(), 7);
    assert_ne!(imported.id, source_wallet.id);
    assert!(target
        .engine
        .wallets()
        .export_keystore(imported.id, "target-password")
        .is_ok());
    assert_eq!(
        target
            .engine
            .wallets()
            .export_keystore(imported.id, "source-password")
            .unwrap_err(),
        WalletError::InvalidPassword
    );
}

#[test]
fn wallet_creation_persists_accounts_and_default_assets() {
    let fixture = engine_fixture();

    fixture
        .engine
        .auth()
        .set_master_password("master-password")
        .unwrap();
    let wallet = fixture
        .engine
        .wallets()
        .create_wallet("Primary", MNEMONIC, "master-password")
        .unwrap();
    let accounts = fixture.engine.wallets().list_accounts(wallet.id).unwrap();
    let assets = fixture.engine.assets().list_assets(wallet.id).unwrap();

    assert_eq!(accounts.len(), 7);
    let ethereum_asset = assets
        .iter()
        .find(|asset| asset.chain == ChainId::Ethereum)
        .unwrap();
    let ethereum_asset_json = serde_json::to_value(ethereum_asset).unwrap();
    assert_eq!(ethereum_asset_json["balance"], "0");
    assert!(accounts.iter().any(|account| account.chain == ChainId::Btc));
    assert!(assets.iter().any(|asset| {
        asset.chain == ChainId::Ethereum && asset.kind == crate::models::AssetKind::Native
    }));
    assert!(assets.iter().any(|asset| asset.symbol == "TRX"));
}

#[test]
fn mnemonic_reveal_requires_wallet_password_and_returns_only_explicit_secret() {
    let fixture = engine_fixture();

    fixture
        .engine
        .auth()
        .set_master_password("master-password")
        .unwrap();
    let wallet = fixture
        .engine
        .wallets()
        .create_wallet("Primary", MNEMONIC, "master-password")
        .unwrap();

    let mnemonic = fixture
        .engine
        .wallets()
        .reveal_mnemonic(wallet.id, "master-password")
        .unwrap();
    let wrong_password = fixture
        .engine
        .wallets()
        .reveal_mnemonic(wallet.id, "wrong-password")
        .unwrap_err();

    assert_eq!(mnemonic, MNEMONIC);
    assert_eq!(wrong_password, WalletError::InvalidPassword);
}

#[test]
fn persisted_database_does_not_contain_plaintext_secrets() {
    let fixture = engine_fixture();

    fixture
        .engine
        .auth()
        .set_master_password("master-password")
        .unwrap();
    fixture
        .engine
        .wallets()
        .create_wallet("Primary", MNEMONIC, "master-password")
        .unwrap();
    let database = WalletDatabase::new(&fixture.db_path);
    let verifier = database
        .app_security()
        .load_master_password_verifier()
        .unwrap()
        .unwrap();
    let wallets = database.wallets().list_wallets().unwrap();
    let connection = database.connect().unwrap();
    let (kdf_name, kdf_params_json, cipher_name, version): (String, String, String, u32) =
        connection
            .query_row(
                "select kdf_name, kdf_params_json, cipher_name, version from keystore_items",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .unwrap();
    let database_bytes = fs::read(&fixture.db_path).unwrap();
    let database_text = String::from_utf8_lossy(&database_bytes);

    assert_eq!(verifier.kdf_name, security::KDF_NAME);
    assert_eq!(verifier.kdf_params_json, security::KDF_PARAMS_JSON);
    assert_eq!(verifier.version, security::CRYPTO_VERSION);
    assert_eq!(wallets.len(), 1);
    assert_eq!(kdf_name, security::KDF_NAME);
    assert_eq!(kdf_params_json, security::KDF_PARAMS_JSON);
    assert_eq!(cipher_name, keystore::CIPHER_NAME);
    assert_eq!(version, security::CRYPTO_VERSION);
    assert!(!database_text.contains("abandon abandon"));
    assert!(!database_text.contains("master-password"));
}
