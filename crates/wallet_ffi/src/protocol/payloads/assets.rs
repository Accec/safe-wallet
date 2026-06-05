use serde::Deserialize;
use uuid::Uuid;
use wallet_core::models::ChainId;

#[derive(Deserialize)]
pub(in crate::protocol) struct AddCustomTokenPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) chain: ChainId,
    pub(in crate::protocol) contract_address: String,
    pub(in crate::protocol) token_name: Option<String>,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct AssetIdPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) asset_id: Uuid,
}
