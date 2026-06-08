#![allow(unused_imports)]

use super::super::WalletEngine;
use super::support::*;
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
fn sync_activity_persists_indexed_transfer_history_without_duplicates() {
    use crate::models::{ActivityKind, ActivityStatus};
    use crate::protocol::indexers::ActivityIndexer;
    use std::cell::RefCell;

    struct FakeActivityIndexer {
        calls: RefCell<Vec<(ChainId, String, String)>>,
    }

    impl ActivityIndexer for FakeActivityIndexer {
        fn fetch_activity(
            &self,
            chain: ChainId,
            endpoint: &str,
            address: &str,
        ) -> Result<Vec<crate::models::ActivityRecord>, WalletError> {
            self.calls
                .borrow_mut()
                .push((chain, endpoint.to_string(), address.to_string()));
            Ok(vec![crate::models::ActivityRecord {
                chain,
                tx_hash: "0xhistorical".to_string(),
                kind: ActivityKind::TokenTransfer,
                status: ActivityStatus::Confirmed,
                summary: "Received 12.5 USDT".to_string(),
            }])
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
        .update_indexer_settings(ChainId::Ethereum, "https://example.invalid/indexer", None)
        .unwrap();
    let indexer = FakeActivityIndexer {
        calls: RefCell::new(Vec::new()),
    };

    let first_count = fixture
        .engine
        .activity()
        .sync_activity_with(wallet.id, Some(ChainId::Ethereum), &indexer)
        .unwrap();
    let second_count = fixture
        .engine
        .activity()
        .sync_activity_with(wallet.id, Some(ChainId::Ethereum), &indexer)
        .unwrap();
    let activity = fixture.engine.activity().list_activity(wallet.id).unwrap();
    let calls = indexer.calls.borrow();

    assert_eq!(first_count, 1);
    assert_eq!(second_count, 1);
    assert_eq!(activity.len(), 1);
    assert_eq!(activity[0].tx_hash, "0xhistorical");
    assert_eq!(activity[0].summary, "Received 12.5 USDT");
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].0, ChainId::Ethereum);
    assert_eq!(calls[0].1, "https://example.invalid/indexer");
    assert!(calls[0].2.starts_with("0x"));
}

#[test]
fn sync_activity_uses_default_scan_endpoint_without_custom_indexer_settings() {
    use crate::models::{ActivityKind, ActivityStatus};
    use crate::protocol::indexers::ActivityIndexer;
    use std::cell::RefCell;

    struct FakeActivityIndexer {
        calls: RefCell<Vec<(ChainId, String, String)>>,
    }

    impl ActivityIndexer for FakeActivityIndexer {
        fn fetch_activity(
            &self,
            chain: ChainId,
            endpoint: &str,
            address: &str,
        ) -> Result<Vec<crate::models::ActivityRecord>, WalletError> {
            self.calls
                .borrow_mut()
                .push((chain, endpoint.to_string(), address.to_string()));
            Ok(vec![crate::models::ActivityRecord {
                chain,
                tx_hash: "0xscanpage".to_string(),
                kind: ActivityKind::TokenTransfer,
                status: ActivityStatus::Confirmed,
                summary: "Received 1 TOKEN".to_string(),
            }])
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
    let indexer = FakeActivityIndexer {
        calls: RefCell::new(Vec::new()),
    };

    let synced = fixture
        .engine
        .activity()
        .sync_activity_with(wallet.id, Some(ChainId::Bsc), &indexer)
        .unwrap();
    let calls = indexer.calls.borrow();

    assert_eq!(synced, 1);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, ChainId::Bsc);
    assert_eq!(calls[0].1, "https://bscscan.com");
    assert!(calls[0].2.starts_with("0x"));
}
