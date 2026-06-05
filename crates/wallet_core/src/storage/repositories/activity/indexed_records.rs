use super::super::super::mappers::*;
use super::super::ActivityRepository;
use crate::error::WalletError;
use crate::models::{ActivityRecord, ChainId};
use chrono::Utc;
use rusqlite::params;
use uuid::Uuid;

impl ActivityRepository {
    pub fn save_indexed_activity(
        &self,
        wallet_id: Uuid,
        chain: ChainId,
        record: &ActivityRecord,
    ) -> Result<(), WalletError> {
        let connection = self.database.connect()?;
        let account_id =
            self.database
                .wallets()
                .account_id_for_chain(&connection, wallet_id, chain)?;
        let now = Utc::now().to_rfc3339();
        connection
            .execute(
                "insert into activities (
                    id, wallet_id, account_id, chain, tx_hash, kind, status,
                    provider_id, happened_at, decoded_summary, safe_provider_ref
                ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'custom', ?8, ?9, 'indexer')
                on conflict(chain, tx_hash, kind, account_id)
                do update set
                    status = excluded.status,
                    decoded_summary = excluded.decoded_summary,
                    safe_provider_ref = excluded.safe_provider_ref",
                params![
                    Uuid::new_v4().to_string(),
                    wallet_id.to_string(),
                    account_id,
                    chain_to_db(chain),
                    record.tx_hash,
                    activity_kind_to_db(record.kind),
                    activity_status_to_db(record.status),
                    now,
                    record.summary,
                ],
            )
            .map_err(|_| WalletError::Storage)?;
        Ok(())
    }
}
