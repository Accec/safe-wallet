use crate::error::WalletError;
use crate::models::{ChainId, ChainSettings, NetworkPrivacySettings};
use crate::storage::WalletDatabase;

mod privacy;
mod settings;
mod validation;

pub struct NetworkDomain {
    pub(super) database: WalletDatabase,
}

impl NetworkDomain {
    pub(crate) fn new(database: WalletDatabase) -> Self {
        Self { database }
    }

    pub fn privacy_settings(&self) -> Result<NetworkPrivacySettings, WalletError> {
        privacy::privacy_settings(&self.database)
    }

    pub fn save_privacy_settings(
        &self,
        settings: NetworkPrivacySettings,
    ) -> Result<(), WalletError> {
        privacy::save_privacy_settings(&self.database, settings)
    }

    pub fn list_settings(&self) -> Result<Vec<ChainSettings>, WalletError> {
        settings::list_settings(&self.database)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn save_settings(
        &self,
        chain: ChainId,
        network_name: &str,
        chain_id: &str,
        rpc_url: &str,
        currency_symbol: &str,
        block_explorer_url: Option<&str>,
        indexer_endpoint: Option<&str>,
    ) -> Result<(), WalletError> {
        settings::save_settings(
            &self.database,
            chain,
            network_name,
            chain_id,
            rpc_url,
            currency_symbol,
            block_explorer_url,
            indexer_endpoint,
        )
    }

    pub fn update_chain_rpc(&self, chain: ChainId, rpc_url: &str) -> Result<(), WalletError> {
        settings::update_chain_rpc(&self.database, chain, rpc_url)
    }

    pub fn update_indexer_settings(
        &self,
        chain: ChainId,
        endpoint: &str,
        api_key: Option<&str>,
    ) -> Result<(), WalletError> {
        settings::update_indexer_settings(&self.database, chain, endpoint, api_key)
    }
}
