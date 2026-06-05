use crate::error::WalletError;
use crate::models::{
    AddMultisigSignatureRequest, CreateMultisigProposalRequest, ImportMultisigAccountRequest,
    MultisigAccount, MultisigProposal,
};
use crate::storage::WalletDatabase;
use uuid::Uuid;

mod accounts;
mod proposals;
mod signatures;

pub struct MultisigDomain {
    pub(super) database: WalletDatabase,
}

impl MultisigDomain {
    pub(crate) fn new(database: WalletDatabase) -> Self {
        Self { database }
    }

    pub fn import_account(
        &self,
        request: &ImportMultisigAccountRequest,
    ) -> Result<MultisigAccount, WalletError> {
        accounts::import_account(&self.database, request)
    }

    pub fn list_accounts(&self) -> Result<Vec<MultisigAccount>, WalletError> {
        accounts::list_accounts(&self.database)
    }

    pub fn create_proposal(
        &self,
        request: &CreateMultisigProposalRequest,
    ) -> Result<MultisigProposal, WalletError> {
        proposals::create_proposal(&self.database, request)
    }

    pub fn list_proposals(
        &self,
        account_id: Option<Uuid>,
    ) -> Result<Vec<MultisigProposal>, WalletError> {
        proposals::list_proposals(&self.database, account_id)
    }

    pub fn add_signature(
        &self,
        request: &AddMultisigSignatureRequest,
    ) -> Result<MultisigProposal, WalletError> {
        signatures::add_signature(&self.database, request)
    }
}
