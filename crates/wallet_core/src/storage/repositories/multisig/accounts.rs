use super::super::super::mappers::*;
use super::super::MultisigRepository;
use crate::error::WalletError;
use crate::models::{MultisigAccount, MultisigOwnerDraft};
use rusqlite::{params, OptionalExtension};
use uuid::Uuid;

impl MultisigRepository {
    pub fn save_multisig_account(
        &self,
        account: &MultisigAccount,
        owners: &[MultisigOwnerDraft],
    ) -> Result<(), WalletError> {
        let mut connection = self.database.connect()?;
        let transaction = connection.transaction().map_err(|_| WalletError::Storage)?;
        let created_at = account.created_at.to_rfc3339();
        transaction
            .execute(
                "insert into multisig_accounts (
                    id, label, chain, kind, address, threshold, permission_id, created_at, updated_at
                ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)",
                params![
                    account.id.to_string(),
                    account.label,
                    chain_to_db(account.chain),
                    multisig_kind_to_db(account.kind),
                    account.address,
                    account.threshold,
                    account.permission_id.map(i64::from),
                    created_at,
                ],
            )
            .map_err(|_| WalletError::Storage)?;
        for owner in owners {
            transaction
                .execute(
                    "insert into multisig_owners (
                        id, multisig_account_id, address, weight, created_at
                    ) values (?1, ?2, ?3, ?4, ?5)",
                    params![
                        Uuid::new_v4().to_string(),
                        account.id.to_string(),
                        owner.address,
                        owner.weight,
                        created_at,
                    ],
                )
                .map_err(|_| WalletError::Storage)?;
        }
        transaction.commit().map_err(|_| WalletError::Storage)?;
        Ok(())
    }

    pub fn list_multisig_accounts(&self) -> Result<Vec<MultisigAccount>, WalletError> {
        let connection = self.database.connect()?;
        let mut statement = connection
            .prepare(
                "select id, label, chain, kind, address, threshold, permission_id, created_at
                from multisig_accounts
                order by created_at asc",
            )
            .map_err(|_| WalletError::Storage)?;
        let accounts = statement
            .query_map([], multisig_account_from_row)
            .map_err(|_| WalletError::Storage)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| WalletError::Storage)?;
        Ok(accounts)
    }

    pub fn load_multisig_account(&self, account_id: Uuid) -> Result<MultisigAccount, WalletError> {
        let connection = self.database.connect()?;
        connection
            .query_row(
                "select id, label, chain, kind, address, threshold, permission_id, created_at
                from multisig_accounts
                where id = ?1",
                params![account_id.to_string()],
                multisig_account_from_row,
            )
            .optional()
            .map_err(|_| WalletError::Storage)?
            .ok_or(WalletError::MultisigNotFound)
    }
}
