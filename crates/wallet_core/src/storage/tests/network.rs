use crate::storage::WalletDatabase;

#[test]
fn network_privacy_settings_persist_to_sqlite() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let database = WalletDatabase::new(&db_path);
    database.initialize().unwrap();

    let settings = crate::models::NetworkPrivacySettings {
        proxy_enabled: true,
        proxy_mode: crate::models::ProxyMode::Custom,
        proxy_url: Some("http://127.0.0.1:8080".to_string()),
    };
    database
        .network()
        .save_network_privacy_settings(&settings)
        .unwrap();

    let reopened = WalletDatabase::new(&db_path);
    reopened.initialize().unwrap();
    assert_eq!(
        reopened.network().network_privacy_settings().unwrap(),
        settings
    );
}
