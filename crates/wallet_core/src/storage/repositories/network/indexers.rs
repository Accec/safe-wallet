use super::super::super::mappers::*;
use super::super::NetworkRepository;
use crate::error::WalletError;
use crate::models::ChainId;
use chrono::Utc;
use rusqlite::{params, OptionalExtension};
use uuid::Uuid;

impl NetworkRepository {
    pub fn indexer_endpoint(&self, chain: ChainId) -> Result<Option<String>, WalletError> {
        let connection = self.database.connect()?;
        connection
            .query_row(
                "select endpoint
                from indexer_settings
                where chain = ?1 and provider = 'custom' and enabled = 1
                order by rowid desc
                limit 1",
                params![chain_to_db(chain)],
                |row| row.get(0),
            )
            .optional()
            .map_err(|_| WalletError::Storage)
    }

    pub fn update_indexer_settings(
        &self,
        chain: ChainId,
        endpoint: &str,
        api_key_configured: bool,
    ) -> Result<(), WalletError> {
        let connection = self.database.connect()?;
        let now = Utc::now().to_rfc3339();
        connection
            .execute(
                "delete from indexer_settings where chain = ?1 and provider = 'custom'",
                params![chain_to_db(chain)],
            )
            .map_err(|_| WalletError::Storage)?;
        connection
            .execute(
                "insert into indexer_settings (
                    id, chain, provider, endpoint, encrypted_api_key, enabled, updated_at
                ) values (?1, ?2, 'custom', ?3, ?4, 1, ?5)",
                params![
                    Uuid::new_v4().to_string(),
                    chain_to_db(chain),
                    endpoint,
                    if api_key_configured {
                        Some("configured")
                    } else {
                        None
                    },
                    now
                ],
            )
            .map_err(|_| WalletError::Storage)?;
        Ok(())
    }

    pub fn clear_indexer_settings(&self, chain: ChainId) -> Result<(), WalletError> {
        let connection = self.database.connect()?;
        connection
            .execute(
                "delete from indexer_settings where chain = ?1 and provider = 'custom'",
                params![chain_to_db(chain)],
            )
            .map_err(|_| WalletError::Storage)?;
        Ok(())
    }
}
