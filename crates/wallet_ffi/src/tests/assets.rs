use serde_json::Value;

use super::support::response;

#[test]
fn discover_assets_command_returns_count_without_auto_refreshing() {
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

    let discovered = response(&format!(
        r#"{{"command":"discover_assets","db_path":"{db_path}","wallet_id":"{wallet_id}","chain":"btc"}}"#
    ));
    let body: Value = serde_json::from_str(&discovered.body_json).unwrap();

    assert!(discovered.ok);
    assert_eq!(body["discovered"], 0);
}
