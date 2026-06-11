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
    #[serde(default)]
    pub block_if_energy_insufficient: bool,
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
    pub resource_status: Option<TransferResourceStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferResourceStatus {
    pub energy_available: u64,
    pub energy_required: u64,
    pub bandwidth_available: u64,
    pub trx_balance_sun: u64,
    pub trx_fee_reserve_required_sun: u64,
    pub can_send_without_burning_trx: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferResult {
    pub chain: ChainId,
    pub tx_hash: String,
    pub status: String,
}
