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
fn send_transfer_rejects_evm_rpc_chain_id_mismatch_before_broadcast() {
    let rpc = TestRpcServer::evm_with_chain_id(
        "0x1",
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": "0xbroadcasted"
        }),
    );
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
        .update_chain_rpc(ChainId::Bsc, &rpc.url)
        .unwrap();
    let asset = fixture
        .engine
        .assets()
        .list_assets(wallet.id)
        .unwrap()
        .into_iter()
        .find(|asset: &Asset| asset.chain == ChainId::Bsc)
        .unwrap();
    let error = fixture
        .engine
        .transfers()
        .send_transfer(
            &TransferRequest {
                wallet_id: wallet.id,
                chain: ChainId::Bsc,
                asset_id: asset.id,
                to_address: "0x0000000000000000000000000000000000000000".to_string(),
                amount: "1".to_string(),
            },
            "master-password",
        )
        .unwrap_err();

    assert_eq!(error, WalletError::InvalidNetworkSettings);
    assert_eq!(rpc.methods(), vec!["eth_chainId"]);
}

#[test]
fn send_transfer_rejects_zero_amount_before_network_request() {
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
    let error = fixture
        .engine
        .transfers()
        .send_transfer(
            &TransferRequest {
                wallet_id: wallet.id,
                chain: ChainId::Ethereum,
                asset_id: asset.id,
                to_address: "0x0000000000000000000000000000000000000000".to_string(),
                amount: "0".to_string(),
            },
            "master-password",
        )
        .unwrap_err();

    assert_eq!(error, WalletError::InsufficientFunds);
    assert!(rpc.methods().is_empty());
}

#[test]
fn send_transfer_does_not_record_activity_when_broadcast_fails() {
    let rpc = TestRpcServer::evm(serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "error": {"code": -32000, "message": "insufficient funds"}
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
    let error = fixture
        .engine
        .transfers()
        .send_transfer(
            &TransferRequest {
                wallet_id: wallet.id,
                chain: ChainId::Ethereum,
                asset_id: asset.id,
                to_address: "0x0000000000000000000000000000000000000000".to_string(),
                amount: "1".to_string(),
            },
            "master-password",
        )
        .unwrap_err();
    let activity = fixture.engine.activity().list_activity(wallet.id).unwrap();

    assert_eq!(error, WalletError::NetworkUnavailable);
    assert!(activity.is_empty());
}

#[test]
fn send_transfer_rejects_tron_native_insufficient_balance_before_broadcast() {
    let rpc = TestRpcServer::tron_with_balances(0, "0");
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
    let error = fixture
        .engine
        .transfers()
        .send_transfer(
            &TransferRequest {
                wallet_id: wallet.id,
                chain: ChainId::Tron,
                asset_id: asset.id,
                to_address: "TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7".to_string(),
                amount: "1".to_string(),
            },
            "master-password",
        )
        .unwrap_err();

    assert_eq!(error, WalletError::InsufficientFunds);
    assert_eq!(rpc.paths(), vec!["/wallet/getaccount"]);
}

#[test]
fn send_transfer_rejects_trc20_insufficient_balance_before_broadcast() {
    let rpc = TestRpcServer::tron_with_balances(10_000_000, "0");
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
    let error = fixture
        .engine
        .transfers()
        .send_transfer(
            &TransferRequest {
                wallet_id: wallet.id,
                chain: ChainId::Tron,
                asset_id: token.id,
                to_address: "TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7".to_string(),
                amount: "2.5".to_string(),
            },
            "master-password",
        )
        .unwrap_err();

    assert_eq!(error, WalletError::InsufficientFunds);
    assert_eq!(
        rpc.paths(),
        vec!["/wallet/getaccount", "/wallet/triggerconstantcontract"]
    );
}
