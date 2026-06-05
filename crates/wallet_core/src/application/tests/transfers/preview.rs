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
        });
    assert_eq!(invalid.unwrap_err(), WalletError::InvalidAddress);
}
