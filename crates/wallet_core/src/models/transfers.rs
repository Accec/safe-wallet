use super::ChainId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferRequest {
    pub wallet_id: Uuid,
    pub chain: ChainId,
    pub asset_id: Uuid,
    pub to_address: String,
    pub amount: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferPreview {
    pub chain: ChainId,
    pub from_address: String,
    pub to_address: String,
    pub asset_symbol: String,
    pub amount: String,
    pub fee_estimate: String,
    pub rpc_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferResult {
    pub chain: ChainId,
    pub tx_hash: String,
    pub status: String,
}
