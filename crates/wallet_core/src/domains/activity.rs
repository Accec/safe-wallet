use crate::error::WalletError;
use crate::models::{ActivityRecord, ChainId};
use crate::protocol::indexers::{ActivityIndexer, HttpActivityIndexer};
use crate::storage::WalletDatabase;
use uuid::Uuid;

pub struct ActivityDomain {
    database: WalletDatabase,
}

impl ActivityDomain {
    pub(crate) fn new(database: WalletDatabase) -> Self {
        Self { database }
    }

    pub fn list_activity(&self, wallet_id: Uuid) -> Result<Vec<ActivityRecord>, WalletError> {
        self.database.activity().list_activity(wallet_id)
    }

    pub fn sync_activity(
        &self,
        wallet_id: Uuid,
        chain: Option<ChainId>,
    ) -> Result<usize, WalletError> {
        let settings = self.database.network().network_privacy_settings()?;
        let indexer = HttpActivityIndexer::new(&settings)?;
        self.sync_activity_with(wallet_id, chain, &indexer)
    }

    pub fn sync_activity_with<I: ActivityIndexer>(
        &self,
        wallet_id: Uuid,
        selected_chain: Option<ChainId>,
        indexer: &I,
    ) -> Result<usize, WalletError> {
        let accounts = self.database.wallets().list_accounts(wallet_id)?;
        let mut synced = 0;
        for account in accounts {
            if selected_chain.is_some_and(|chain| account.chain != chain) {
                continue;
            }
            let Some(endpoint) = self.database.network().indexer_endpoint(account.chain)? else {
                continue;
            };
            for record in indexer.fetch_activity(account.chain, &endpoint, &account.address)? {
                if record.chain != account.chain {
                    continue;
                }
                self.database.activity().save_indexed_activity(
                    wallet_id,
                    account.chain,
                    &record,
                )?;
                synced += 1;
            }
        }
        Ok(synced)
    }
}
