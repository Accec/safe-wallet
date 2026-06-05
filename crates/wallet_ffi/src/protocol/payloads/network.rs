use serde::Deserialize;
use wallet_core::models::{ChainId, NetworkPrivacySettings};

#[derive(Deserialize)]
pub(in crate::protocol) struct NetworkPrivacyPayload {
    pub(in crate::protocol) db_path: String,
    #[serde(flatten)]
    pub(in crate::protocol) settings: NetworkPrivacySettings,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct SaveNetworkSettingsPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) network_name: String,
    pub(in crate::protocol) rpc_url: String,
    pub(in crate::protocol) chain_id: String,
    pub(in crate::protocol) currency_symbol: String,
    pub(in crate::protocol) block_explorer_url: Option<String>,
    pub(in crate::protocol) indexer_endpoint: Option<String>,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct UpdateChainRpcPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) chain: ChainId,
    pub(in crate::protocol) rpc_url: String,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct UpdateIndexerPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) chain: ChainId,
    pub(in crate::protocol) endpoint: String,
    pub(in crate::protocol) api_key: Option<String>,
}
