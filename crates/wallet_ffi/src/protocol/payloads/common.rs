use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;
use wallet_core::error::WalletError;
use wallet_core::models::ChainId;

use super::super::error::CommandError;

pub(in crate::protocol) fn payload_as<T: DeserializeOwned>(
    payload: &Value,
) -> Result<T, CommandError> {
    serde_json::from_value(payload.clone()).map_err(|_| CommandError::invalid_payload())
}

pub(in crate::protocol) fn chain_from_numeric_id(chain_id: &str) -> Result<ChainId, CommandError> {
    match chain_id.trim() {
        "1" => Ok(ChainId::Ethereum),
        "10" => Ok(ChainId::Optimism),
        "56" => Ok(ChainId::Bsc),
        "137" => Ok(ChainId::Polygon),
        "42161" => Ok(ChainId::Arbitrum),
        "728126428" => Ok(ChainId::Tron),
        _ => Err(WalletError::InvalidNetworkSettings.into()),
    }
}

#[derive(Deserialize)]
pub(in crate::protocol) struct DbPayload {
    pub(in crate::protocol) db_path: String,
}
