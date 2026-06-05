use serde::Deserialize;
use wallet_core::models::TransferRequest;

use super::super::error::CommandError;

pub(in crate::protocol) fn transfer_request_from_json(
    value: &str,
) -> Result<TransferRequest, CommandError> {
    serde_json::from_str(value).map_err(|_| CommandError::invalid_payload())
}

#[derive(Deserialize)]
pub(in crate::protocol) struct PaymentUriPayload {
    pub(in crate::protocol) payload: String,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct TransferPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) request_json: String,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct SendTransferPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) request_json: String,
    pub(in crate::protocol) password: String,
}
