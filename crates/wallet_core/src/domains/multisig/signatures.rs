use chrono::Utc;

use crate::domains::validation::validate_chain_address;
use crate::error::WalletError;
use crate::models::{AddMultisigSignatureRequest, MultisigProposal, MultisigSignature};
use crate::storage::WalletDatabase;

pub(super) fn add_signature(
    database: &WalletDatabase,
    request: &AddMultisigSignatureRequest,
) -> Result<MultisigProposal, WalletError> {
    if request.signature.trim().is_empty() {
        return Err(WalletError::InvalidMultisigSettings);
    }
    let proposal = database
        .multisig()
        .load_multisig_proposal(request.proposal_id)?;
    let account = database
        .multisig()
        .load_multisig_account(proposal.multisig_account_id)?;
    validate_chain_address(account.chain, &request.owner_address)?;
    let weight = database
        .multisig()
        .owner_weight_for_multisig(account.id, request.owner_address.trim())?;
    let signature = MultisigSignature {
        proposal_id: request.proposal_id,
        owner_address: request.owner_address.trim().to_string(),
        signature: request.signature.trim().to_string(),
        weight,
        created_at: Utc::now(),
    };
    database.multisig().save_multisig_signature(&signature)
}
