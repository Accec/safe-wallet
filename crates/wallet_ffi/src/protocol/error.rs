use serde_json::Value;
use wallet_core::error::WalletError;

pub(super) type CommandResult = Result<Value, CommandError>;

#[derive(Debug)]
pub(super) struct CommandError {
    pub(super) code: &'static str,
    pub(super) message: String,
    pub(super) retryable: bool,
}

impl CommandError {
    pub(super) fn new(code: &'static str, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            code,
            message: message.into(),
            retryable,
        }
    }

    pub(super) fn invalid_payload() -> Self {
        Self::new("invalid_payload", "Invalid payload", false)
    }
}

impl From<WalletError> for CommandError {
    fn from(value: WalletError) -> Self {
        Self::new("wallet_error", value.safe_message(), false)
    }
}
