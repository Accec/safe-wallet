use serde_json::json;
use wallet_core::models::NetworkPrivacySettings;

use super::support::{
    engine_for_path, parse_chain, parse_chain_id, parse_proxy_mode, test_proxy_connection,
};
use crate::response::{ok_json, wallet_error_response, WalletResponse};

pub(super) fn get_privacy_settings(db_path: String) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match engine
        .initialize()
        .and_then(|_| engine.network().privacy_settings())
    {
        Ok(settings) => ok_json(settings),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn save_privacy_settings(
    db_path: String,
    proxy_enabled: bool,
    proxy_mode: String,
    proxy_url: Option<String>,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_proxy_mode(&proxy_mode).and_then(|proxy_mode| {
        engine.initialize().and_then(|_| {
            engine
                .network()
                .save_privacy_settings(NetworkPrivacySettings {
                    proxy_enabled,
                    proxy_mode,
                    proxy_url,
                })
        })
    }) {
        Ok(()) => ok_json(json!({ "saved": true })),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn test_proxy(
    db_path: String,
    proxy_enabled: bool,
    proxy_mode: String,
    proxy_url: Option<String>,
) -> WalletResponse {
    let _ = db_path;
    match parse_proxy_mode(&proxy_mode).and_then(|proxy_mode| {
        test_proxy_connection(NetworkPrivacySettings {
            proxy_enabled,
            proxy_mode,
            proxy_url,
        })
    }) {
        Ok(()) => ok_json(json!({ "ok": true })),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn list_settings(db_path: String) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match engine
        .initialize()
        .and_then(|_| engine.network().list_settings())
    {
        Ok(settings) => ok_json(settings),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn save_settings(
    db_path: String,
    network_name: String,
    rpc_url: String,
    chain_id: String,
    currency_symbol: String,
    block_explorer_url: Option<String>,
    indexer_endpoint: Option<String>,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_chain_id(&chain_id).and_then(|chain| {
        engine.initialize().and_then(|_| {
            engine.network().save_settings(
                chain,
                &network_name,
                &chain_id,
                &rpc_url,
                &currency_symbol,
                block_explorer_url.as_deref(),
                indexer_endpoint.as_deref(),
            )
        })
    }) {
        Ok(()) => ok_json(json!({ "saved": true })),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn update_chain_rpc(db_path: String, chain: String, rpc_url: String) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_chain(&chain).and_then(|chain| {
        engine
            .initialize()
            .and_then(|_| engine.network().update_chain_rpc(chain, &rpc_url))
    }) {
        Ok(()) => ok_json(json!({})),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn update_indexer_settings(
    db_path: String,
    chain: String,
    endpoint: String,
    api_key: Option<String>,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_chain(&chain).and_then(|chain| {
        engine.initialize().and_then(|_| {
            engine
                .network()
                .update_indexer_settings(chain, &endpoint, api_key.as_deref())
        })
    }) {
        Ok(()) => ok_json(json!({})),
        Err(error) => wallet_error_response(error),
    }
}
