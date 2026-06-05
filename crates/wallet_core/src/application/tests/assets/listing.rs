#![allow(unused_imports)]

use super::super::super::WalletEngine;
use super::super::support::*;
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
fn list_assets_includes_saved_balance_for_wallet_account() {
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
    let asset = fixture
        .engine
        .assets()
        .list_assets(wallet.id)
        .unwrap()
        .into_iter()
        .find(|asset| asset.chain == ChainId::Ethereum)
        .unwrap();
    let asset_id = asset.id;
    let connection = rusqlite::Connection::open(&fixture.db_path).unwrap();
    let account_id: String = connection
        .query_row(
            "select id from accounts where wallet_id = ?1 and chain = 'ethereum'",
            [wallet.id.to_string()],
            |row| row.get(0),
        )
        .unwrap();
    connection
        .execute(
            "insert into asset_balances (
                id, wallet_id, account_id, asset_id, balance, block_height,
                source, refreshed_at
            ) values (?1, ?2, ?3, ?4, '12.5', 123, 'indexer', '2026-06-03')",
            [
                Uuid::new_v4().to_string(),
                wallet.id.to_string(),
                account_id,
                asset_id.to_string(),
            ],
        )
        .unwrap();

    let refreshed = fixture.engine.assets().list_assets(wallet.id).unwrap();
    let refreshed_asset = refreshed.iter().find(|asset| asset.id == asset_id).unwrap();

    assert_eq!(refreshed_asset.balance, "12.5");
}
