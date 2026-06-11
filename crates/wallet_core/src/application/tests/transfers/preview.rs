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
fn transfer_preview_uses_wallet_account_asset_and_validates_address() {
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
        .find(|asset: &Asset| asset.chain == ChainId::Ethereum)
        .unwrap();
    let preview = fixture
        .engine
        .transfers()
        .preview_transfer(&TransferRequest {
            wallet_id: wallet.id,
            chain: ChainId::Ethereum,
            asset_id: asset.id,
            to_address: "0x0000000000000000000000000000000000000000".to_string(),
            amount: "1.25".to_string(),
            block_if_energy_insufficient: false,
        })
        .unwrap();

    assert_eq!(preview.chain, ChainId::Ethereum);
    assert_eq!(preview.asset_symbol, asset.symbol);
    assert_eq!(preview.amount, "1.25");
    assert_eq!(
        preview.to_address,
        "0x0000000000000000000000000000000000000000"
    );
    assert!(preview.from_address.starts_with("0x"));
    assert_eq!(preview.fee_estimate, "0.00042");

    let invalid = fixture
        .engine
        .transfers()
        .preview_transfer(&TransferRequest {
            wallet_id: wallet.id,
            chain: ChainId::Ethereum,
            asset_id: asset.id,
            to_address: "not-an-address".to_string(),
            amount: "1.25".to_string(),
            block_if_energy_insufficient: false,
        });
    assert_eq!(invalid.unwrap_err(), WalletError::InvalidAddress);
}

#[test]
fn transfer_preview_includes_tron_trc20_energy_status() {
    let rpc = TestRpcServer::tron_with_resources(
        42_012,
        "000000000000000000000000000000000000000000000001b1ae4d6e2ef50000",
        196_503,
        126_534,
        600,
        526,
        8_624,
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
    let preview = fixture
        .engine
        .transfers()
        .preview_transfer(&TransferRequest {
            wallet_id: wallet.id,
            chain: ChainId::Tron,
            asset_id: token.id,
            to_address: "TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7".to_string(),
            amount: "2.5".to_string(),
            block_if_energy_insufficient: true,
        })
        .unwrap();
    let status = preview.resource_status.unwrap();

    assert_eq!(status.energy_available, 69_969);
    assert_eq!(status.energy_required, 8_624);
    assert_eq!(status.bandwidth_available, 74);
    assert_eq!(status.trx_balance_sun, 42_012);
    assert_eq!(status.trx_fee_reserve_required_sun, 1_000_000);
    assert!(status.can_send_without_burning_trx);
    assert_eq!(
        rpc.paths(),
        vec![
            "/wallet/getaccount",
            "/wallet/getaccountresource",
            "/wallet/triggerconstantcontract"
        ]
    );
}
