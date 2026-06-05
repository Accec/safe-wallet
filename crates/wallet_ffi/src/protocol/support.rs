use wallet_core::application::WalletEngine;
use wallet_core::storage::WalletDatabase;

use super::error::{CommandError, CommandResult};

pub(super) fn engine_for_path(db_path: &str) -> WalletEngine {
    WalletEngine::new(WalletDatabase::new(db_path))
}

pub(super) fn initialized_engine(db_path: &str) -> Result<WalletEngine, CommandError> {
    let engine = engine_for_path(db_path);
    engine.initialize()?;
    Ok(engine)
}

pub(super) fn json_data<T: serde::Serialize>(value: T) -> CommandResult {
    serde_json::to_value(value)
        .map_err(|_| CommandError::new("serialization_failed", "Serialization failed", false))
}
