pub mod assets;
pub mod discovery;
pub mod indexers;
pub mod network;
pub mod qr;
pub mod rpc;
pub mod transactions;
pub mod transfers;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const PROTOCOL_VERSION: u16 = 2;

#[derive(Debug, Clone, Deserialize)]
pub struct ProtocolRequest {
    pub protocol_version: u16,
    pub request_id: String,
    pub domain: String,
    pub action: String,
    #[serde(default)]
    pub payload: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProtocolResponse {
    pub request_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ProtocolError>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProtocolError {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

impl ProtocolResponse {
    pub fn ok(request_id: impl Into<String>, data: Value) -> Self {
        Self {
            request_id: request_id.into(),
            ok: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(
        request_id: impl Into<String>,
        code: impl Into<String>,
        message: impl Into<String>,
        retryable: bool,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            ok: false,
            data: None,
            error: Some(ProtocolError {
                code: code.into(),
                message: message.into(),
                retryable,
            }),
        }
    }
}
