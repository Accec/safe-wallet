use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChainId {
    Btc,
    Ethereum,
    Bsc,
    Polygon,
    Arbitrum,
    Optimism,
    Tron,
}
