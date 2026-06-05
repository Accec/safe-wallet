use serde_json::Value;

use super::support::response;

#[test]
fn settings_commands_update_local_configuration() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let db_path = db_path.to_string_lossy();
    response(&format!(
        r#"{{"command":"set_master_password","db_path":"{db_path}","password":"master-password"}}"#
    ));
    response(&format!(
        r#"{{"command":"create_wallet","db_path":"{db_path}","label":"Primary","mnemonic":"abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about","password":"master-password"}}"#
    ));
    let rpc_update = response(&format!(
        r#"{{"command":"update_chain_rpc","db_path":"{db_path}","chain":"ethereum","rpc_url":"https://example.invalid/rpc"}}"#
    ));
    let indexer_update = response(&format!(
        r#"{{"command":"update_indexer_settings","db_path":"{db_path}","chain":"ethereum","endpoint":"https://example.invalid/indexer","api_key":null}}"#
    ));
    let networks = response(&format!(
        r#"{{"command":"list_network_settings","db_path":"{db_path}"}}"#
    ));
    let networks_body: Vec<Value> = serde_json::from_str(&networks.body_json).unwrap();
    let ethereum = networks_body
        .iter()
        .find(|network| network["chain"] == "ethereum")
        .unwrap();

    assert!(rpc_update.ok);
    assert!(indexer_update.ok);
    assert!(networks.ok);
    assert_eq!(ethereum["user_rpc_url"], "https://example.invalid/rpc");
}

#[test]
fn network_privacy_commands_save_and_load_proxy_settings() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let db_path = db_path.to_string_lossy();

    let save = response(&format!(
        r#"{{"command":"save_network_privacy_settings","db_path":"{db_path}","proxy_enabled":true,"proxy_mode":"tor","proxy_url":null}}"#
    ));
    assert!(save.ok);

    let loaded = response(&format!(
        r#"{{"command":"get_network_privacy_settings","db_path":"{db_path}"}}"#
    ));
    assert!(loaded.ok);
    let body: Value = serde_json::from_str(&loaded.body_json).unwrap();
    assert_eq!(body["proxy_enabled"], true);
    assert_eq!(body["proxy_mode"], "tor");
    assert_eq!(body["proxy_url"], "socks5h://127.0.0.1:9050");
}

#[test]
fn invalid_proxy_settings_return_safe_error() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let db_path = db_path.to_string_lossy();

    let response = response(&format!(
        r#"{{"command":"save_network_privacy_settings","db_path":"{db_path}","proxy_enabled":true,"proxy_mode":"custom","proxy_url":"ftp://127.0.0.1:21"}}"#
    ));

    assert!(!response.ok);
    assert_eq!(response.error.as_deref(), Some("Invalid proxy settings"));
}
