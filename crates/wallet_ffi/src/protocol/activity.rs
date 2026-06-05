use serde_json::{json, Value};

use super::error::CommandResult;
use super::payloads::{payload_as, WalletChainPayload, WalletIdPayload};
use super::support::{initialized_engine, json_data};

pub(super) fn list_activity(payload: &Value) -> CommandResult {
    let payload: WalletIdPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(engine.activity().list_activity(payload.wallet_id)?)
}

pub(super) fn sync_activity(payload: &Value) -> CommandResult {
    let payload: WalletChainPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    let synced = engine
        .activity()
        .sync_activity(payload.wallet_id, payload.chain)?;
    Ok(json!({ "synced": synced }))
}
