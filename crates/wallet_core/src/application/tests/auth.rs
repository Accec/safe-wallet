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
fn master_password_unlocks_app() {
    let fixture = engine_fixture();

    fixture
        .engine
        .auth()
        .set_master_password("master-password")
        .unwrap();
    fixture.engine.auth().unlock_app("master-password").unwrap();
}

#[test]
fn wrong_master_password_is_rejected() {
    let fixture = engine_fixture();

    fixture
        .engine
        .auth()
        .set_master_password("master-password")
        .unwrap();
    let error = fixture
        .engine
        .auth()
        .unlock_app("wrong-password")
        .unwrap_err();

    assert_eq!(error, WalletError::InvalidPassword);
}

#[test]
fn duress_password_wipes_real_wallets_and_creates_decoy_wallet() {
    let fixture = engine_fixture();

    fixture
        .engine
        .auth()
        .set_master_password("master-password")
        .unwrap();
    let real_wallet = fixture
        .engine
        .wallets()
        .create_wallet("Real Wallet", MNEMONIC, "master-password")
        .unwrap();
    fixture
        .engine
        .auth()
        .set_duress_password("master-password", "duress-password")
        .unwrap();

    fixture.engine.auth().unlock_app("duress-password").unwrap();
    let wallets = fixture.engine.wallets().list_wallets().unwrap();

    assert_eq!(wallets.len(), 1);
    assert_eq!(wallets[0].label, "Primary");
    assert_ne!(wallets[0].id, real_wallet.id);
    assert_eq!(
        fixture
            .engine
            .auth()
            .unlock_app("master-password")
            .unwrap_err(),
        WalletError::InvalidPassword
    );
    fixture.engine.auth().unlock_app("duress-password").unwrap();
    assert_eq!(
        fixture
            .engine
            .wallets()
            .list_accounts(wallets[0].id)
            .unwrap()
            .len(),
        7
    );
    assert_eq!(
        fixture
            .engine
            .assets()
            .list_assets(wallets[0].id)
            .unwrap()
            .len(),
        7
    );
}

#[test]
fn duress_password_cannot_match_master_password() {
    let fixture = engine_fixture();

    fixture
        .engine
        .auth()
        .set_master_password("master-password")
        .unwrap();
    let error = fixture
        .engine
        .auth()
        .set_duress_password("master-password", "master-password")
        .unwrap_err();

    assert_eq!(error, WalletError::InvalidPassword);
}

#[test]
fn master_password_setup_is_first_run_only() {
    let fixture = engine_fixture();

    fixture
        .engine
        .auth()
        .set_master_password("master-password")
        .unwrap();
    let result = fixture
        .engine
        .auth()
        .set_master_password("replacement-password");

    assert_eq!(result.unwrap_err(), WalletError::MasterPasswordAlreadySet);
    fixture.engine.auth().unlock_app("master-password").unwrap();
    assert_eq!(
        fixture
            .engine
            .auth()
            .unlock_app("replacement-password")
            .unwrap_err(),
        WalletError::InvalidPassword
    );
}

#[test]
fn concurrent_master_password_setup_persists_only_one_verifier() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    WalletEngine::new(WalletDatabase::new(&db_path))
        .initialize()
        .unwrap();
    let barrier = Arc::new(Barrier::new(2));

    let handles = ["first-password", "second-password"].map(|password| {
        let db_path = db_path.clone();
        let barrier = Arc::clone(&barrier);
        thread::spawn(move || {
            let engine = WalletEngine::new(WalletDatabase::new(&db_path));
            barrier.wait();
            (password, engine.auth().set_master_password(password))
        })
    });
    let results = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect::<Vec<_>>();
    let successful_passwords = results
        .iter()
        .filter_map(|(password, result)| result.as_ref().ok().map(|_| *password))
        .collect::<Vec<_>>();
    let rejected_passwords = results
        .iter()
        .filter_map(|(password, result)| match result {
            Err(WalletError::MasterPasswordAlreadySet) => Some(*password),
            _ => None,
        })
        .collect::<Vec<_>>();
    let engine = WalletEngine::new(WalletDatabase::new(&db_path));

    assert_eq!(successful_passwords.len(), 1);
    assert_eq!(rejected_passwords.len(), 1);
    engine.auth().unlock_app(successful_passwords[0]).unwrap();
    assert_eq!(
        engine.auth().unlock_app(rejected_passwords[0]).unwrap_err(),
        WalletError::InvalidPassword
    );
}
