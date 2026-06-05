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
fn refreshed_asset_balance_persists_to_sqlite_for_reopened_engine() {
    struct FakeBalanceClient;

    impl AssetBalanceClient for FakeBalanceClient {
        fn supports_chain(&self, chain: ChainId) -> bool {
            chain == ChainId::Ethereum
        }

        fn fetch_native_balance(
            &self,
            _chain: ChainId,
            _rpc_url: &str,
            _address: &str,
        ) -> Result<String, WalletError> {
            Ok("1.25".to_string())
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
    }

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
        .assets()
        .refresh_chain_balances_with(wallet.id, ChainId::Ethereum, &FakeBalanceClient)
        .unwrap();
    let reopened = reopened_engine(&fixture.db_path);
    let assets = reopened.assets().list_assets(wallet.id).unwrap();
    let eth = assets
        .iter()
        .find(|asset| asset.chain == ChainId::Ethereum && asset.kind == AssetKind::Native)
        .unwrap();
    let connection = rusqlite::Connection::open(&fixture.db_path).unwrap();
    let stored_balance: String = connection
        .query_row(
            "select asset_balances.balance
            from asset_balances
            join tokens on tokens.id = asset_balances.asset_id
            where asset_balances.wallet_id = ?1
                and tokens.chain = 'ethereum'
                and tokens.kind = 'native'",
            [wallet.id.to_string()],
            |row| row.get(0),
        )
        .unwrap();

    assert_eq!(eth.balance, "1.25");
    assert_eq!(stored_balance, "1.25");
}

#[test]
fn refresh_native_balances_uses_configured_rpc_url() {
    use std::cell::RefCell;

    struct FakeBalanceClient {
        calls: RefCell<Vec<(ChainId, String, String)>>,
    }

    impl AssetBalanceClient for FakeBalanceClient {
        fn supports_chain(&self, chain: ChainId) -> bool {
            chain == ChainId::Ethereum
        }

        fn fetch_native_balance(
            &self,
            chain: ChainId,
            rpc_url: &str,
            address: &str,
        ) -> Result<String, WalletError> {
            self.calls
                .borrow_mut()
                .push((chain, rpc_url.to_string(), address.to_string()));
            Ok("1.25".to_string())
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
    }

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
    let client = FakeBalanceClient {
        calls: RefCell::new(Vec::new()),
    };

    fixture
        .engine
        .assets()
        .refresh_native_balances_with(wallet.id, &client)
        .unwrap();
    let assets = fixture.engine.assets().list_assets(wallet.id).unwrap();
    let eth = assets
        .iter()
        .find(|asset| asset.chain == ChainId::Ethereum && asset.kind == AssetKind::Native)
        .unwrap();
    let calls = client.calls.borrow();

    assert_eq!(eth.balance, "1.25");
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, ChainId::Ethereum);
    assert_eq!(calls[0].1, "https://example.invalid/rpc");
    assert!(calls[0].2.starts_with("0x"));
}

#[test]
fn refresh_asset_balances_updates_erc20_with_configured_rpc_url() {
    use std::cell::RefCell;

    struct FakeBalanceClient {
        token_calls: RefCell<Vec<(ChainId, String, String, String, u8)>>,
    }

    impl AssetBalanceClient for FakeBalanceClient {
        fn supports_chain(&self, chain: ChainId) -> bool {
            chain == ChainId::Ethereum
        }

        fn fetch_native_balance(
            &self,
            _chain: ChainId,
            _rpc_url: &str,
            _address: &str,
        ) -> Result<String, WalletError> {
            Ok("1.25".to_string())
        }

        fn fetch_token_balance(
            &self,
            chain: ChainId,
            rpc_url: &str,
            owner_address: &str,
            contract_address: &str,
            decimals: u8,
        ) -> Result<String, WalletError> {
            self.token_calls.borrow_mut().push((
                chain,
                rpc_url.to_string(),
                owner_address.to_string(),
                contract_address.to_string(),
                decimals,
            ));
            Ok("12.34".to_string())
        }

        fn fetch_token_metadata(
            &self,
            chain: ChainId,
            _rpc_url: &str,
            contract_address: &str,
        ) -> Result<TokenMetadata, WalletError> {
            Ok(TokenMetadata {
                chain,
                kind: AssetKind::Erc20,
                contract_address: contract_address.to_string(),
                symbol: "USDT".to_string(),
                name: "Tether USD".to_string(),
                decimals: 18,
            })
        }
    }

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
    let client = FakeBalanceClient {
        token_calls: RefCell::new(Vec::new()),
    };
    let token = fixture
        .engine
        .assets()
        .add_custom_token_with(
            &client,
            ChainId::Ethereum,
            "0x0000000000000000000000000000000000000001",
            "USDT",
        )
        .unwrap();

    fixture
        .engine
        .assets()
        .refresh_native_balances_with(wallet.id, &client)
        .unwrap();
    let assets = fixture.engine.assets().list_assets(wallet.id).unwrap();
    let refreshed_token = assets.iter().find(|asset| asset.id == token.id).unwrap();
    let token_calls = client.token_calls.borrow();

    assert_eq!(refreshed_token.balance, "12.34");
    assert_eq!(token_calls.len(), 1);
    assert_eq!(token_calls[0].0, ChainId::Ethereum);
    assert_eq!(token_calls[0].1, "https://example.invalid/rpc");
    assert!(token_calls[0].2.starts_with("0x"));
    assert_eq!(
        token_calls[0].3,
        "0x0000000000000000000000000000000000000001"
    );
    assert_eq!(token_calls[0].4, 18);
}

#[test]
fn refresh_asset_balances_updates_tron_native_and_trc20_with_configured_rpc_url() {
    use std::cell::RefCell;

    struct FakeBalanceClient {
        native_calls: RefCell<Vec<(ChainId, String, String)>>,
        token_calls: RefCell<Vec<(ChainId, String, String, String, u8)>>,
    }

    impl AssetBalanceClient for FakeBalanceClient {
        fn supports_chain(&self, chain: ChainId) -> bool {
            chain == ChainId::Tron
        }

        fn fetch_native_balance(
            &self,
            chain: ChainId,
            rpc_url: &str,
            address: &str,
        ) -> Result<String, WalletError> {
            self.native_calls
                .borrow_mut()
                .push((chain, rpc_url.to_string(), address.to_string()));
            Ok("2.5".to_string())
        }

        fn fetch_token_balance(
            &self,
            chain: ChainId,
            rpc_url: &str,
            owner_address: &str,
            contract_address: &str,
            decimals: u8,
        ) -> Result<String, WalletError> {
            self.token_calls.borrow_mut().push((
                chain,
                rpc_url.to_string(),
                owner_address.to_string(),
                contract_address.to_string(),
                decimals,
            ));
            Ok("7.89".to_string())
        }
    }

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
        .save_settings(
            ChainId::Tron,
            "Tron",
            "728126428",
            "https://example.invalid/tron",
            "TRX",
            Some("https://tronscan.org"),
            None,
        )
        .unwrap();
    let token = fixture
        .engine
        .assets()
        .add_custom_token(ChainId::Tron, "TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7", "TRC")
        .unwrap();
    let client = FakeBalanceClient {
        native_calls: RefCell::new(Vec::new()),
        token_calls: RefCell::new(Vec::new()),
    };

    fixture
        .engine
        .assets()
        .refresh_native_balances_with(wallet.id, &client)
        .unwrap();
    let assets = fixture.engine.assets().list_assets(wallet.id).unwrap();
    let trx = assets
        .iter()
        .find(|asset| asset.chain == ChainId::Tron && asset.kind == AssetKind::Native)
        .unwrap();
    let trc = assets.iter().find(|asset| asset.id == token.id).unwrap();
    let native_calls = client.native_calls.borrow();
    let token_calls = client.token_calls.borrow();

    assert_eq!(trx.balance, "2.5");
    assert_eq!(trc.balance, "7.89");
    assert_eq!(native_calls.len(), 1);
    assert_eq!(token_calls.len(), 1);
    assert_eq!(native_calls[0].1, "https://example.invalid/tron");
    assert_eq!(token_calls[0].1, "https://example.invalid/tron");
    assert!(native_calls[0].2.starts_with('T'));
    assert!(token_calls[0].2.starts_with('T'));
    assert_eq!(token_calls[0].3, "TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7");
    assert_eq!(token_calls[0].4, 6);
}

#[test]
fn refresh_chain_balances_only_refreshes_requested_chain() {
    use std::cell::RefCell;

    struct FakeBalanceClient {
        calls: RefCell<Vec<ChainId>>,
    }

    impl AssetBalanceClient for FakeBalanceClient {
        fn supports_chain(&self, chain: ChainId) -> bool {
            chain != ChainId::Btc
        }

        fn fetch_native_balance(
            &self,
            chain: ChainId,
            _rpc_url: &str,
            _address: &str,
        ) -> Result<String, WalletError> {
            self.calls.borrow_mut().push(chain);
            Ok("3.21".to_string())
        }

        fn fetch_token_balance(
            &self,
            chain: ChainId,
            _rpc_url: &str,
            _owner_address: &str,
            _contract_address: &str,
            _decimals: u8,
        ) -> Result<String, WalletError> {
            self.calls.borrow_mut().push(chain);
            Ok("0".to_string())
        }
    }

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
    let client = FakeBalanceClient {
        calls: RefCell::new(Vec::new()),
    };

    fixture
        .engine
        .assets()
        .refresh_chain_balances_with(wallet.id, ChainId::Tron, &client)
        .unwrap();
    let calls = client.calls.borrow();

    assert_eq!(&*calls, &[ChainId::Tron]);
}
