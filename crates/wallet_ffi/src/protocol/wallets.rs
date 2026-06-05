use serde_json::{json, Value};
use wallet_core::domains::wallets as wallet_domain;

use super::error::CommandResult;
use super::payloads::{
    payload_as, DbPayload, KeystoreWalletPayload, MnemonicWalletPayload, PrivateKeyWalletPayload,
    WalletIdPayload, WalletPasswordPayload,
};
use super::support::{initialized_engine, json_data};

pub(super) fn generate_mnemonic() -> CommandResult {
    Ok(json!({ "mnemonic": wallet_domain::generate_mnemonic()? }))
}

pub(super) fn create_wallet(payload: &Value) -> CommandResult {
    let payload: MnemonicWalletPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(engine.wallets().create_wallet(
        &payload.label,
        &payload.mnemonic,
        &payload.password,
    )?)
}

pub(super) fn import_private_key(payload: &Value) -> CommandResult {
    let payload: PrivateKeyWalletPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(engine.wallets().import_private_key(
        &payload.label,
        &payload.private_key,
        &payload.password,
    )?)
}

pub(super) fn import_keystore(payload: &Value) -> CommandResult {
    let payload: KeystoreWalletPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(engine.wallets().import_keystore(
        &payload.label,
        &payload.keystore_json,
        &payload.keystore_password,
        &payload.password,
    )?)
}

pub(super) fn export_keystore(payload: &Value) -> CommandResult {
    let payload: WalletPasswordPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(
        engine
            .wallets()
            .export_keystore(payload.wallet_id, &payload.password)?,
    )
}

pub(super) fn delete_wallet(payload: &Value) -> CommandResult {
    let payload: WalletPasswordPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    engine
        .wallets()
        .delete_wallet(payload.wallet_id, &payload.password)?;
    Ok(json!({ "deleted": true }))
}

pub(super) fn list_wallets(payload: &Value) -> CommandResult {
    let payload: DbPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(engine.wallets().list_wallets()?)
}

pub(super) fn list_accounts(payload: &Value) -> CommandResult {
    let payload: WalletIdPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(engine.wallets().list_accounts(payload.wallet_id)?)
}
