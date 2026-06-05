use super::super::super::mappers::*;
use super::super::AssetRepository;
use crate::error::WalletError;
use crate::models::{Asset, ChainId, TokenMetadata};
use chrono::Utc;
use rusqlite::{params, OptionalExtension};
use uuid::Uuid;

impl AssetRepository {
    pub fn save_custom_token(&self, metadata: &TokenMetadata) -> Result<Asset, WalletError> {
        let connection = self.database.connect()?;
        let now = Utc::now().to_rfc3339();
        let updated = connection
            .execute(
                "update tokens
                set kind = ?1, symbol = ?2, name = ?3, decimals = ?4,
                    source = 'user', visible = 1, updated_at = ?5
                where chain = ?6 and contract_address = ?7",
                params![
                    asset_kind_to_db(metadata.kind),
                    metadata.symbol,
                    metadata.name,
                    metadata.decimals,
                    now,
                    chain_to_db(metadata.chain),
                    metadata.contract_address,
                ],
            )
            .map_err(|_| WalletError::Storage)?;
        if updated == 0 {
            connection
                .execute(
                    "insert into tokens (
                        id, chain, contract_address, kind, symbol, name, decimals,
                        source, visible, updated_at
                    ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'user', 1, ?8)",
                    params![
                        Uuid::new_v4().to_string(),
                        chain_to_db(metadata.chain),
                        metadata.contract_address,
                        asset_kind_to_db(metadata.kind),
                        metadata.symbol,
                        metadata.name,
                        metadata.decimals,
                        now
                    ],
                )
                .map_err(|_| WalletError::Storage)?;
        }
        self.find_token(metadata.chain, Some(&metadata.contract_address))?
            .ok_or(WalletError::Storage)
    }

    pub fn remove_custom_token(&self, asset_id: Uuid) -> Result<(), WalletError> {
        let mut connection = self.database.connect()?;
        let transaction = connection.transaction().map_err(|_| WalletError::Storage)?;
        transaction
            .execute(
                "delete from asset_balances where asset_id = ?1",
                params![asset_id.to_string()],
            )
            .map_err(|_| WalletError::Storage)?;
        let updated = transaction
            .execute(
                "update tokens
                set visible = 0, updated_at = ?1
                where id = ?2 and contract_address is not null and kind in ('erc20', 'trc20')",
                params![Utc::now().to_rfc3339(), asset_id.to_string()],
            )
            .map_err(|_| WalletError::Storage)?;
        if updated == 0 {
            return Err(WalletError::InvalidTokenContract);
        }
        transaction.commit().map_err(|_| WalletError::Storage)?;
        Ok(())
    }

    fn find_token(
        &self,
        chain: ChainId,
        contract_address: Option<&str>,
    ) -> Result<Option<Asset>, WalletError> {
        let connection = self.database.connect()?;
        if let Some(contract_address) = contract_address {
            return connection
                .query_row(
                    "select id, chain, kind, symbol, name, decimals, contract_address, visible,
                    '0' as balance
                    from tokens where chain = ?1 and contract_address = ?2",
                    params![chain_to_db(chain), contract_address],
                    asset_from_row,
                )
                .optional()
                .map_err(|_| WalletError::Storage);
        }
        connection
            .query_row(
                "select id, chain, kind, symbol, name, decimals, contract_address, visible,
                '0' as balance
                from tokens where chain = ?1 and contract_address is null",
                params![chain_to_db(chain)],
                asset_from_row,
            )
            .optional()
            .map_err(|_| WalletError::Storage)
    }
}
