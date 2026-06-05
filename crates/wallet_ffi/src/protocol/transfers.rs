use serde_json::Value;
use wallet_core::qr;

use super::error::CommandResult;
use super::payloads::{
    payload_as, transfer_request_from_json, PaymentUriPayload, SendTransferPayload, TransferPayload,
};
use super::support::{initialized_engine, json_data};

pub(super) fn parse_payment_uri(payload: &Value) -> CommandResult {
    let payload: PaymentUriPayload = payload_as(payload)?;
    json_data(qr::parse_payment_uri(&payload.payload)?)
}

pub(super) fn preview_transfer(payload: &Value) -> CommandResult {
    let payload: TransferPayload = payload_as(payload)?;
    let request = transfer_request_from_json(&payload.request_json)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(engine.transfers().preview_transfer(&request)?)
}

pub(super) fn send_transfer(payload: &Value) -> CommandResult {
    let payload: SendTransferPayload = payload_as(payload)?;
    let request = transfer_request_from_json(&payload.request_json)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(
        engine
            .transfers()
            .send_transfer(&request, &payload.password)?,
    )
}
