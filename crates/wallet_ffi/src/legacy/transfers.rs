use wallet_core::models::TransferRequest;
use wallet_core::qr;

use super::support::engine_for_path;
use crate::response::{error_response, ok_json, wallet_error_response, WalletResponse};

pub(super) fn parse_payment_uri(payload: String) -> WalletResponse {
    match qr::parse_payment_uri(&payload) {
        Ok(parsed) => ok_json(parsed),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn preview_transfer(db_path: String, request_json: String) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match serde_json::from_str::<TransferRequest>(&request_json) {
        Ok(request) => match engine
            .initialize()
            .and_then(|_| engine.transfers().preview_transfer(&request))
        {
            Ok(preview) => ok_json(preview),
            Err(error) => wallet_error_response(error),
        },
        Err(_) => error_response("Invalid transfer request"),
    }
}

pub(super) fn send_transfer(
    db_path: String,
    request_json: String,
    password: String,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match serde_json::from_str::<TransferRequest>(&request_json) {
        Ok(request) => match engine
            .initialize()
            .and_then(|_| engine.transfers().send_transfer(&request, &password))
        {
            Ok(result) => ok_json(result),
            Err(error) => wallet_error_response(error),
        },
        Err(_) => error_response("Invalid transfer request"),
    }
}
