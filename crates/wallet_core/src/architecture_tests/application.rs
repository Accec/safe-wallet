use crate::storage::WalletDatabase;

#[test]
fn wallet_engine_is_the_application_facade() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let engine = crate::application::WalletEngine::new(WalletDatabase::new(&db_path));

    engine.initialize().unwrap();
    engine
        .auth()
        .set_master_password("master-password")
        .unwrap();
    let status = engine.app_status().unwrap();

    assert!(status.initialized);
    assert!(status.locked);
    assert!(!status.biometric_enabled);
}

#[test]
fn wallet_engine_does_not_embed_legacy_wallet_service() {
    let source = include_str!("../application/mod.rs");

    assert!(!source.contains("WalletService"));
    assert!(!source.contains("crate::service"));
}

#[test]
fn integration_tests_exercise_wallet_engine_not_legacy_service() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let legacy_service_tests = manifest_dir.join("src/service/tests.rs");
    assert!(
        !legacy_service_tests.exists(),
        "integration tests should live with application/domain boundaries, not service"
    );

    let application_tests = std::fs::read_to_string(manifest_dir.join("src/application/tests.rs"))
        .expect("application integration tests should exist");
    assert!(!application_tests.contains("WalletService"));
    assert!(!application_tests.contains("crate::service"));
}

#[test]
fn application_integration_tests_are_split_by_domain() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_root = manifest_dir.join("src/application/tests");
    for module in [
        "activity.rs",
        "assets.rs",
        "auth.rs",
        "multisig.rs",
        "network.rs",
        "transfers.rs",
        "wallets.rs",
    ] {
        assert!(
            test_root.join(module).exists(),
            "missing application test module {module}"
        );
    }

    let tests_entry = std::fs::read_to_string(manifest_dir.join("src/application/tests.rs"))
        .expect("application test module entry should exist");
    assert!(!tests_entry.contains("#[test]"));
}
