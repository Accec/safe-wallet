use super::super::super::mappers::*;
use super::super::AssetRepository;
use crate::error::WalletError;
use crate::models::{Asset, DiscoveredAsset};
use chrono::Utc;
use rusqlite::params;
use uuid::Uuid;

impl AssetRepository {
    pub fn save_discovered_token(
        &self,
        wallet_id: Uuid,
        discovered: &DiscoveredAsset,
    ) -> Result<Asset, WalletError> {
        if !self.database.wallets().wallet_exists(wallet_id)? {
            return Err(WalletError::WalletNotFound);
        }
        let mut connection = self.database.connect()?;
        let transaction = connection.transaction().map_err(|_| WalletError::Storage)?;
        let now = Utc::now().to_rfc3339();
        let updated = transaction
            .execute(
                "update tokens
                set kind = ?1, symbol = ?2, name = ?3, decimals = ?4,
                    source = case when source = 'user' then source else 'auto_discovered' end,
                    visible = case when visible = 0 then 0 else 1 end,
                    updated_at = ?5
                where chain = ?6 and contract_address = ?7",
                params![
                    asset_kind_to_db(discovered.kind),
                    discovered.symbol,
                    discovered.name,
                    discovered.decimals,
                    now,
                    chain_to_db(discovered.chain),
                    discovered.contract_address,
                ],
            )
            .map_err(|_| WalletError::Storage)?;
        if updated == 0 {
            transaction
                .execute(
                    "insert into tokens (
                        id, chain, contract_address, kind, symbol, name, decimals,
                        source, visible, updated_at
                    ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'auto_discovered', 1, ?8)",
                    params![
                        Uuid::new_v4().to_string(),
                        chain_to_db(discovered.chain),
                        discovered.contract_address,
                        asset_kind_to_db(discovered.kind),
                        discovered.symbol,
                        discovered.name,
                        discovered.decimals,
                        now,
                    ],
                )
                .map_err(|_| WalletError::Storage)?;
        }
        let asset = transaction
            .query_row(
                "select id, chain, kind, symbol, name, decimals, contract_address, visible,
                    '0' as balance
                from tokens where chain = ?1 and contract_address = ?2",
                params![chain_to_db(discovered.chain), discovered.contract_address],
                asset_from_row,
            )
            .map_err(|_| WalletError::Storage)?;
        let account_id = self.database.wallets().account_id_for_chain(
            &transaction,
            wallet_id,
            discovered.chain,
        )?;
        transaction
            .execute(
                "insert into asset_balances (
                    id, wallet_id, account_id, asset_id, balance, source, refreshed_at
                ) values (?1, ?2, ?3, ?4, ?5, 'discovery', ?6)
                on conflict(wallet_id, account_id, asset_id)
                do update set
                    balance = excluded.balance,
                    source = excluded.source,
                    refreshed_at = excluded.refreshed_at",
                params![
                    Uuid::new_v4().to_string(),
                    wallet_id.to_string(),
                    account_id,
                    asset.id.to_string(),
                    discovered.balance,
                    now,
                ],
            )
            .map_err(|_| WalletError::Storage)?;
        transaction.commit().map_err(|_| WalletError::Storage)?;
        Ok(asset)
    }
}
