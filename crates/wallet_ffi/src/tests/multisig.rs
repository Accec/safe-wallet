use serde_json::Value;

use super::support::response;

#[test]
fn multisig_commands_round_trip_public_models() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let db_path = db_path.to_string_lossy();
    let import = response(&format!(
        r#"{{"command":"import_multisig_account","db_path":"{db_path}","label":"Treasury Safe","chain":"ethereum","kind":"evm_safe","address":"0x1111111111111111111111111111111111111111","threshold":2,"permission_id":null,"owners":[{{"address":"0x2222222222222222222222222222222222222222","weight":1}},{{"address":"0x3333333333333333333333333333333333333333","weight":1}}]}}"#
    ));
    let account: Value = serde_json::from_str(&import.body_json).unwrap();
    let account_id = account["id"].as_str().unwrap();
    let list = response(&format!(
        r#"{{"command":"list_multisig_accounts","db_path":"{db_path}"}}"#
    ));
    let proposal = response(&format!(
        r#"{{"command":"create_multisig_proposal","db_path":"{db_path}","multisig_account_id":"{account_id}","to_address":"0x4444444444444444444444444444444444444444","asset_symbol":"ETH","amount":"1.25"}}"#
    ));
    let proposal_body: Value = serde_json::from_str(&proposal.body_json).unwrap();
    let proposal_id = proposal_body["id"].as_str().unwrap();
    let first_signature = response(&format!(
        r#"{{"command":"add_multisig_signature","db_path":"{db_path}","proposal_id":"{proposal_id}","owner_address":"0x2222222222222222222222222222222222222222","signature":"0xsig1"}}"#
    ));
    let second_signature = response(&format!(
        r#"{{"command":"add_multisig_signature","db_path":"{db_path}","proposal_id":"{proposal_id}","owner_address":"0x3333333333333333333333333333333333333333","signature":"0xsig2"}}"#
    ));
    let duplicate_signature = response(&format!(
        r#"{{"command":"add_multisig_signature","db_path":"{db_path}","proposal_id":"{proposal_id}","owner_address":"0x3333333333333333333333333333333333333333","signature":"0xsig3"}}"#
    ));
    let proposals = response(&format!(
        r#"{{"command":"list_multisig_proposals","db_path":"{db_path}","multisig_account_id":null}}"#
    ));
    let list_body: Vec<Value> = serde_json::from_str(&list.body_json).unwrap();
    let first_signature_body: Value = serde_json::from_str(&first_signature.body_json).unwrap();
    let second_signature_body: Value = serde_json::from_str(&second_signature.body_json).unwrap();
    let proposals_body: Vec<Value> = serde_json::from_str(&proposals.body_json).unwrap();

    assert!(import.ok);
    assert!(list.ok);
    assert!(proposal.ok);
    assert!(first_signature.ok);
    assert!(second_signature.ok);
    assert!(!duplicate_signature.ok);
    assert!(proposals.ok);
    assert_eq!(account["kind"], "evm_safe");
    assert_eq!(list_body.len(), 1);
    assert_eq!(proposal_body["status"], "pending_signatures");
    assert!(proposal_body["payload_json"]
        .as_str()
        .unwrap()
        .contains("evm_safe"));
    assert_eq!(first_signature_body["signature_weight"], 1);
    assert_eq!(second_signature_body["status"], "ready");
    assert_eq!(second_signature_body["signature_weight"], 2);
    assert_eq!(proposals_body.len(), 1);
    assert!(!proposal.body_json.contains("master-password"));
    assert!(!second_signature.body_json.contains("master-password"));
}
