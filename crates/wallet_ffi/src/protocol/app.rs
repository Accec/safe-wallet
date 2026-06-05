use serde_json::{json, Value};

use super::error::CommandResult;
use super::payloads::{payload_as, AppStatusPayload};
use super::support::{engine_for_path, json_data};

pub(super) fn status(payload: &Value) -> CommandResult {
    let payload: AppStatusPayload = payload_as(payload)?;
    match payload.db_path {
        Some(db_path) => {
            let engine = engine_for_path(&db_path);
            json_data(engine.app_status()?)
        }
        None => Ok(json!({
            "initialized": false,
            "locked": true,
            "biometric_enabled": false
        })),
    }
}
