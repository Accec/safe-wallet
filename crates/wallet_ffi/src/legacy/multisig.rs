use wallet_core::error::WalletError;
use wallet_core::models::{
    AddMultisigSignatureRequest, CreateMultisigProposalRequest, ImportMultisigAccountRequest,
    MultisigOwnerDraft,
};

use super::support::{engine_for_path, parse_chain, parse_multisig_kind, parse_uuid};
use crate::response::{ok_json, wallet_error_response, WalletResponse};

pub(super) fn import_account(
    db_path: String,
    label: String,
    chain: String,
    kind: String,
    address: String,
    threshold: u32,
    permission_id: Option<u32>,
    owners: Vec<MultisigOwnerDraft>,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_chain(&chain).and_then(|chain| {
        Ok(ImportMultisigAccountRequest {
            label,
            chain,
            kind: parse_multisig_kind(&kind)?,
            address,
            threshold,
            permission_id,
            owners,
        })
    }) {
        Ok(request) => match engine
            .initialize()
            .and_then(|_| engine.multisig().import_account(&request))
        {
            Ok(account) => ok_json(account),
            Err(error) => wallet_error_response(error),
        },
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn list_accounts(db_path: String) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match engine
        .initialize()
        .and_then(|_| engine.multisig().list_accounts())
    {
        Ok(accounts) => ok_json(accounts),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn create_proposal(
    db_path: String,
    multisig_account_id: String,
    to_address: String,
    asset_symbol: String,
    amount: String,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_uuid(&multisig_account_id, WalletError::MultisigNotFound).and_then(
        |multisig_account_id| {
            engine.initialize()?;
            engine
                .multisig()
                .create_proposal(&CreateMultisigProposalRequest {
                    multisig_account_id,
                    to_address,
                    asset_symbol,
                    amount,
                })
        },
    ) {
        Ok(proposal) => ok_json(proposal),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn list_proposals(
    db_path: String,
    multisig_account_id: Option<String>,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    let account_id = multisig_account_id
        .as_deref()
        .map(|id| parse_uuid(id, WalletError::MultisigNotFound))
        .transpose();
    match account_id.and_then(|account_id| {
        engine.initialize()?;
        engine.multisig().list_proposals(account_id)
    }) {
        Ok(proposals) => ok_json(proposals),
        Err(error) => wallet_error_response(error),
    }
}

pub(super) fn add_signature(
    db_path: String,
    proposal_id: String,
    owner_address: String,
    signature: String,
) -> WalletResponse {
    let engine = engine_for_path(&db_path);
    match parse_uuid(&proposal_id, WalletError::MultisigNotFound).and_then(|proposal_id| {
        engine.initialize()?;
        engine
            .multisig()
            .add_signature(&AddMultisigSignatureRequest {
                proposal_id,
                owner_address,
                signature,
            })
    }) {
        Ok(proposal) => ok_json(proposal),
        Err(error) => wallet_error_response(error),
    }
}
