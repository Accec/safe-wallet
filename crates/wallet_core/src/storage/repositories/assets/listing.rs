use super::super::super::mappers::*;
use super::super::AssetRepository;
use crate::error::WalletError;
use crate::models::Asset;
use rusqlite::{params, OptionalExtension};
use uuid::Uuid;

impl AssetRepository {
    pub fn list_assets(&self, wallet_id: Uuid) -> Result<Vec<Asset>, WalletError> {
        if !self.database.wallets().wallet_exists(wallet_id)? {
            return Err(WalletError::WalletNotFound);
        }
        let connection = self.database.connect()?;
        let mut statement = connection
            .prepare(
                "select tokens.id, tokens.chain, tokens.kind, tokens.symbol, tokens.name,
                    tokens.decimals, tokens.contract_address, tokens.visible,
                    coalesce(asset_balances.balance, '0') as balance
                from tokens
                left join accounts
                    on accounts.wallet_id = ?1
                    and accounts.chain = tokens.chain
                    and accounts.account_index = 0
                left join asset_balances
                    on asset_balances.wallet_id = ?1
                    and asset_balances.account_id = accounts.id
                    and asset_balances.asset_id = tokens.id
                where tokens.visible = 1
                order by tokens.contract_address is not null, tokens.chain asc, tokens.symbol asc",
            )
            .map_err(|_| WalletError::Storage)?;
        let assets = statement
            .query_map(params![wallet_id.to_string()], asset_from_row)
            .map_err(|_| WalletError::Storage)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| WalletError::Storage)?;
        Ok(assets)
    }

    pub fn find_asset(&self, asset_id: Uuid) -> Result<Asset, WalletError> {
        let connection = self.database.connect()?;
        connection
            .query_row(
                "select id, chain, kind, symbol, name, decimals, contract_address, visible,
                    '0' as balance
                from tokens
                where id = ?1",
                params![asset_id.to_string()],
                asset_from_row,
            )
            .optional()
            .map_err(|_| WalletError::Storage)?
            .ok_or(WalletError::Storage)
    }
}
