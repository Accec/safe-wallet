use super::super::super::mappers::*;
use super::super::MultisigRepository;
use crate::error::WalletError;
use crate::models::{MultisigProposal, MultisigProposalStatus, MultisigSignature};
use chrono::Utc;
use rusqlite::params;
use uuid::Uuid;

impl MultisigRepository {
    pub fn save_multisig_signature(
        &self,
        signature: &MultisigSignature,
    ) -> Result<MultisigProposal, WalletError> {
        let mut connection = self.database.connect()?;
        let transaction = connection.transaction().map_err(|_| WalletError::Storage)?;
        let created_at = signature.created_at.to_rfc3339();
        transaction
            .execute(
                "insert into multisig_signatures (
                    id, proposal_id, owner_address, signature, weight, created_at
                ) values (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    Uuid::new_v4().to_string(),
                    signature.proposal_id.to_string(),
                    signature.owner_address,
                    signature.signature,
                    signature.weight,
                    created_at,
                ],
            )
            .map_err(map_multisig_signature_insert_error)?;
        let signature_weight: i64 = transaction
            .query_row(
                "select coalesce(sum(weight), 0)
                from multisig_signatures
                where proposal_id = ?1",
                params![signature.proposal_id.to_string()],
                |row| row.get(0),
            )
            .map_err(|_| WalletError::Storage)?;
        let threshold: i64 = transaction
            .query_row(
                "select threshold from multisig_proposals where id = ?1",
                params![signature.proposal_id.to_string()],
                |row| row.get(0),
            )
            .map_err(|_| WalletError::Storage)?;
        let status = if signature_weight >= threshold {
            MultisigProposalStatus::Ready
        } else {
            MultisigProposalStatus::PendingSignatures
        };
        transaction
            .execute(
                "update multisig_proposals
                set signature_weight = ?1, status = ?2, updated_at = ?3
                where id = ?4",
                params![
                    signature_weight,
                    multisig_status_to_db(status),
                    Utc::now().to_rfc3339(),
                    signature.proposal_id.to_string(),
                ],
            )
            .map_err(|_| WalletError::Storage)?;
        transaction.commit().map_err(|_| WalletError::Storage)?;
        self.load_multisig_proposal(signature.proposal_id)
    }
}
