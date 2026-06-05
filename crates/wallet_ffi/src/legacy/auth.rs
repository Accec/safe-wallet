use wallet_core::error::WalletError;

use super::support::{engine_for_path, run_wallet_result};
use crate::response::{wallet_error_response, WalletResponse};

pub(super) fn set_master_password(db_path: String, password: String) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    run_wallet_result(|| {
        engine.initialize()?;
        engine.auth().set_master_password(&password)
    })
}

pub(super) fn unlock_app(db_path: String, password: String) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    run_wallet_result(|| {
        engine.initialize()?;
        engine.auth().unlock_app(&password)
    })
}

pub(super) fn set_duress_password(
    db_path: String,
    master_password: String,
    duress_password: String,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    run_wallet_result(|| {
        engine.initialize()?;
        engine
            .auth()
            .set_duress_password(&master_password, &duress_password)
    })
}

pub(super) fn update_biometric_unlock(
    db_path: String,
    password: String,
    enabled: bool,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    run_wallet_result(|| {
        engine.initialize()?;
        engine.auth().set_biometric_unlock(&password, enabled)
    })
}

pub(super) fn lock_app(db_path: String) -> WalletResponse {
    let _ = db_path;
    wallet_error_response(WalletError::Locked)
}
