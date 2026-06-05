use super::super::super::mappers::*;
use super::super::MultisigRepository;
use crate::error::WalletError;
use crate::models::MultisigProposal;
use rusqlite::{params, OptionalExtension};
use uuid::Uuid;

impl MultisigRepository {
    pub fn save_multisig_proposal(&self, proposal: &MultisigProposal) -> Result<(), WalletError> {
        let connection = self.database.connect()?;
        let created_at = proposal.created_at.to_rfc3339();
        connection
            .execute(
                "insert into multisig_proposals (
                    id, multisig_account_id, chain, to_address, asset_symbol, amount,
                    payload_json, status, threshold, signature_weight, created_at, updated_at
                ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?11)",
                params![
                    proposal.id.to_string(),
                    proposal.multisig_account_id.to_string(),
                    chain_to_db(proposal.chain),
                    proposal.to_address,
                    proposal.asset_symbol,
                    proposal.amount,
                    proposal.payload_json,
                    multisig_status_to_db(proposal.status),
                    proposal.threshold,
                    proposal.signature_weight,
                    created_at,
                ],
            )
            .map_err(|_| WalletError::Storage)?;
        Ok(())
    }

    pub fn list_multisig_proposals(
        &self,
        account_id: Option<Uuid>,
    ) -> Result<Vec<MultisigProposal>, WalletError> {
        let connection = self.database.connect()?;
        let sql = match account_id {
            Some(_) => {
                "select id, multisig_account_id, chain, to_address, asset_symbol, amount,
                    payload_json, status, threshold, signature_weight, created_at
                from multisig_proposals
                where multisig_account_id = ?1
                order by created_at desc"
            }
            None => {
                "select id, multisig_account_id, chain, to_address, asset_symbol, amount,
                    payload_json, status, threshold, signature_weight, created_at
                from multisig_proposals
                order by created_at desc"
            }
        };
        let mut statement = connection.prepare(sql).map_err(|_| WalletError::Storage)?;
        if let Some(account_id) = account_id {
            statement
                .query_map(params![account_id.to_string()], multisig_proposal_from_row)
                .map_err(|_| WalletError::Storage)?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| WalletError::Storage)
        } else {
            statement
                .query_map([], multisig_proposal_from_row)
                .map_err(|_| WalletError::Storage)?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| WalletError::Storage)
        }
    }

    pub fn load_multisig_proposal(
        &self,
        proposal_id: Uuid,
    ) -> Result<MultisigProposal, WalletError> {
        let connection = self.database.connect()?;
        connection
            .query_row(
                "select id, multisig_account_id, chain, to_address, asset_symbol, amount,
                    payload_json, status, threshold, signature_weight, created_at
                from multisig_proposals
                where id = ?1",
                params![proposal_id.to_string()],
                multisig_proposal_from_row,
            )
            .optional()
            .map_err(|_| WalletError::Storage)?
            .ok_or(WalletError::MultisigNotFound)
    }
}
