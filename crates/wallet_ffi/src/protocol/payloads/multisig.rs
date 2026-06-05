use serde::Deserialize;
use uuid::Uuid;
use wallet_core::models::{
    AddMultisigSignatureRequest, CreateMultisigProposalRequest, ImportMultisigAccountRequest,
};

#[derive(Deserialize)]
pub(in crate::protocol) struct ImportMultisigAccountPayload {
    pub(in crate::protocol) db_path: String,
    #[serde(flatten)]
    pub(in crate::protocol) request: ImportMultisigAccountRequest,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct CreateMultisigProposalPayload {
    pub(in crate::protocol) db_path: String,
    #[serde(flatten)]
    pub(in crate::protocol) request: CreateMultisigProposalRequest,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct ListMultisigProposalsPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) multisig_account_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct AddMultisigSignaturePayload {
    pub(in crate::protocol) db_path: String,
    #[serde(flatten)]
    pub(in crate::protocol) request: AddMultisigSignatureRequest,
}
