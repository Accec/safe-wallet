use serde_json::Value;

use super::error::CommandResult;
use super::payloads::{
    payload_as, AddMultisigSignaturePayload, CreateMultisigProposalPayload, DbPayload,
    ImportMultisigAccountPayload, ListMultisigProposalsPayload,
};
use super::support::{initialized_engine, json_data};

pub(super) fn import_account(payload: &Value) -> CommandResult {
    let payload: ImportMultisigAccountPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(engine.multisig().import_account(&payload.request)?)
}

pub(super) fn list_accounts(payload: &Value) -> CommandResult {
    let payload: DbPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(engine.multisig().list_accounts()?)
}

pub(super) fn create_proposal(payload: &Value) -> CommandResult {
    let payload: CreateMultisigProposalPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(engine.multisig().create_proposal(&payload.request)?)
}

pub(super) fn list_proposals(payload: &Value) -> CommandResult {
    let payload: ListMultisigProposalsPayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(
        engine
            .multisig()
            .list_proposals(payload.multisig_account_id)?,
    )
}

pub(super) fn add_signature(payload: &Value) -> CommandResult {
    let payload: AddMultisigSignaturePayload = payload_as(payload)?;
    let engine = initialized_engine(&payload.db_path)?;
    json_data(engine.multisig().add_signature(&payload.request)?)
}
