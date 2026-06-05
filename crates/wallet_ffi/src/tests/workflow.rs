use serde_json::{json, Value};

use super::support::{response, TestRpcServer};

#[test]
fn wallet_local_data_commands_return_public_models() {
    let rpc = TestRpcServer::evm();
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
    let accounts = response(&format!(
        r#"{{"command":"list_accounts","db_path":"{db_path}","wallet_id":"{wallet_id}"}}"#
    ));
    let assets = response(&format!(
        r#"{{"command":"list_assets","db_path":"{db_path}","wallet_id":"{wallet_id}"}}"#
    ));
    let token = response(&format!(
        r#"{{"command":"add_custom_token","db_path":"{db_path}","chain":"tron","contract_address":"TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7","token_name":"TRC"}}"#
    ));
    let network = response(&format!(
        r#"{{"command":"save_network_settings","db_path":"{db_path}","network_name":"BNB Smart Chain","rpc_url":"https://example.invalid/rpc","chain_id":"56","currency_symbol":"BNB","block_explorer_url":"https://example.invalid/explorer"}}"#
    ));
    let rpc_update = response(&format!(
        r#"{{"command":"update_chain_rpc","db_path":"{db_path}","chain":"ethereum","rpc_url":"{}"}}"#,
        rpc.url
    ));
    let assets_body: Vec<Value> = serde_json::from_str(&assets.body_json).unwrap();
    let eth_asset = assets_body
        .iter()
        .find(|asset| asset["chain"] == "ethereum")
        .unwrap();
    let request = json!({
        "wallet_id": wallet_id,
        "chain": "ethereum",
        "asset_id": eth_asset["id"],
        "to_address": "0x0000000000000000000000000000000000000000",
        "amount": "1"
    });
    let preview = response(&format!(
        r#"{{"command":"preview_transfer","db_path":"{db_path}","request_json":{}}}"#,
        serde_json::to_string(&request.to_string()).unwrap()
    ));
    let send = response(&format!(
        r#"{{"command":"send_transfer","db_path":"{db_path}","request_json":{},"password":"master-password"}}"#,
        serde_json::to_string(&request.to_string()).unwrap()
    ));
    let reveal = response(&format!(
        r#"{{"command":"reveal_mnemonic","db_path":"{db_path}","wallet_id":"{wallet_id}","password":"master-password"}}"#
    ));
    let activity = response(&format!(
        r#"{{"command":"list_activity","db_path":"{db_path}","wallet_id":"{wallet_id}"}}"#
    ));
    let accounts_body: Vec<Value> = serde_json::from_str(&accounts.body_json).unwrap();
    let token_body: Value = serde_json::from_str(&token.body_json).unwrap();
    let remove_token = response(&format!(
        r#"{{"command":"remove_custom_token","db_path":"{db_path}","asset_id":"{}"}}"#,
        token_body["id"].as_str().unwrap()
    ));
    let assets_after_remove = response(&format!(
        r#"{{"command":"list_assets","db_path":"{db_path}","wallet_id":"{wallet_id}"}}"#
    ));
    let preview_body: Value = serde_json::from_str(&preview.body_json).unwrap();
    let send_body: Value = serde_json::from_str(&send.body_json).unwrap();
    let reveal_body: Value = serde_json::from_str(&reveal.body_json).unwrap();
    let activity_body: Vec<Value> = serde_json::from_str(&activity.body_json).unwrap();
    let assets_after_remove_body: Vec<Value> =
        serde_json::from_str(&assets_after_remove.body_json).unwrap();

    assert!(accounts.ok);
    assert!(assets.ok);
    assert!(token.ok);
    assert!(remove_token.ok);
    assert!(assets_after_remove.ok);
    assert!(network.ok);
    assert!(rpc_update.ok);
    assert!(preview.ok);
    assert!(send.ok);
    assert!(reveal.ok);
    assert!(activity.ok);
    assert_eq!(accounts_body.len(), 7);
    assert_eq!(token_body["symbol"], "TRC");
    assert!(assets_after_remove_body
        .iter()
        .all(|asset| asset["id"] != token_body["id"]));
    assert_eq!(preview_body["asset_symbol"], eth_asset["symbol"]);
    assert_eq!(send_body["status"], "broadcasted");
    assert_eq!(send_body["tx_hash"], "0xffibroadcast");
    assert!(rpc.saw_method("eth_sendRawTransaction"));
    assert_eq!(reveal_body["mnemonic"], "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about");
    assert_eq!(activity_body.len(), 1);
    assert_eq!(activity_body[0]["tx_hash"], send_body["tx_hash"]);
    assert!(!preview.body_json.contains("mnemonic"));
    assert!(!preview.body_json.contains("master-password"));
    assert!(!send.body_json.contains("mnemonic"));
    assert!(!send.body_json.contains("master-password"));
}
