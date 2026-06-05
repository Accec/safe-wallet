#[test]
fn wallet_and_auth_support_code_lives_under_domains() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_root = manifest_dir.join("src");

    for root_module in ["accounts.rs", "keystore.rs", "mnemonic.rs", "security.rs"] {
        assert!(
            !source_root.join(root_module).exists(),
            "{root_module} should live under domains instead of src root"
        );
    }

    let auth_mod = std::fs::read_to_string(source_root.join("domains/auth/mod.rs"))
        .expect("auth domain should be a directory module");
    assert!(auth_mod.contains("pub(crate) mod security;"));

    let wallets_mod = std::fs::read_to_string(source_root.join("domains/wallets/mod.rs"))
        .expect("wallets domain should be a directory module");
    for module in ["accounts", "keystore", "mnemonic"] {
        assert!(
            wallets_mod.contains(&format!("pub(crate) mod {module};")),
            "wallets domain should own {module}"
        );
    }
}

#[test]
fn assets_domain_is_split_by_asset_workflow() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let domains_root = manifest_dir.join("src/domains");
    let assets_root = domains_root.join("assets");

    assert!(
        !domains_root.join("assets.rs").exists(),
        "assets domain should be a directory module split by asset workflow"
    );

    for module in [
        "mod.rs",
        "balances.rs",
        "discovery.rs",
        "listing.rs",
        "tokens.rs",
    ] {
        assert!(
            assets_root.join(module).exists(),
            "assets domain workflow is missing {module}"
        );
    }
}

#[test]
fn transfers_domain_is_split_by_transfer_workflow() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let domains_root = manifest_dir.join("src/domains");
    let transfers_root = domains_root.join("transfers");

    assert!(
        !domains_root.join("transfers.rs").exists(),
        "transfers domain should be a directory module split by preview, sending, and signing responsibilities"
    );

    for module in ["mod.rs", "preview.rs", "sending.rs", "signing.rs"] {
        assert!(
            transfers_root.join(module).exists(),
            "transfers domain workflow is missing {module}"
        );
    }
}

#[test]
fn network_domain_is_split_by_network_workflow() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let domains_root = manifest_dir.join("src/domains");
    let network_root = domains_root.join("network");

    assert!(
        !domains_root.join("network.rs").exists(),
        "network domain should be a directory module split by privacy, settings, and validation responsibilities"
    );

    for module in ["mod.rs", "privacy.rs", "settings.rs", "validation.rs"] {
        assert!(
            network_root.join(module).exists(),
            "network domain workflow is missing {module}"
        );
    }
}

#[test]
fn multisig_domain_is_split_by_multisig_workflow() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let domains_root = manifest_dir.join("src/domains");
    let multisig_root = domains_root.join("multisig");

    assert!(
        !domains_root.join("multisig.rs").exists(),
        "multisig domain should be a directory module split by account, proposal, and signature responsibilities"
    );

    for module in ["mod.rs", "accounts.rs", "proposals.rs", "signatures.rs"] {
        assert!(
            multisig_root.join(module).exists(),
            "multisig domain workflow is missing {module}"
        );
    }
}

#[test]
fn wallets_domain_is_split_by_wallet_workflow() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let wallets_root = manifest_dir.join("src/domains/wallets");
    let wallets_mod = std::fs::read_to_string(wallets_root.join("mod.rs"))
        .expect("wallets domain module should exist");

    for module in ["creation.rs", "imports.rs", "listing.rs", "secrets.rs"] {
        assert!(
            wallets_root.join(module).exists(),
            "wallets domain workflow is missing {module}"
        );
    }

    for implementation_detail in [
        "mnemonic::validate_mnemonic(",
        "keystore::encrypt_mnemonic(",
        "accounts::derive_default_accounts(",
        "serde_json::from_str::<KeystoreExport>",
        "KeystoreExport {",
    ] {
        assert!(
            !wallets_mod.contains(implementation_detail),
            "wallets facade should delegate workflow detail `{implementation_detail}`"
        );
    }
    assert!(
        wallets_mod.lines().count() < 130,
        "wallets facade should stay bounded"
    );
}

#[test]
fn wallet_keystore_is_split_by_secret_handling_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let wallets_root = manifest_dir.join("src/domains/wallets");
    let keystore_root = wallets_root.join("keystore");

    assert!(
        !wallets_root.join("keystore.rs").exists(),
        "wallet keystore should be split by model, crypto, key normalization, export mapping, and tests"
    );

    for module in [
        "mod.rs",
        "crypto.rs",
        "exports.rs",
        "keys.rs",
        "model.rs",
        "tests.rs",
    ] {
        assert!(
            keystore_root.join(module).exists(),
            "wallet keystore responsibility is missing {module}"
        );
    }
}

#[test]
fn wallet_accounts_are_split_by_derivation_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let wallets_root = manifest_dir.join("src/domains/wallets");
    let accounts_root = wallets_root.join("accounts");

    assert!(
        !wallets_root.join("accounts.rs").exists(),
        "wallet account derivation should be split by paths, address encoding, private keys, and tests"
    );

    for module in [
        "mod.rs",
        "addresses.rs",
        "derivation.rs",
        "private_keys.rs",
        "tests.rs",
    ] {
        assert!(
            accounts_root.join(module).exists(),
            "wallet account derivation responsibility is missing {module}"
        );
    }
}

#[test]
fn auth_security_is_split_by_password_crypto_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let auth_root = manifest_dir.join("src/domains/auth");
    let security_root = auth_root.join("security");

    assert!(
        !auth_root.join("security.rs").exists(),
        "auth security should be split by KDF, verifier model, verification logic, and tests"
    );

    for module in ["mod.rs", "kdf.rs", "model.rs", "verifier.rs", "tests.rs"] {
        assert!(
            security_root.join(module).exists(),
            "auth security responsibility is missing {module}"
        );
    }
}

#[test]
fn domains_use_repositories_instead_of_database_sql_methods() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let domains_root = manifest_dir.join("src/domains");
    let forbidden_calls = [
        ".save_master_password_verifier(",
        ".load_master_password_verifier(",
        ".biometric_enabled(",
        ".update_biometric_enabled(",
        ".wipe_all_wallet_data(",
        ".save_wallet(",
        ".list_wallets(",
        ".delete_wallet(",
        ".load_wallet_summary(",
        ".load_wallet_keystore(",
        ".list_accounts(",
        ".account_for_chain(",
        ".list_assets(",
        ".save_asset_balance(",
        ".save_custom_token(",
        ".save_discovered_token(",
        ".remove_custom_token(",
        ".find_asset(",
        ".network_privacy_settings(",
        ".save_network_privacy_settings(",
        ".list_network_settings(",
        ".chain_settings(",
        ".update_chain_rpc(",
        ".indexer_endpoint(",
        ".save_network_settings(",
        ".update_indexer_settings(",
        ".clear_indexer_settings(",
        ".save_local_transfer(",
        ".list_activity(",
        ".save_indexed_activity(",
        ".save_multisig_account(",
        ".list_multisig_accounts(",
        ".load_multisig_account(",
        ".list_multisig_owners(",
        ".save_multisig_proposal(",
        ".list_multisig_proposals(",
        ".load_multisig_proposal(",
        ".owner_weight_for_multisig(",
        ".save_multisig_signature(",
    ];

    for entry in walk_rs_files(&domains_root) {
        let source = std::fs::read_to_string(&entry).expect("domain source should exist");
        for call in forbidden_calls {
            let direct_database_call = format!("self.database{call}");
            assert!(
                !source.contains(&direct_database_call),
                "domain {:?} should use a typed repository instead of calling {call}",
                entry
            );
        }
    }
}

#[test]
fn network_domain_owns_logic_without_wallet_service_dependency() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let network_root = manifest_dir.join("src/domains/network");

    for entry in walk_rs_files(&network_root) {
        let source = std::fs::read_to_string(&entry).expect("network domain source should exist");
        assert!(!source.contains("WalletService"));
        assert!(!source.contains("crate::service"));
    }
}

#[test]
fn auth_domain_owns_logic_without_wallet_service_dependency() {
    let source = include_str!("../domains/auth/mod.rs");

    assert!(!source.contains("WalletService"));
    assert!(!source.contains("crate::service"));
}

#[test]
fn activity_domain_owns_logic_without_wallet_service_dependency() {
    let source = include_str!("../domains/activity.rs");

    assert!(!source.contains("WalletService"));
    assert!(!source.contains("crate::service"));
}

#[test]
fn wallets_domain_owns_logic_without_wallet_service_dependency() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let wallets_root = manifest_dir.join("src/domains/wallets");

    for entry in walk_rs_files(&wallets_root) {
        let source = std::fs::read_to_string(&entry).expect("wallet domain source should exist");
        assert!(!source.contains("WalletService"));
        assert!(!source.contains("crate::service"));
    }
}

#[test]
fn multisig_domain_owns_logic_without_wallet_service_dependency() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let multisig_root = manifest_dir.join("src/domains/multisig");

    for entry in walk_rs_files(&multisig_root) {
        let source = std::fs::read_to_string(&entry).expect("multisig domain source should exist");
        assert!(!source.contains("WalletService"));
        assert!(!source.contains("crate::service"));
    }
}

#[test]
fn assets_domain_owns_logic_without_wallet_service_dependency() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let assets_root = manifest_dir.join("src/domains/assets");

    for entry in walk_rs_files(&assets_root) {
        let source = std::fs::read_to_string(&entry).expect("asset domain source should exist");
        assert!(!source.contains("WalletService"));
        assert!(!source.contains("crate::service"));
    }
}

#[test]
fn transfers_domain_owns_logic_without_wallet_service_dependency() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let transfers_root = manifest_dir.join("src/domains/transfers");

    for entry in walk_rs_files(&transfers_root) {
        let source = std::fs::read_to_string(&entry).expect("transfer domain source should exist");
        assert!(!source.contains("WalletService"));
        assert!(!source.contains("crate::service"));
    }
}

fn walk_rs_files(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    let entries = std::fs::read_dir(root).expect("source directory exists");
    for entry in entries {
        let path = entry.expect("source entry should be readable").path();
        if path.is_dir() {
            files.extend(walk_rs_files(&path));
        } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    files
}
