use super::ChainId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MultisigKind {
    EvmSafe,
    TronPermission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MultisigProposalStatus {
    PendingSignatures,
    Ready,
    Executed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultisigOwnerDraft {
    pub address: String,
    pub weight: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportMultisigAccountRequest {
    pub label: String,
    pub chain: ChainId,
    pub kind: MultisigKind,
    pub address: String,
    pub threshold: u32,
    pub permission_id: Option<u32>,
    pub owners: Vec<MultisigOwnerDraft>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultisigAccount {
    pub id: Uuid,
    pub label: String,
    pub chain: ChainId,
    pub kind: MultisigKind,
    pub address: String,
    pub threshold: u32,
    pub permission_id: Option<u32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultisigOwner {
    pub multisig_account_id: Uuid,
    pub address: String,
    pub weight: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateMultisigProposalRequest {
    pub multisig_account_id: Uuid,
    pub to_address: String,
    pub asset_symbol: String,
    pub amount: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddMultisigSignatureRequest {
    pub proposal_id: Uuid,
    pub owner_address: String,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultisigProposal {
    pub id: Uuid,
    pub multisig_account_id: Uuid,
    pub chain: ChainId,
    pub to_address: String,
    pub asset_symbol: String,
    pub amount: String,
    pub payload_json: String,
    pub status: MultisigProposalStatus,
    pub threshold: u32,
    pub signature_weight: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultisigSignature {
    pub proposal_id: Uuid,
    pub owner_address: String,
    pub signature: String,
    pub weight: u32,
    pub created_at: DateTime<Utc>,
}
