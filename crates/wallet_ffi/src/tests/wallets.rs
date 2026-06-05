use serde_json::Value;

use super::support::response;

#[test]
fn master_password_and_wallet_commands_use_wallet_engine() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let db_path = db_path.to_string_lossy();
    let set = response(&format!(
        r#"{{"command":"set_master_password","db_path":"{db_path}","password":"master-password"}}"#
    ));
    let create = response(&format!(
        r#"{{"command":"create_wallet","db_path":"{db_path}","label":"Primary","mnemonic":"abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about","password":"master-password"}}"#
    ));
    let list = response(&format!(
        r#"{{"command":"list_wallets","db_path":"{db_path}"}}"#
    ));

    assert!(set.ok);
    assert!(create.ok);
    assert!(list.ok);
    assert!(!create.body_json.contains("mnemonic"));
    assert!(!create.body_json.contains("master-password"));

    let wallets: Vec<serde_json::Value> = serde_json::from_str(&list.body_json).unwrap();
    assert_eq!(wallets.len(), 1);
    assert_eq!(wallets[0]["label"], "Primary");
}

#[test]
fn private_key_import_and_keystore_export_return_encrypted_metadata_only() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let db_path = db_path.to_string_lossy();
    let private_key = "0000000000000000000000000000000000000000000000000000000000000001";
    let set = response(&format!(
        r#"{{"command":"set_master_password","db_path":"{db_path}","password":"master-password"}}"#
    ));
    let imported = response(&format!(
        r#"{{"command":"import_private_key","db_path":"{db_path}","label":"Key Wallet","private_key":"{private_key}","password":"master-password"}}"#
    ));
    let wallet: Value = serde_json::from_str(&imported.body_json).unwrap();
    let wallet_id = wallet["id"].as_str().unwrap();
    let accounts = response(&format!(
        r#"{{"command":"list_accounts","db_path":"{db_path}","wallet_id":"{wallet_id}"}}"#
    ));
    let export = response(&format!(
        r#"{{"command":"export_keystore","db_path":"{db_path}","wallet_id":"{wallet_id}","password":"master-password"}}"#
    ));
    let wrong_password = response(&format!(
        r#"{{"command":"export_keystore","db_path":"{db_path}","wallet_id":"{wallet_id}","password":"wrong-password"}}"#
    ));
    let accounts_body: Vec<Value> = serde_json::from_str(&accounts.body_json).unwrap();
    let export_body: Value = serde_json::from_str(&export.body_json).unwrap();

    assert!(set.ok);
    assert!(imported.ok);
    assert!(accounts.ok);
    assert!(export.ok);
    assert!(!wrong_password.ok);
    assert_eq!(accounts_body.len(), 7);
    assert_eq!(
        accounts_body
            .iter()
            .find(|account| account["chain"] == "ethereum")
            .unwrap()["address"],
        "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf"
    );
    assert_eq!(export_body["secret_kind"], "private_key");
    assert_eq!(export_body["label"], "Key Wallet");
    assert!(!imported.body_json.contains(private_key));
    assert!(!export.body_json.contains(private_key));
    assert!(!export.body_json.contains("master-password"));
}

#[test]
fn keystore_import_command_imports_exported_wallet() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let db_path = db_path.to_string_lossy();
    response(&format!(
        r#"{{"command":"set_master_password","db_path":"{db_path}","password":"master-password"}}"#
    ));
    let create = response(&format!(
        r#"{{"command":"create_wallet","db_path":"{db_path}","label":"Primary","mnemonic":"abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about","password":"master-password"}}"#
    ));
    let wallet: Value = serde_json::from_str(&create.body_json).unwrap();
    let wallet_id = wallet["id"].as_str().unwrap();
    let export = response(&format!(
        r#"{{"command":"export_keystore","db_path":"{db_path}","wallet_id":"{wallet_id}","password":"master-password"}}"#
    ));
    let import = response(&format!(
        r#"{{"command":"import_keystore","db_path":"{db_path}","label":"Imported","keystore_json":{},"keystore_password":"master-password","password":"master-password"}}"#,
        serde_json::to_string(&export.body_json).unwrap()
    ));
    let imported: Value = serde_json::from_str(&import.body_json).unwrap();
    let imported_id = imported["id"].as_str().unwrap();
    let accounts = response(&format!(
        r#"{{"command":"list_accounts","db_path":"{db_path}","wallet_id":"{imported_id}"}}"#
    ));
    let accounts_body: Vec<Value> = serde_json::from_str(&accounts.body_json).unwrap();

    assert!(export.ok);
    assert!(import.ok);
    assert!(accounts.ok);
    assert_eq!(imported["label"], "Imported");
    assert_ne!(imported["id"], wallet["id"]);
    assert_eq!(accounts_body.len(), 7);
    assert!(!import.body_json.contains("master-password"));
}

#[test]
fn delete_wallet_command_removes_only_selected_wallet() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let db_path = db_path.to_string_lossy();
    response(&format!(
        r#"{{"command":"set_master_password","db_path":"{db_path}","password":"master-password"}}"#
    ));
    let first = response(&format!(
        r#"{{"command":"create_wallet","db_path":"{db_path}","label":"Primary","mnemonic":"abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about","password":"master-password"}}"#
    ));
    response(&format!(
        r#"{{"command":"create_wallet","db_path":"{db_path}","label":"Trading","mnemonic":"abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about","password":"master-password"}}"#
    ));
    let first: Value = serde_json::from_str(&first.body_json).unwrap();
    let first_id = first["id"].as_str().unwrap();

    let deleted = response(&format!(
        r#"{{"command":"delete_wallet","db_path":"{db_path}","wallet_id":"{first_id}","password":"master-password"}}"#
    ));
    let wallets = response(&format!(
        r#"{{"command":"list_wallets","db_path":"{db_path}"}}"#
    ));
    let wallets_body: Vec<Value> = serde_json::from_str(&wallets.body_json).unwrap();

    assert!(deleted.ok);
    assert!(wallets.ok);
    assert_eq!(wallets_body.len(), 1);
    assert_eq!(wallets_body[0]["label"], "Trading");
}
