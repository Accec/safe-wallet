use chrono::Utc;
use uuid::Uuid;

use crate::domains::validation::{multisig_payload_json, validate_chain_address};
use crate::error::WalletError;
use crate::models::{CreateMultisigProposalRequest, MultisigProposal, MultisigProposalStatus};
use crate::storage::WalletDatabase;

pub(super) fn create_proposal(
    database: &WalletDatabase,
    request: &CreateMultisigProposalRequest,
) -> Result<MultisigProposal, WalletError> {
    if request.amount.trim().is_empty()
        || request.amount.trim().starts_with('-')
        || request.asset_symbol.trim().is_empty()
    {
        return Err(WalletError::InvalidMultisigSettings);
    }
    let account = database
        .multisig()
        .load_multisig_account(request.multisig_account_id)?;
    validate_chain_address(account.chain, &request.to_address)?;
    let payload_json = multisig_payload_json(&account, request)?;
    let proposal = MultisigProposal {
        id: Uuid::new_v4(),
        multisig_account_id: account.id,
        chain: account.chain,
        to_address: request.to_address.trim().to_string(),
        asset_symbol: request.asset_symbol.trim().to_string(),
        amount: request.amount.trim().to_string(),
        payload_json,
        status: MultisigProposalStatus::PendingSignatures,
        threshold: account.threshold,
        signature_weight: 0,
        created_at: Utc::now(),
    };
    database.multisig().save_multisig_proposal(&proposal)?;
    Ok(proposal)
}

pub(super) fn list_proposals(
    database: &WalletDatabase,
    account_id: Option<Uuid>,
) -> Result<Vec<MultisigProposal>, WalletError> {
    database.multisig().list_multisig_proposals(account_id)
}
