use super::ChainId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyMode {
    Custom,
    Tor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainSettings {
    pub chain: ChainId,
    pub network_name: String,
    pub chain_id: Option<String>,
    pub enabled: bool,
    pub default_rpc_url: String,
    pub user_rpc_url: Option<String>,
    pub indexer_endpoint: Option<String>,
    pub explorer_url: Option<String>,
    pub native_symbol: String,
    pub native_decimals: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkPrivacySettings {
    pub proxy_enabled: bool,
    pub proxy_mode: ProxyMode,
    pub proxy_url: Option<String>,
}
