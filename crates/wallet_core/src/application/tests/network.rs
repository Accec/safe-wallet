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
fn tor_network_privacy_settings_default_to_local_socks_proxy() {
    let fixture = engine_fixture();

    fixture
        .engine
        .network()
        .save_privacy_settings(crate::models::NetworkPrivacySettings {
            proxy_enabled: true,
            proxy_mode: crate::models::ProxyMode::Tor,
            proxy_url: None,
        })
        .unwrap();

    let settings = fixture.engine.network().privacy_settings().unwrap();
    assert!(settings.proxy_enabled);
    assert_eq!(settings.proxy_mode, crate::models::ProxyMode::Tor);
    assert_eq!(
        settings.proxy_url.as_deref(),
        Some(crate::protocol::network::DEFAULT_TOR_PROXY_URL)
    );
}

#[test]
fn network_settings_can_be_listed_for_editing() {
    let fixture = engine_fixture();

    fixture
        .engine
        .network()
        .save_settings(
            ChainId::Bsc,
            "BNB Smart Chain",
            "56",
            "https://example.invalid/rpc",
            "BNB",
            Some("https://example.invalid/explorer"),
            Some("https://example.invalid/indexer"),
        )
        .unwrap();
    let settings = fixture.engine.network().list_settings().unwrap();
    let bsc = settings
        .iter()
        .find(|settings| settings.chain == ChainId::Bsc)
        .unwrap();

    assert_eq!(settings.len(), 7);
    assert_eq!(bsc.network_name, "BNB Smart Chain");
    assert_eq!(bsc.chain_id.as_deref(), Some("56"));
    assert_eq!(
        bsc.user_rpc_url.as_deref(),
        Some("https://example.invalid/rpc")
    );
}

#[test]
fn network_settings_update_rpc_symbol_and_explorer() {
    let fixture = engine_fixture();

    fixture
        .engine
        .network()
        .save_settings(
            ChainId::Bsc,
            "BNB Smart Chain",
            "56",
            "https://example.invalid/rpc",
            "BNB",
            Some("https://example.invalid/explorer"),
            None,
        )
        .unwrap();
    let settings = fixture
        .engine
        .database
        .network()
        .chain_settings(ChainId::Bsc)
        .unwrap();

    assert_eq!(settings.network_name, "BNB Smart Chain");
    assert_eq!(settings.chain_id.as_deref(), Some("56"));
    assert_eq!(
        settings.user_rpc_url.as_deref(),
        Some("https://example.invalid/rpc")
    );
    assert_eq!(
        settings.explorer_url.as_deref(),
        Some("https://example.invalid/explorer")
    );
    assert_eq!(settings.native_symbol, "BNB");
}
