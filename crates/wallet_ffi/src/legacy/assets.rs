use serde_json::json;

use super::support::{engine_for_path, parse_asset_id, parse_chain, parse_wallet_id};
use crate::response::{ok_json, wallet_error_response, WalletResponse};

pub(super) fn list_assets(db_path: String, wallet_id: String) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_wallet_id(&wallet_id).and_then(|wallet_id| {
        engine
            .initialize()
            .and_then(|_| engine.assets().list_assets(wallet_id))
    }) {
        Ok(assets) => ok_json(assets),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn refresh_assets(
    db_path: String,
    wallet_id: String,
    chain: Option<String>,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_wallet_id(&wallet_id).and_then(|wallet_id| {
        engine.initialize()?;
        match chain {
            Some(chain) => {
                let chain = parse_chain(&chain)?;
                engine.assets().refresh_chain_balances(wallet_id, chain)
            }
            None => engine.assets().refresh_native_balances(wallet_id),
        }
    }) {
        Ok(()) => ok_json(json!({ "refreshed": true })),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn discover_assets(
    db_path: String,
    wallet_id: String,
    chain: Option<String>,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_wallet_id(&wallet_id).and_then(|wallet_id| {
        engine.initialize()?;
        let chain = chain.as_deref().map(parse_chain).transpose()?;
        engine.assets().discover_assets(wallet_id, chain)
    }) {
        Ok(assets) => ok_json(json!({ "discovered": assets.len() })),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn add_custom_token(
    db_path: String,
    chain: String,
    contract_address: String,
    token_name: Option<String>,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    let token_name = token_name.unwrap_or_else(|| "TOKEN".to_string());
    match parse_chain(&chain).and_then(|chain| {
        engine.initialize().and_then(|_| {
            engine
                .assets()
                .add_custom_token(chain, &contract_address, &token_name)
        })
    }) {
        Ok(token) => ok_json(token),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn remove_custom_token(db_path: String, asset_id: String) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_asset_id(&asset_id).and_then(|asset_id| {
        engine
            .initialize()
            .and_then(|_| engine.assets().remove_custom_token(asset_id))
    }) {
        Ok(()) => ok_json(json!({ "removed": true })),
        Err(error) => wallet_error_response(error),
    }
}
