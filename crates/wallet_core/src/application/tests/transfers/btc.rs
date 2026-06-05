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
fn send_transfer_broadcasts_btc_transaction_with_esplora_utxos() {
    let rpc = TestRpcServer::btc();
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
    fixture
        .engine
        .network()
        .update_chain_rpc(ChainId::Btc, &rpc.url)
        .unwrap();
    let asset = fixture
        .engine
        .assets()
        .list_assets(wallet.id)
        .unwrap()
        .into_iter()
        .find(|asset: &Asset| asset.chain == ChainId::Btc && asset.kind == AssetKind::Native)
        .unwrap();

    let result = fixture
        .engine
        .transfers()
        .send_transfer(
            &TransferRequest {
                wallet_id: wallet.id,
                chain: ChainId::Btc,
                asset_id: asset.id,
                to_address: "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu".to_string(),
                amount: "0.001".to_string(),
            },
            "master-password",
        )
        .unwrap();
    let activity = fixture.engine.activity().list_activity(wallet.id).unwrap();

    assert_eq!(result.status, "broadcasted");
    assert_eq!(result.tx_hash.len(), 64);
    assert!(result.tx_hash.chars().all(|ch| ch.is_ascii_hexdigit()));
    assert_eq!(
        rpc.paths(),
        vec![
            "/fee-estimates",
            "/address/bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu/utxo",
            "/tx"
        ]
    );
    let raw_tx = rpc.body_text_for_path("/tx");
    assert!(raw_tx.len() > 200);
    assert!(raw_tx.chars().all(|ch| ch.is_ascii_hexdigit()));
    assert_eq!(activity.len(), 1);
    assert_eq!(activity[0].tx_hash, result.tx_hash);
    assert_eq!(activity[0].summary, "Sent 0.001 BTC");
}
