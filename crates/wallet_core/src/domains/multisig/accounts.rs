use chrono::Utc;
use uuid::Uuid;

use crate::domains::validation::validate_multisig_request;
use crate::error::WalletError;
use crate::models::{ImportMultisigAccountRequest, MultisigAccount, MultisigOwnerDraft};
use crate::storage::WalletDatabase;

pub(super) fn import_account(
    database: &WalletDatabase,
    request: &ImportMultisigAccountRequest,
) -> Result<MultisigAccount, WalletError> {
    validate_multisig_request(request)?;
    let account = MultisigAccount {
        id: Uuid::new_v4(),
        label: if request.label.trim().is_empty() {
            "Multisig".to_string()
        } else {
            request.label.trim().to_string()
        },
        chain: request.chain,
        kind: request.kind,
        address: request.address.trim().to_string(),
        threshold: request.threshold,
        permission_id: request.permission_id,
        created_at: Utc::now(),
    };
    let owners = request
        .owners
        .iter()
        .map(|owner| MultisigOwnerDraft {
            address: owner.address.trim().to_string(),
            weight: owner.weight,
        })
        .collect::<Vec<_>>();
    database
        .multisig()
        .save_multisig_account(&account, &owners)?;
    Ok(account)
}

pub(super) fn list_accounts(
    database: &WalletDatabase,
) -> Result<Vec<MultisigAccount>, WalletError> {
    database.multisig().list_multisig_accounts()
}
