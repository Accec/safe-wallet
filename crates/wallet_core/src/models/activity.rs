use super::ChainId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityKind {
    NativeTransfer,
    TokenTransfer,
    Approval,
    Swap,
    ContractCall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRecord {
    pub chain: ChainId,
    pub tx_hash: String,
    pub kind: ActivityKind,
    pub status: ActivityStatus,
    pub summary: String,
}
