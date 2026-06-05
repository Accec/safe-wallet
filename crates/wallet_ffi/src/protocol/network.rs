use serde_json::{json, Value};
use std::time::Duration;
use wallet_core::error::WalletError;

use super::error::CommandResult;
use super::payloads::{
    chain_from_numeric_id, payload_as, DbPayload, NetworkPrivacyPayload,
    SaveNetworkSettingsPayload, UpdateChainRpcPayload, UpdateIndexerPayload,
};
use super::support::{initialized_engine, json_data};

pub(super) fn get_privacy(payload: &Value) -> CommandResult {
    let payload: DbPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(engine.network().privacy_settings()?)
}

pub(super) fn save_privacy(payload: &Value) -> CommandResult {
    let payload: NetworkPrivacyPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    engine.network().save_privacy_settings(payload.settings)?;
    Ok(json!({ "saved": true }))
}

pub(super) fn test_proxy_connection(payload: &Value) -> CommandResult {
    let payload: NetworkPrivacyPayload = payload_as(payload)?;
    let client = wallet_core::protocol::network::build_http_client(
        &payload.settings,
        Duration::from_secs(8),
    )?;
    client
        .get("https://example.com")
        .send()
        .and_then(|response| response.error_for_status())
        .map_err(|_| WalletError::ProxyConnectionFailed)?;
    Ok(json!({ "ok": true }))
}

pub(super) fn list_settings(payload: &Value) -> CommandResult {
    let payload: DbPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(engine.network().list_settings()?)
}

pub(super) fn save_settings(payload: &Value) -> CommandResult {
    let payload: SaveNetworkSettingsPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    let chain = chain_from_numeric_id(&payload.chain_id)?;
    engine.network().save_settings(
        chain,
        &payload.network_name,
        &payload.chain_id,
        &payload.rpc_url,
        &payload.currency_symbol,
        payload.block_explorer_url.as_deref(),
        payload.indexer_endpoint.as_deref(),
    )?;
    Ok(json!({ "saved": true }))
}

pub(super) fn update_chain_rpc(payload: &Value) -> CommandResult {
    let payload: UpdateChainRpcPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    engine
        .network()
        .update_chain_rpc(payload.chain, &payload.rpc_url)?;
    Ok(json!({}))
}

pub(super) fn update_indexer_settings(payload: &Value) -> CommandResult {
    let payload: UpdateIndexerPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    engine.network().update_indexer_settings(
        payload.chain,
        &payload.endpoint,
        payload.api_key.as_deref(),
    )?;
    Ok(json!({}))
}
