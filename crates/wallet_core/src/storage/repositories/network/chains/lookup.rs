use super::super::super::super::mappers::*;
use super::super::super::NetworkRepository;
use crate::error::WalletError;
use crate::models::{ChainId, ChainSettings};
use rusqlite::{params, OptionalExtension};

impl NetworkRepository {
    pub fn chain_settings(&self, chain: ChainId) -> Result<ChainSettings, WalletError> {
        let connection = self.database.connect()?;
        connection
            .query_row(
                "select chain_settings.network_name, chain_settings.chain_id,
                    chain_settings.enabled, chain_settings.default_rpc_url,
                    chain_settings.user_rpc_url, custom_indexer.endpoint,
                    chain_settings.explorer_url, chain_settings.native_symbol,
                    chain_settings.native_decimals
                from chain_settings
                left join indexer_settings custom_indexer
                    on custom_indexer.chain = chain_settings.chain
                    and custom_indexer.provider = 'custom'
                    and custom_indexer.enabled = 1
                where chain_settings.chain = ?1",
                params![chain_to_db(chain)],
                |row| {
                    Ok(ChainSettings {
                        chain,
                        network_name: row.get(0)?,
                        chain_id: row.get(1)?,
                        enabled: row.get::<_, i64>(2)? == 1,
                        default_rpc_url: row.get(3)?,
                        user_rpc_url: row.get(4)?,
                        indexer_endpoint: row.get(5)?,
                        explorer_url: row.get(6)?,
                        native_symbol: row.get(7)?,
                        native_decimals: row.get::<_, i64>(8)? as u8,
                    })
                },
            )
            .optional()
            .map_err(|_| WalletError::Storage)?
            .ok_or(WalletError::Storage)
    }
}
