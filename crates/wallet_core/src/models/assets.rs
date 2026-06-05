use super::ChainId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetKind {
    Native,
    Erc20,
    Trc20,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asset {
    pub id: Uuid,
    pub chain: ChainId,
    pub kind: AssetKind,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub contract_address: Option<String>,
    pub balance: String,
    pub visible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenMetadata {
    pub chain: ChainId,
    pub kind: AssetKind,
    pub contract_address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveredAsset {
    pub chain: ChainId,
    pub kind: AssetKind,
    pub contract_address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub balance: String,
}
