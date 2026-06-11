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
fn send_transfer_broadcasts_evm_native_transaction_and_records_activity() {
    let rpc = TestRpcServer::evm(serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": "0xbroadcasted"
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
    let asset = fixture
        .engine
        .assets()
        .list_assets(wallet.id)
        .unwrap()
        .into_iter()
        .find(|asset: &Asset| asset.chain == ChainId::Ethereum)
        .unwrap();
    let result = fixture
        .engine
        .transfers()
        .send_transfer(
            &TransferRequest {
                wallet_id: wallet.id,
                chain: ChainId::Ethereum,
                asset_id: asset.id,
                to_address: "0x0000000000000000000000000000000000000000".to_string(),
                amount: "1".to_string(),
                block_if_energy_insufficient: false,
            },
            "master-password",
        )
        .unwrap();
    let activity = fixture.engine.activity().list_activity(wallet.id).unwrap();

    assert_eq!(result.chain, ChainId::Ethereum);
    assert_eq!(result.tx_hash, "0xbroadcasted");
    assert_eq!(result.status, "broadcasted");
    assert_eq!(activity.len(), 1);
    assert_eq!(activity[0].tx_hash, result.tx_hash);
    assert_eq!(activity[0].status, crate::models::ActivityStatus::Pending);
    assert_eq!(activity[0].summary, "Sent 1 ETH");
    assert_eq!(
        rpc.methods(),
        vec![
            "eth_chainId",
            "eth_getTransactionCount",
            "eth_estimateGas",
            "eth_maxPriorityFeePerGas",
            "eth_getBlockByNumber",
            "eth_getBalance",
            "eth_sendRawTransaction"
        ]
    );
    let send_request = rpc.request_for_method("eth_sendRawTransaction");
    let raw_tx = send_request["params"][0].as_str().unwrap();
    assert!(raw_tx.starts_with("0x02"));
    assert!(raw_tx.len() > 100);
}
