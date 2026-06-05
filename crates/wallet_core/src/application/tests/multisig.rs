#![allow(unused_imports)]

use super::super::WalletEngine;
use super::support::*;
use crate::domains::auth::security;
use crate::domains::wallets::keystore;
use crate::error::WalletError;
use crate::models::*;
use crate::protocol::rpc::AssetBalanceClient;
use crate::storage::WalletDatabase;
use std::fs;
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use uuid::Uuid;

#[test]
fn imports_evm_safe_multisig_account() {
    let fixture = engine_fixture();

    let account = fixture
        .engine
        .multisig()
        .import_account(&crate::models::ImportMultisigAccountRequest {
            label: "Treasury Safe".to_string(),
            chain: ChainId::Ethereum,
            kind: crate::models::MultisigKind::EvmSafe,
            address: "0x1111111111111111111111111111111111111111".to_string(),
            threshold: 2,
            permission_id: None,
            owners: vec![
                crate::models::MultisigOwnerDraft {
                    address: "0x2222222222222222222222222222222222222222".to_string(),
                    weight: 1,
                },
                crate::models::MultisigOwnerDraft {
                    address: "0x3333333333333333333333333333333333333333".to_string(),
                    weight: 1,
                },
            ],
        })
        .unwrap();

    assert_eq!(account.label, "Treasury Safe");
    assert_eq!(account.kind, crate::models::MultisigKind::EvmSafe);
    assert_eq!(account.threshold, 2);

    let accounts = fixture.engine.multisig().list_accounts().unwrap();
    assert_eq!(accounts, vec![account]);
}

#[test]
fn imports_tron_permission_multisig_account() {
    let fixture = engine_fixture();

    let account = fixture
        .engine
        .multisig()
        .import_account(&crate::models::ImportMultisigAccountRequest {
            label: "TRON Ops".to_string(),
            chain: ChainId::Tron,
            kind: crate::models::MultisigKind::TronPermission,
            address: "TUEZSdKsoDHQMeZwihtdoBiN46zxhGWYdH".to_string(),
            threshold: 2,
            permission_id: Some(2),
            owners: vec![
                crate::models::MultisigOwnerDraft {
                    address: "TUEZSdKsoDHQMeZwihtdoBiN46zxhGWYdH".to_string(),
                    weight: 1,
                },
                crate::models::MultisigOwnerDraft {
                    address: "TX7kybeP6UwTBRHLNPYmswFESHfyjm9bAS".to_string(),
                    weight: 1,
                },
            ],
        })
        .unwrap();

    assert_eq!(account.kind, crate::models::MultisigKind::TronPermission);
    assert_eq!(account.permission_id, Some(2));
}

#[test]
fn multisig_proposal_becomes_ready_when_signature_weight_reaches_threshold() {
    let fixture = engine_fixture();
    let account = fixture
        .engine
        .multisig()
        .import_account(&crate::models::ImportMultisigAccountRequest {
            label: "Treasury Safe".to_string(),
            chain: ChainId::Ethereum,
            kind: crate::models::MultisigKind::EvmSafe,
            address: "0x1111111111111111111111111111111111111111".to_string(),
            threshold: 2,
            permission_id: None,
            owners: vec![
                crate::models::MultisigOwnerDraft {
                    address: "0x2222222222222222222222222222222222222222".to_string(),
                    weight: 1,
                },
                crate::models::MultisigOwnerDraft {
                    address: "0x3333333333333333333333333333333333333333".to_string(),
                    weight: 1,
                },
            ],
        })
        .unwrap();

    let proposal = fixture
        .engine
        .multisig()
        .create_proposal(&crate::models::CreateMultisigProposalRequest {
            multisig_account_id: account.id,
            to_address: "0x4444444444444444444444444444444444444444".to_string(),
            asset_symbol: "ETH".to_string(),
            amount: "1.25".to_string(),
        })
        .unwrap();
    assert_eq!(
        proposal.status,
        crate::models::MultisigProposalStatus::PendingSignatures
    );
    assert!(proposal.payload_json.contains("\"kind\":\"evm_safe\""));

    let proposal = fixture
        .engine
        .multisig()
        .add_signature(&crate::models::AddMultisigSignatureRequest {
            proposal_id: proposal.id,
            owner_address: "0x2222222222222222222222222222222222222222".to_string(),
            signature: "0xsig1".to_string(),
        })
        .unwrap();
    assert_eq!(
        proposal.status,
        crate::models::MultisigProposalStatus::PendingSignatures
    );
    assert_eq!(proposal.signature_weight, 1);

    let proposal = fixture
        .engine
        .multisig()
        .add_signature(&crate::models::AddMultisigSignatureRequest {
            proposal_id: proposal.id,
            owner_address: "0x3333333333333333333333333333333333333333".to_string(),
            signature: "0xsig2".to_string(),
        })
        .unwrap();
    assert_eq!(
        proposal.status,
        crate::models::MultisigProposalStatus::Ready
    );
    assert_eq!(proposal.signature_weight, 2);
}

#[test]
fn multisig_rejects_duplicate_owner_signature() {
    let fixture = engine_fixture();
    let account = fixture
        .engine
        .multisig()
        .import_account(&crate::models::ImportMultisigAccountRequest {
            label: "Treasury Safe".to_string(),
            chain: ChainId::Ethereum,
            kind: crate::models::MultisigKind::EvmSafe,
            address: "0x1111111111111111111111111111111111111111".to_string(),
            threshold: 1,
            permission_id: None,
            owners: vec![crate::models::MultisigOwnerDraft {
                address: "0x2222222222222222222222222222222222222222".to_string(),
                weight: 1,
            }],
        })
        .unwrap();
    let proposal = fixture
        .engine
        .multisig()
        .create_proposal(&crate::models::CreateMultisigProposalRequest {
            multisig_account_id: account.id,
            to_address: "0x4444444444444444444444444444444444444444".to_string(),
            asset_symbol: "ETH".to_string(),
            amount: "1.25".to_string(),
        })
        .unwrap();
    fixture
        .engine
        .multisig()
        .add_signature(&crate::models::AddMultisigSignatureRequest {
            proposal_id: proposal.id,
            owner_address: "0x2222222222222222222222222222222222222222".to_string(),
            signature: "0xsig1".to_string(),
        })
        .unwrap();

    let error = fixture
        .engine
        .multisig()
        .add_signature(&crate::models::AddMultisigSignatureRequest {
            proposal_id: proposal.id,
            owner_address: "0x2222222222222222222222222222222222222222".to_string(),
            signature: "0xsig2".to_string(),
        })
        .unwrap_err();

    assert_eq!(error, WalletError::DuplicateMultisigSignature);
}
