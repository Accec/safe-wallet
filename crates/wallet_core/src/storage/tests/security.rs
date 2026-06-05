use crate::error::WalletError;
use crate::storage::WalletDatabase;

#[test]
fn master_password_verifier_save_is_insert_only() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let database = WalletDatabase::new(&db_path);
    database.initialize().unwrap();
    let original =
        crate::domains::auth::security::create_master_password_verifier("master-password").unwrap();
    let replacement =
        crate::domains::auth::security::create_master_password_verifier("replacement-password")
            .unwrap();

    database
        .app_security()
        .save_master_password_verifier(&original)
        .unwrap();
    let error = database
        .app_security()
        .save_master_password_verifier(&replacement)
        .unwrap_err();
    let loaded = database
        .app_security()
        .load_master_password_verifier()
        .unwrap()
        .unwrap();

    assert_eq!(error, WalletError::MasterPasswordAlreadySet);
    crate::domains::auth::security::verify_master_password("master-password", &loaded).unwrap();
    assert_eq!(
        crate::domains::auth::security::verify_master_password("replacement-password", &loaded)
            .unwrap_err(),
        WalletError::InvalidPassword
    );
}
