use serde_json::json;

use super::support::{engine_for_path, parse_chain, parse_wallet_id};
use crate::response::{ok_json, wallet_error_response, WalletResponse};

pub(super) fn list_activity(db_path: String, wallet_id: String) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_wallet_id(&wallet_id).and_then(|wallet_id| {
        engine
            .initialize()
            .and_then(|_| engine.activity().list_activity(wallet_id))
    }) {
        Ok(activity) => ok_json(activity),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn sync_activity(
    db_path: String,
    wallet_id: String,
    chain: Option<String>,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_wallet_id(&wallet_id).and_then(|wallet_id| {
        engine.initialize()?;
        let chain = chain.as_deref().map(parse_chain).transpose()?;
        engine.activity().sync_activity(wallet_id, chain)
    }) {
        Ok(synced) => ok_json(json!({ "synced": synced })),
        Err(error) => wallet_error_response(error),
    }
}
