use super::super::super::mappers::*;
use super::super::MultisigRepository;
use crate::error::WalletError;
use crate::models::MultisigOwner;
use rusqlite::{params, OptionalExtension};
use uuid::Uuid;

impl MultisigRepository {
    pub fn list_multisig_owners(
        &self,
        account_id: Uuid,
    ) -> Result<Vec<MultisigOwner>, WalletError> {
        let connection = self.database.connect()?;
        let mut statement = connection
            .prepare(
                "select multisig_account_id, address, weight
                from multisig_owners
                where multisig_account_id = ?1
                order by address asc",
            )
            .map_err(|_| WalletError::Storage)?;
        let owners = statement
            .query_map(params![account_id.to_string()], multisig_owner_from_row)
            .map_err(|_| WalletError::Storage)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| WalletError::Storage)?;
        Ok(owners)
    }

    pub fn owner_weight_for_multisig(
        &self,
        account_id: Uuid,
        owner_address: &str,
    ) -> Result<u32, WalletError> {
        let connection = self.database.connect()?;
        connection
            .query_row(
                "select weight
                from multisig_owners
                where multisig_account_id = ?1 and lower(address) = lower(?2)",
                params![account_id.to_string(), owner_address],
                |row| row.get::<_, i64>(0),
            )
            .optional()
            .map_err(|_| WalletError::Storage)?
            .map(|weight| weight as u32)
            .ok_or(WalletError::UnauthorizedMultisigSigner)
    }
}
