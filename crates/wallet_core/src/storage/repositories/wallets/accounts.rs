use super::super::super::mappers::*;
use super::super::WalletRepository;
use crate::error::WalletError;
use crate::models::{Account, ChainId};
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

impl WalletRepository {
    pub fn list_accounts(&self, wallet_id: Uuid) -> Result<Vec<Account>, WalletError> {
        if !self.wallet_exists(wallet_id)? {
            return Err(WalletError::WalletNotFound);
        }
        let connection = self.database.connect()?;
        let mut statement = connection
            .prepare(
                "select chain, address, derivation_path, account_index
                from accounts
                where wallet_id = ?1
                order by chain asc, account_index asc",
            )
            .map_err(|_| WalletError::Storage)?;
        let accounts = statement
            .query_map(params![wallet_id.to_string()], |row| {
                let chain: String = row.get(0)?;
                Ok(Account {
                    wallet_id,
                    chain: db_to_chain(&chain).map_err(sql_conversion_error)?,
                    address: row.get(1)?,
                    derivation_path: row.get(2)?,
                    account_index: row.get::<_, i64>(3)? as u32,
                })
            })
            .map_err(|_| WalletError::Storage)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| WalletError::Storage)?;
        Ok(accounts)
    }

    pub fn account_for_chain(
        &self,
        wallet_id: Uuid,
        chain: ChainId,
    ) -> Result<Account, WalletError> {
        let connection = self.database.connect()?;
        connection
            .query_row(
                "select chain, address, derivation_path, account_index
                from accounts
                where wallet_id = ?1 and chain = ?2 and account_index = 0",
                params![wallet_id.to_string(), chain_to_db(chain)],
                |row| {
                    Ok(Account {
                        wallet_id,
                        chain,
                        address: row.get(1)?,
                        derivation_path: row.get(2)?,
                        account_index: row.get::<_, i64>(3)? as u32,
                    })
                },
            )
            .optional()
            .map_err(|_| WalletError::Storage)?
            .ok_or(WalletError::WalletNotFound)
    }

    pub(crate) fn account_id_for_chain(
        &self,
        connection: &Connection,
        wallet_id: Uuid,
        chain: ChainId,
    ) -> Result<String, WalletError> {
        connection
            .query_row(
                "select id
                from accounts
                where wallet_id = ?1 and chain = ?2 and account_index = 0",
                params![wallet_id.to_string(), chain_to_db(chain)],
                |row| row.get(0),
            )
            .optional()
            .map_err(|_| WalletError::Storage)?
            .ok_or(WalletError::WalletNotFound)
    }
}
