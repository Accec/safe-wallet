use super::super::super::super::mappers::*;
use super::super::super::NetworkRepository;
use crate::error::WalletError;
use crate::models::ChainId;
use chrono::Utc;
use rusqlite::params;

impl NetworkRepository {
    pub fn update_chain_rpc(&self, chain: ChainId, rpc_url: &str) -> Result<(), WalletError> {
        let connection = self.database.connect()?;
        let updated = connection
            .execute(
                "update chain_settings
                set user_rpc_url = ?1, updated_at = ?2
                where chain = ?3",
                params![rpc_url, Utc::now().to_rfc3339(), chain_to_db(chain)],
            )
            .map_err(|_| WalletError::Storage)?;
        if updated == 0 {
            return Err(WalletError::Storage);
        }
        Ok(())
    }

    pub fn save_network_settings(
        &self,
        chain: ChainId,
        network_name: &str,
        chain_id: &str,
        rpc_url: &str,
        native_symbol: &str,
        native_decimals: u8,
        explorer_url: Option<&str>,
    ) -> Result<(), WalletError> {
        let connection = self.database.connect()?;
        let now = Utc::now().to_rfc3339();
        let updated = connection
            .execute(
                "update chain_settings
                set network_name = ?1, chain_id = ?2, user_rpc_url = ?3,
                    explorer_url = ?4, native_symbol = ?5, native_decimals = ?6,
                    updated_at = ?7
                where chain = ?8",
                params![
                    network_name,
                    chain_id,
                    rpc_url,
                    explorer_url,
                    native_symbol,
                    native_decimals,
                    now,
                    chain_to_db(chain),
                ],
            )
            .map_err(|_| WalletError::Storage)?;
        if updated == 0 {
            connection
                .execute(
                    "insert into chain_settings (
                        chain, network_name, chain_id, enabled, default_rpc_url,
                        user_rpc_url, explorer_url, native_symbol, native_decimals,
                        updated_at
                    ) values (?1, ?2, ?3, 1, ?4, ?4, ?5, ?6, ?7, ?8)",
                    params![
                        chain_to_db(chain),
                        network_name,
                        chain_id,
                        rpc_url,
                        explorer_url,
                        native_symbol,
                        native_decimals,
                        now,
                    ],
                )
                .map_err(|_| WalletError::Storage)?;
        }
        Ok(())
    }
}
