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
fn discovered_token_is_saved_with_balance_and_survives_reopen() {
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
        .database
        .assets()
        .save_discovered_token(
            wallet.id,
            &crate::models::DiscoveredAsset {
                chain: ChainId::Bsc,
                kind: AssetKind::Erc20,
                contract_address: "0x1111111111111111111111111111111111111111".to_string(),
                symbol: "POSI".to_string(),
                name: "Position".to_string(),
                decimals: 18,
                balance: "0.0001".to_string(),
            },
        )
        .unwrap();
    let reopened = reopened_engine(&fixture.db_path);

    let assets = reopened.assets().list_assets(wallet.id).unwrap();
    let discovered = assets.iter().find(|item| item.id == asset.id).unwrap();

    assert_eq!(discovered.symbol, "POSI");
    assert_eq!(discovered.balance, "0.0001");
}

#[test]
fn discovered_token_does_not_reveal_user_hidden_token() {
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
    let token = fixture
        .engine
        .assets()
        .add_custom_token_with(
            &static_token_client("POSI"),
            ChainId::Bsc,
            "0x1111111111111111111111111111111111111111",
            "Position",
        )
        .unwrap();

    fixture
        .engine
        .assets()
        .remove_custom_token(token.id)
        .unwrap();
    fixture
        .engine
        .database
        .assets()
        .save_discovered_token(
            wallet.id,
            &crate::models::DiscoveredAsset {
                chain: ChainId::Bsc,
                kind: AssetKind::Erc20,
                contract_address: "0x1111111111111111111111111111111111111111".to_string(),
                symbol: "POSI".to_string(),
                name: "Position".to_string(),
                decimals: 18,
                balance: "0.0001".to_string(),
            },
        )
        .unwrap();

    let assets = fixture.engine.assets().list_assets(wallet.id).unwrap();

    assert!(assets.iter().all(|asset| asset.id != token.id));
}

#[test]
fn discover_chain_assets_saves_indexed_tokens_and_refreshes_balances() {
    struct FakeDiscoveryProvider {
        seen_endpoint: Arc<Mutex<Option<String>>>,
    }

    impl crate::protocol::discovery::AssetDiscoveryProvider for FakeDiscoveryProvider {
        fn discover_assets(
            &self,
            chain: ChainId,
            endpoint: &str,
            _address: &str,
        ) -> Result<Vec<crate::models::DiscoveredAsset>, WalletError> {
            *self.seen_endpoint.lock().unwrap() = Some(endpoint.to_string());
            Ok(vec![crate::models::DiscoveredAsset {
                chain,
                kind: AssetKind::Erc20,
                contract_address: "0x2222222222222222222222222222222222222222".to_string(),
                symbol: "USDC".to_string(),
                name: "USD Coin".to_string(),
                decimals: 6,
                balance: "5".to_string(),
            }])
        }
    }

    struct FakeBalanceClient;

    impl AssetBalanceClient for FakeBalanceClient {
        fn supports_chain(&self, chain: ChainId) -> bool {
            chain == ChainId::Bsc
        }

        fn fetch_native_balance(
            &self,
            _chain: ChainId,
            _rpc_url: &str,
            _address: &str,
        ) -> Result<String, WalletError> {
            Ok("0".to_string())
        }

        fn fetch_token_balance(
            &self,
            _chain: ChainId,
            _rpc_url: &str,
            _owner_address: &str,
            _contract_address: &str,
            _decimals: u8,
        ) -> Result<String, WalletError> {
            Ok("7.25".to_string())
        }
    }

    let fixture = engine_fixture();
    let seen_endpoint = Arc::new(Mutex::new(None));
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
        .update_indexer_settings(
            ChainId::Bsc,
            "https://example.invalid/indexer?apikey=test",
            None,
        )
        .unwrap();

    let discovered = fixture
        .engine
        .assets()
        .discover_chain_assets_with(
            wallet.id,
            ChainId::Bsc,
            &FakeDiscoveryProvider {
                seen_endpoint: Arc::clone(&seen_endpoint),
            },
            &FakeBalanceClient,
        )
        .unwrap();
    let assets = fixture.engine.assets().list_assets(wallet.id).unwrap();
    let usdc = assets.iter().find(|asset| asset.symbol == "USDC").unwrap();

    assert_eq!(discovered.len(), 1);
    assert_eq!(
        seen_endpoint.lock().unwrap().as_deref(),
        Some("https://example.invalid/indexer?apikey=test")
    );
    assert_eq!(usdc.balance, "7.25");
}
