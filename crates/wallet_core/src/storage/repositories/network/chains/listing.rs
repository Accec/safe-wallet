use super::super::super::super::mappers::*;
use super::super::super::NetworkRepository;
use crate::error::WalletError;
use crate::models::ChainSettings;

impl NetworkRepository {
    pub fn list_network_settings(&self) -> Result<Vec<ChainSettings>, WalletError> {
        let connection = self.database.connect()?;
        let mut statement = connection
            .prepare(
                "select chain_settings.chain, chain_settings.network_name,
                    chain_settings.chain_id, chain_settings.enabled,
                    chain_settings.default_rpc_url, chain_settings.user_rpc_url,
                    custom_indexer.endpoint, chain_settings.explorer_url,
                    chain_settings.native_symbol, chain_settings.native_decimals
                from chain_settings
                left join indexer_settings custom_indexer
                    on custom_indexer.chain = chain_settings.chain
                    and custom_indexer.provider = 'custom'
                    and custom_indexer.enabled = 1
                order by chain_settings.chain asc",
            )
            .map_err(|_| WalletError::Storage)?;
        let settings = statement
            .query_map([], chain_settings_from_row)
            .map_err(|_| WalletError::Storage)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| WalletError::Storage)?;
        Ok(settings)
    }
}
