use crate::error::WalletError;
use rusqlite::ErrorCode;

pub(crate) fn map_app_security_insert_error(error: rusqlite::Error) -> WalletError {
    match error {
        rusqlite::Error::SqliteFailure(sqlite_error, _)
            if sqlite_error.code == ErrorCode::ConstraintViolation =>
        {
            WalletError::MasterPasswordAlreadySet
        }
        _ => WalletError::Storage,
    }
}

pub(crate) fn map_multisig_signature_insert_error(error: rusqlite::Error) -> WalletError {
    match error {
        rusqlite::Error::SqliteFailure(sqlite_error, _)
            if sqlite_error.code == ErrorCode::ConstraintViolation =>
        {
            WalletError::DuplicateMultisigSignature
        }
        _ => WalletError::Storage,
    }
}

pub(crate) fn sql_conversion_error(error: WalletError) -> rusqlite::Error {
    rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error))
}
