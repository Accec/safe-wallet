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
fn add_custom_token_uses_rpc_metadata_for_evm_contracts() {
    use std::cell::RefCell;

    struct FakeTokenClient {
        calls: RefCell<Vec<(ChainId, String, String)>>,
    }

    impl AssetBalanceClient for FakeTokenClient {
        fn supports_chain(&self, chain: ChainId) -> bool {
            chain == ChainId::Ethereum
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
            Ok("0".to_string())
        }

        fn fetch_token_metadata(
            &self,
            chain: ChainId,
            rpc_url: &str,
            contract_address: &str,
        ) -> Result<TokenMetadata, WalletError> {
            self.calls.borrow_mut().push((
                chain,
                rpc_url.to_string(),
                contract_address.to_string(),
            ));
            Ok(TokenMetadata {
                chain,
                kind: AssetKind::Erc20,
                contract_address: contract_address.to_string(),
                symbol: "USDC".to_string(),
                name: "USD Coin".to_string(),
                decimals: 6,
            })
        }
    }

    let fixture = engine_fixture();
    fixture
        .engine
        .network()
        .save_settings(
            ChainId::Ethereum,
            "Ethereum",
            "1",
            "https://example.invalid/rpc",
            "ETH",
            Some("https://etherscan.io"),
            None,
        )
        .unwrap();
    let client = FakeTokenClient {
        calls: RefCell::new(Vec::new()),
    };

    let token = fixture
        .engine
        .assets()
        .add_custom_token_with(
            &client,
            ChainId::Ethereum,
            "0x0000000000000000000000000000000000000001",
            "My USDC",
        )
        .unwrap();
    let calls = client.calls.borrow();

    assert_eq!(token.symbol, "USDC");
    assert_eq!(token.name, "My USDC");
    assert_eq!(token.decimals, 6);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].1, "https://example.invalid/rpc");
}

#[test]
fn custom_token_is_saved_and_listed() {
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
    let client = static_token_client("USDT");
    let token = fixture
        .engine
        .assets()
        .add_custom_token_with(
            &client,
            ChainId::Ethereum,
            "0x0000000000000000000000000000000000000000",
            "USDT",
        )
        .unwrap();
    let assets = fixture.engine.assets().list_assets(wallet.id).unwrap();

    assert_eq!(token.symbol, "USDT");
    assert!(assets.iter().any(|asset| asset.id == token.id));
}

#[test]
fn custom_token_list_persists_to_sqlite_for_reopened_engine() {
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
    let client = static_token_client("USDT");
    let token = fixture
        .engine
        .assets()
        .add_custom_token_with(
            &client,
            ChainId::Ethereum,
            "0x0000000000000000000000000000000000000000",
            "USDT",
        )
        .unwrap();
    let reopened = reopened_engine(&fixture.db_path);
    let assets = reopened.assets().list_assets(wallet.id).unwrap();
    let persisted_token = assets.iter().find(|asset| asset.id == token.id).unwrap();
    let connection = rusqlite::Connection::open(&fixture.db_path).unwrap();
    let stored_symbol: String = connection
        .query_row(
            "select symbol from tokens where id = ?1",
            [token.id.to_string()],
            |row| row.get(0),
        )
        .unwrap();

    assert_eq!(persisted_token.symbol, "USDT");
    assert_eq!(
        persisted_token.contract_address.as_deref(),
        Some("0x0000000000000000000000000000000000000000")
    );
    assert_eq!(stored_symbol, "USDT");
}

#[test]
fn custom_token_can_be_removed_without_removing_native_assets() {
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
    let native_asset = fixture
        .engine
        .assets()
        .list_assets(wallet.id)
        .unwrap()
        .into_iter()
        .find(|asset| asset.chain == ChainId::Ethereum && asset.kind == AssetKind::Native)
        .unwrap();
    let client = static_token_client("USDT");
    let token = fixture
        .engine
        .assets()
        .add_custom_token_with(
            &client,
            ChainId::Ethereum,
            "0x0000000000000000000000000000000000000000",
            "USDT",
        )
        .unwrap();

    fixture
        .engine
        .assets()
        .remove_custom_token(token.id)
        .unwrap();
    let assets = fixture.engine.assets().list_assets(wallet.id).unwrap();

    assert!(assets.iter().all(|asset| asset.id != token.id));
    assert!(assets.iter().any(|asset| asset.id == native_asset.id));
    assert_eq!(
        fixture
            .engine
            .assets()
            .remove_custom_token(native_asset.id)
            .unwrap_err(),
        WalletError::InvalidTokenContract
    );
}

#[test]
fn invalid_custom_token_contract_reports_token_contract_error() {
    let fixture = engine_fixture();

    let error = fixture
        .engine
        .assets()
        .add_custom_token(ChainId::Ethereum, "not-a-contract", "USDT")
        .unwrap_err();

    assert_eq!(error, WalletError::InvalidTokenContract);
}
