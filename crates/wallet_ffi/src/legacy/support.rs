use serde_json::json;
use std::time::Duration;
use wallet_core::application::WalletEngine;
use wallet_core::error::WalletError;
use wallet_core::models::{ChainId, MultisigKind, NetworkPrivacySettings, ProxyMode};
use wallet_core::storage::WalletDatabase;

use crate::response::{ok_json, wallet_error_response, WalletResponse};

pub(super) fn engine_for_path(db_path: &str) -> WalletEngine {
    WalletEngine::new(WalletDatabase::new(db_path))
}

pub(super) fn run_wallet_result<F>(operation: F) -> WalletResponse
where
    F: FnOnce() -> Result<(), WalletError>,
{
    match operation() {
        Ok(()) => ok_json(json!({})),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn parse_chain(chain: &str) -> Result<ChainId, WalletError> {
    match chain {
        "btc" => Ok(ChainId::Btc),
        "ethereum" => Ok(ChainId::Ethereum),
        "bsc" => Ok(ChainId::Bsc),
        "polygon" => Ok(ChainId::Polygon),
        "arbitrum" => Ok(ChainId::Arbitrum),
        "optimism" => Ok(ChainId::Optimism),
        "tron" => Ok(ChainId::Tron),
        _ => Err(WalletError::InvalidAddress),
    }
}

pub(super) fn parse_chain_id(chain_id: &str) -> Result<ChainId, WalletError> {
    match chain_id.trim() {
        "1" => Ok(ChainId::Ethereum),
        "10" => Ok(ChainId::Optimism),
        "56" => Ok(ChainId::Bsc),
        "137" => Ok(ChainId::Polygon),
        "42161" => Ok(ChainId::Arbitrum),
        "728126428" => Ok(ChainId::Tron),
        _ => Err(WalletError::InvalidNetworkSettings),
    }
}

pub(super) fn parse_proxy_mode(value: &str) -> Result<ProxyMode, WalletError> {
    match value {
        "custom" => Ok(ProxyMode::Custom),
        "tor" => Ok(ProxyMode::Tor),
        _ => Err(WalletError::InvalidProxySettings),
    }
}

pub(super) fn parse_multisig_kind(value: &str) -> Result<MultisigKind, WalletError> {
    match value {
        "evm_safe" => Ok(MultisigKind::EvmSafe),
        "tron_permission" => Ok(MultisigKind::TronPermission),
        _ => Err(WalletError::InvalidMultisigSettings),
    }
}

pub(super) fn test_proxy_connection(settings: NetworkPrivacySettings) -> Result<(), WalletError> {
    let client =
        wallet_core::protocol::network::build_http_client(&settings, Duration::from_secs(8))?;
    client
        .get("https://example.com")
        .send()
        .and_then(|response| response.error_for_status())
        .map_err(|_| WalletError::ProxyConnectionFailed)?;
    Ok(())
}

pub(super) fn parse_wallet_id(wallet_id: &str) -> Result<uuid::Uuid, WalletError> {
    parse_uuid(wallet_id, WalletError::WalletNotFound)
}

pub(super) fn parse_asset_id(asset_id: &str) -> Result<uuid::Uuid, WalletError> {
    parse_uuid(asset_id, WalletError::InvalidTokenContract)
}

pub(super) fn parse_uuid(value: &str, error: WalletError) -> Result<uuid::Uuid, WalletError> {
    uuid::Uuid::parse_str(value).map_err(|_| error)
}
