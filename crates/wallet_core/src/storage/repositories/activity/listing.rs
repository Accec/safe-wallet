use super::super::super::mappers::*;
use super::super::ActivityRepository;
use crate::error::WalletError;
use crate::models::ActivityRecord;
use rusqlite::params;
use uuid::Uuid;

impl ActivityRepository {
    pub fn list_activity(&self, wallet_id: Uuid) -> Result<Vec<ActivityRecord>, WalletError> {
        if !self.database.wallets().wallet_exists(wallet_id)? {
            return Err(WalletError::WalletNotFound);
        }
        let connection = self.database.connect()?;
        let mut statement = connection
            .prepare(
                "select chain, tx_hash, kind, status, decoded_summary
                from activities
                where wallet_id = ?1
                order by happened_at desc, rowid desc",
            )
            .map_err(|_| WalletError::Storage)?;
        let activity = statement
            .query_map(params![wallet_id.to_string()], |row| {
                let chain: String = row.get(0)?;
                let kind: String = row.get(2)?;
                let status: String = row.get(3)?;
                Ok(ActivityRecord {
                    chain: db_to_chain(&chain).map_err(sql_conversion_error)?,
                    tx_hash: row.get(1)?,
                    kind: db_to_activity_kind(&kind).map_err(sql_conversion_error)?,
                    status: db_to_activity_status(&status).map_err(sql_conversion_error)?,
                    summary: row
                        .get::<_, Option<String>>(4)?
                        .unwrap_or_else(|| "Activity".to_string()),
                })
            })
            .map_err(|_| WalletError::Storage)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| WalletError::Storage)?;
        Ok(activity)
    }
}
