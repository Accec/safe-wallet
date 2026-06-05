use serde::Deserialize;
use uuid::Uuid;
use wallet_core::models::ChainId;

#[derive(Deserialize)]
pub(in crate::protocol) struct WalletChainPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) wallet_id: Uuid,
    pub(in crate::protocol) chain: Option<ChainId>,
}
