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
fn send_transfer_broadcasts_tron_native_transaction() {
    let rpc = TestRpcServer::tron();
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
        .update_chain_rpc(ChainId::Tron, &rpc.url)
        .unwrap();
    let asset = fixture
        .engine
        .assets()
        .list_assets(wallet.id)
        .unwrap()
        .into_iter()
        .find(|asset: &Asset| asset.chain == ChainId::Tron && asset.kind == AssetKind::Native)
        .unwrap();
    let result = fixture
        .engine
        .transfers()
        .send_transfer(
            &TransferRequest {
                wallet_id: wallet.id,
                chain: ChainId::Tron,
                asset_id: asset.id,
                to_address: "TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7".to_string(),
                amount: "1".to_string(),
                block_if_energy_insufficient: false,
            },
            "master-password",
        )
        .unwrap();
    let create_body = rpc.request_for_path("/wallet/createtransaction");
    let broadcast_body = rpc.request_for_path("/wallet/broadcasttransaction");
    let activity = fixture.engine.activity().list_activity(wallet.id).unwrap();

    assert_eq!(result.tx_hash, "tron-native-tx-id");
    assert_eq!(result.status, "broadcasted");
    assert_eq!(
        rpc.paths(),
        vec![
            "/wallet/getaccount",
            "/wallet/createtransaction",
            "/wallet/broadcasttransaction"
        ]
    );
    assert_eq!(create_body["amount"], 1_000_000_u64);
    assert!(broadcast_body["signature"][0].as_str().unwrap().len() >= 130);
    assert_eq!(activity[0].summary, "Sent 1 TRX");
}

#[test]
fn send_transfer_broadcasts_trc20_transaction() {
    let rpc = TestRpcServer::tron();
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
        .update_chain_rpc(ChainId::Tron, &rpc.url)
        .unwrap();
    let token = fixture
        .engine
        .assets()
        .add_custom_token_with(
            &static_token_client("USDT"),
            ChainId::Tron,
            "TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7",
            "USDT",
        )
        .unwrap();

    let result = fixture
        .engine
        .transfers()
        .send_transfer(
            &TransferRequest {
                wallet_id: wallet.id,
                chain: ChainId::Tron,
                asset_id: token.id,
                to_address: "TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7".to_string(),
                amount: "2.5".to_string(),
                block_if_energy_insufficient: false,
            },
            "master-password",
        )
        .unwrap();
    let trigger_body = rpc.request_for_path("/wallet/triggersmartcontract");
    let activity = fixture.engine.activity().list_activity(wallet.id).unwrap();

    assert_eq!(result.tx_hash, "tron-token-tx-id");
    assert_eq!(result.status, "broadcasted");
    assert_eq!(
        rpc.paths(),
        vec![
            "/wallet/getaccount",
            "/wallet/getaccountresource",
            "/wallet/triggerconstantcontract",
            "/wallet/triggerconstantcontract",
            "/wallet/triggersmartcontract",
            "/wallet/broadcasttransaction"
        ]
    );
    assert_eq!(
        trigger_body["function_selector"],
        "transfer(address,uint256)"
    );
    assert!(trigger_body["parameter"]
        .as_str()
        .unwrap()
        .ends_with("00000000000000000000000000000000000000000000000000000000002625a0"));
    assert_eq!(activity[0].summary, "Sent 2.5 USDT");
}
