use serde_json::json;
use wallet_core::application::WalletEngine;
use wallet_core::storage::WalletDatabase;

use crate::response::{ok_json, wallet_error_response, WalletResponse};

pub(super) fn status(db_path: Option<String>) -> WalletResponse {
    match db_path {
        Some(db_path) => {
            let engine = WalletEngine::new(WalletDatabase::new(&db_path));
            match engine.app_status() {
                Ok(status) => ok_json(status),
                Err(error) => wallet_error_response(error),
            }
        }
        None => ok_json(json!({
            "initialized": false,
            "locked": true,
            "biometric_enabled": false
        })),
    }
}
