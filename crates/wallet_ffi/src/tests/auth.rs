use serde_json::Value;

use super::support::response;

#[test]
fn security_commands_set_duress_password_and_biometric_unlock() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let db_path = db_path.to_string_lossy();
    let set = response(&format!(
        r#"{{"command":"set_master_password","db_path":"{db_path}","password":"master-password"}}"#
    ));
    let real = response(&format!(
        r#"{{"command":"create_wallet","db_path":"{db_path}","label":"Real Wallet","mnemonic":"abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about","password":"master-password"}}"#
    ));
    let real_body: Value = serde_json::from_str(&real.body_json).unwrap();
    let duress = response(&format!(
        r#"{{"command":"set_duress_password","db_path":"{db_path}","master_password":"master-password","duress_password":"duress-password"}}"#
    ));
    let biometric = response(&format!(
        r#"{{"command":"update_biometric_unlock","db_path":"{db_path}","password":"master-password","enabled":true}}"#
    ));
    let status = response(&format!(
        r#"{{"command":"app_status","db_path":"{db_path}"}}"#
    ));
    let duress_unlock = response(&format!(
        r#"{{"command":"unlock_app","db_path":"{db_path}","password":"duress-password"}}"#
    ));
    let old_master_unlock = response(&format!(
        r#"{{"command":"unlock_app","db_path":"{db_path}","password":"master-password"}}"#
    ));
    let wallets = response(&format!(
        r#"{{"command":"list_wallets","db_path":"{db_path}"}}"#
    ));
    let status_body: Value = serde_json::from_str(&status.body_json).unwrap();
    let wallets_body: Vec<Value> = serde_json::from_str(&wallets.body_json).unwrap();

    assert!(set.ok);
    assert!(real.ok);
    assert!(duress.ok);
    assert!(biometric.ok);
    assert!(duress_unlock.ok);
    assert!(!old_master_unlock.ok);
    assert!(wallets.ok);
    assert_eq!(status_body["biometric_enabled"], true);
    assert_eq!(wallets_body.len(), 1);
    assert_eq!(wallets_body[0]["label"], "Primary");
    assert_ne!(wallets_body[0]["id"], real_body["id"]);
    assert!(!duress.body_json.contains("duress-password"));
    assert!(!duress_unlock.body_json.contains("duress-password"));
}

#[test]
fn lock_command_does_not_report_fake_success() {
    let lock = response(r#"{"command":"lock_app","db_path":"/tmp/wallet.sqlite"}"#);

    assert!(!lock.ok);
    assert_eq!(lock.error.as_deref(), Some("Wallet is locked"));
}
