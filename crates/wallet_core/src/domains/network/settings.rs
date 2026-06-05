use crate::chains;
use crate::error::WalletError;
use crate::models::{ChainId, ChainSettings};
use crate::storage::WalletDatabase;

use super::validation::is_http_url;

pub(super) fn list_settings(database: &WalletDatabase) -> Result<Vec<ChainSettings>, WalletError> {
    let stored_settings = database.network().list_network_settings()?;
    let mut settings = chains::default_chain_settings()
        .into_iter()
        .map(|default_setting| {
            stored_settings
                .iter()
                .find(|stored_setting| stored_setting.chain == default_setting.chain)
                .cloned()
                .unwrap_or(default_setting)
        })
        .collect::<Vec<_>>();
    for stored_setting in stored_settings {
        if !settings
            .iter()
            .any(|setting| setting.chain == stored_setting.chain)
        {
            settings.push(stored_setting);
        }
    }
    Ok(settings)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn save_settings(
    database: &WalletDatabase,
    chain: ChainId,
    network_name: &str,
    chain_id: &str,
    rpc_url: &str,
    currency_symbol: &str,
    block_explorer_url: Option<&str>,
    indexer_endpoint: Option<&str>,
) -> Result<(), WalletError> {
    if network_name.trim().is_empty()
        || chain_id.trim().is_empty()
        || currency_symbol.trim().is_empty()
        || !is_http_url(rpc_url)
        || block_explorer_url.is_some_and(|url| !is_http_url(url))
        || indexer_endpoint.is_some_and(|url| !url.trim().is_empty() && !is_http_url(url))
    {
        return Err(WalletError::InvalidNetworkSettings);
    }
    let native_decimals = chains::default_chain_settings()
        .into_iter()
        .find(|setting| setting.chain == chain)
        .map(|setting| setting.native_decimals)
        .ok_or(WalletError::InvalidNetworkSettings)?;
    database.network().save_network_settings(
        chain,
        network_name.trim(),
        chain_id.trim(),
        rpc_url.trim(),
        currency_symbol.trim(),
        native_decimals,
        block_explorer_url
            .map(str::trim)
            .filter(|url| !url.is_empty()),
    )?;
    let indexer_endpoint = indexer_endpoint
        .map(str::trim)
        .filter(|url| !url.is_empty());
    if let Some(indexer_endpoint) = indexer_endpoint {
        database
            .network()
            .update_indexer_settings(chain, indexer_endpoint, false)?;
    } else {
        database.network().clear_indexer_settings(chain)?;
    }
    Ok(())
}

pub(super) fn update_chain_rpc(
    database: &WalletDatabase,
    chain: ChainId,
    rpc_url: &str,
) -> Result<(), WalletError> {
    if !is_http_url(rpc_url) {
        return Err(WalletError::NetworkUnavailable);
    }
    database.network().update_chain_rpc(chain, rpc_url)
}

pub(super) fn update_indexer_settings(
    database: &WalletDatabase,
    chain: ChainId,
    endpoint: &str,
    api_key: Option<&str>,
) -> Result<(), WalletError> {
    if !is_http_url(endpoint) {
        return Err(WalletError::NetworkUnavailable);
    }
    database.network().update_indexer_settings(
        chain,
        endpoint,
        api_key.is_some_and(|key| !key.is_empty()),
    )
}
