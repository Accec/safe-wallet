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
fn send_custom_token_broadcasts_erc20_transfer_data() {
    let rpc = TestRpcServer::evm(serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": "0xtokenbroadcast"
    }));
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
        .update_chain_rpc(ChainId::Ethereum, &rpc.url)
        .unwrap();
    let client = static_token_client("TOKEN");
    let token = fixture
        .engine
        .assets()
        .add_custom_token_with(
            &client,
            ChainId::Ethereum,
            "0x0000000000000000000000000000000000000000",
            "TOKEN",
        )
        .unwrap();

    fixture
        .engine
        .transfers()
        .send_transfer(
            &TransferRequest {
                wallet_id: wallet.id,
                chain: ChainId::Ethereum,
                asset_id: token.id,
                to_address: "0x0000000000000000000000000000000000000000".to_string(),
                amount: "2".to_string(),
            },
            "master-password",
        )
        .unwrap();
    let activity = fixture.engine.activity().list_activity(wallet.id).unwrap();
    let estimate_request = rpc.request_for_method("eth_estimateGas");
    let tx = &estimate_request["params"][0];

    assert_eq!(tx["to"], "0x0000000000000000000000000000000000000000");
    assert!(tx["data"].as_str().unwrap().starts_with("0xa9059cbb"));
    assert_eq!(activity[0].kind, crate::models::ActivityKind::TokenTransfer);
    assert_eq!(activity[0].summary, "Sent 2 TOKEN");
}
