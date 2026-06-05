use super::ChainId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletSummary {
    pub id: Uuid,
    pub label: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeystoreExport {
    pub wallet_id: Uuid,
    pub label: String,
    pub secret_kind: String,
    pub ciphertext_b64: String,
    pub nonce_b64: String,
    pub salt_b64: String,
    pub kdf_name: String,
    pub kdf_params_json: String,
    pub cipher_name: String,
    pub version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub wallet_id: Uuid,
    pub chain: ChainId,
    pub address: String,
    pub derivation_path: String,
    pub account_index: u32,
}
