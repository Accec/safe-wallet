use serde_json::{json, Value};

use super::error::CommandResult;
use super::payloads::{payload_as, BiometricUnlockPayload, DuressPasswordPayload, PasswordPayload};
use super::support::initialized_engine;

pub(super) fn set_master_password(payload: &Value) -> CommandResult {
    let payload: PasswordPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    engine.auth().set_master_password(&payload.password)?;
    Ok(json!({}))
}

pub(super) fn unlock_app(payload: &Value) -> CommandResult {
    let payload: PasswordPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    engine.auth().unlock_app(&payload.password)?;
    Ok(json!({}))
}

pub(super) fn set_duress_password(payload: &Value) -> CommandResult {
    let payload: DuressPasswordPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    engine
        .auth()
        .set_duress_password(&payload.master_password, &payload.duress_password)?;
    Ok(json!({}))
}

pub(super) fn update_biometric_unlock(payload: &Value) -> CommandResult {
    let payload: BiometricUnlockPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    engine
        .auth()
        .set_biometric_unlock(&payload.password, payload.enabled)?;
    Ok(json!({}))
}
