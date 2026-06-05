use serde_json::{json, Value};

use super::error::CommandResult;
use super::payloads::{
    payload_as, AddCustomTokenPayload, AssetIdPayload, WalletChainPayload, WalletIdPayload,
};
use super::support::{initialized_engine, json_data};

pub(super) fn list_assets(payload: &Value) -> CommandResult {
    let payload: WalletIdPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(engine.assets().list_assets(payload.wallet_id)?)
}

pub(super) fn refresh_assets(payload: &Value) -> CommandResult {
    let payload: WalletChainPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    engine
        .assets()
        .refresh_balances(payload.wallet_id, payload.chain)?;
    Ok(json!({ "refreshed": true }))
}

pub(super) fn discover_assets(payload: &Value) -> CommandResult {
    let payload: WalletChainPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    let assets = engine
        .assets()
        .discover_assets(payload.wallet_id, payload.chain)?;
    Ok(json!({ "discovered": assets.len() }))
}

pub(super) fn add_custom_token(payload: &Value) -> CommandResult {
    let payload: AddCustomTokenPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    let token_name = payload.token_name.unwrap_or_else(|| "TOKEN".to_string());
    json_data(engine.assets().add_custom_token(
        payload.chain,
        &payload.contract_address,
        &token_name,
    )?)
}

pub(super) fn remove_custom_token(payload: &Value) -> CommandResult {
    let payload: AssetIdPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    engine.assets().remove_custom_token(payload.asset_id)?;
    Ok(json!({ "removed": true }))
}
