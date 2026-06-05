use serde::Serialize;
use wallet_core::error::WalletError;

#[derive(Debug, Serialize)]
pub struct WalletResponse {
    pub ok: bool,
    pub body_json: String,
    pub error: Option<String>,
}

pub(crate) fn ok_json<T: Serialize>(body: T) -> WalletResponse {
    match serde_json::to_string(&body) {
        Ok(body_json) => WalletResponse {
            ok: true,
            body_json,
            error: None,
        },
        Err(_) => error_response("Serialization failed"),
    }
}

pub(crate) fn wallet_error_response(error: WalletError) -> WalletResponse {
    error_response(error.safe_message())
}

pub(crate) fn error_response(message: &str) -> WalletResponse {
    WalletResponse {
        ok: false,
        body_json: "{}".to_string(),
        error: Some(message.to_string()),
    }
}
