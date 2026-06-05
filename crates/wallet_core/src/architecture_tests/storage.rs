#[test]
fn storage_repositories_are_explicit_types_not_wallet_database_extensions() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let repositories_root = manifest_dir.join("src/storage/repositories");

    for entry in std::fs::read_dir(&repositories_root).expect("repository directory exists") {
        let entry = entry.expect("repository entry should be readable");
        if entry.path().extension().and_then(|value| value.to_str()) != Some("rs") {
            continue;
        }
        let source = std::fs::read_to_string(entry.path()).expect("repository source exists");
        assert!(
            !source.contains("impl WalletDatabase"),
            "repository files should define typed repositories instead of extending WalletDatabase"
        );
    }
}

#[test]
fn wallet_repository_is_split_by_storage_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let repositories_root = manifest_dir.join("src/storage/repositories");
    let wallets_root = repositories_root.join("wallets");

    assert!(
        !repositories_root.join("wallets.rs").exists(),
        "wallet repository should be a directory module, not a mixed query and command file"
    );

    for module in ["mod.rs", "commands/mod.rs", "queries.rs", "accounts.rs"] {
        assert!(
            wallets_root.join(module).exists(),
            "wallet repository responsibility is missing {module}"
        );
    }
}

#[test]
fn wallet_repository_commands_are_split_by_write_flow() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let wallets_root = manifest_dir.join("src/storage/repositories/wallets");
    let commands_root = wallets_root.join("commands");

    assert!(
        !wallets_root.join("commands.rs").exists(),
        "wallet command repository should be split by save and delete write flows"
    );

    for module in ["mod.rs", "delete.rs", "save.rs"] {
        assert!(
            commands_root.join(module).exists(),
            "wallet command repository write flow is missing {module}"
        );
    }
}

#[test]
fn asset_repository_is_split_by_storage_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let repositories_root = manifest_dir.join("src/storage/repositories");
    let assets_root = repositories_root.join("assets");

    assert!(
        !repositories_root.join("assets.rs").exists(),
        "asset repository should be a directory module, not a mixed token and balance file"
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
            "asset repository responsibility is missing {module}"
        );
    }
}

#[test]
fn activity_repository_is_split_by_query_and_indexed_write_flow() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let repositories_root = manifest_dir.join("src/storage/repositories");
    let activity_root = repositories_root.join("activity");

    assert!(
        !repositories_root.join("activity.rs").exists(),
        "activity repository should split listing and indexed activity writes"
    );

    for module in ["mod.rs", "listing.rs", "indexed_records.rs"] {
        assert!(
            activity_root.join(module).exists(),
            "activity repository flow is missing {module}"
        );
    }
}

#[test]
fn transfer_repository_is_split_by_write_flow() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let repositories_root = manifest_dir.join("src/storage/repositories");
    let transfers_root = repositories_root.join("transfers");

    assert!(
        !repositories_root.join("transfers.rs").exists(),
        "transfer repository should be a directory module for write flows"
    );

    for module in ["mod.rs", "local_records.rs"] {
        assert!(
            transfers_root.join(module).exists(),
            "transfer repository write flow is missing {module}"
        );
    }
}

#[test]
fn network_repository_is_split_by_storage_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let repositories_root = manifest_dir.join("src/storage/repositories");
    let network_root = repositories_root.join("network");

    assert!(
        !repositories_root.join("network.rs").exists(),
        "network repository should be a directory module, not a mixed settings file"
    );

    for module in ["mod.rs", "chains/mod.rs", "indexers.rs", "privacy.rs"] {
        assert!(
            network_root.join(module).exists(),
            "network repository responsibility is missing {module}"
        );
    }
}

#[test]
fn network_chain_repository_is_split_by_query_and_write_flow() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let network_root = manifest_dir.join("src/storage/repositories/network");
    let chains_root = network_root.join("chains");

    assert!(
        !network_root.join("chains.rs").exists(),
        "network chain repository should split list, lookup, and write flows"
    );

    for module in ["mod.rs", "listing.rs", "lookup.rs", "writes.rs"] {
        assert!(
            chains_root.join(module).exists(),
            "network chain repository flow is missing {module}"
        );
    }
}

#[test]
fn multisig_repository_is_split_by_storage_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let repositories_root = manifest_dir.join("src/storage/repositories");
    let multisig_root = repositories_root.join("multisig");

    assert!(
        !repositories_root.join("multisig.rs").exists(),
        "multisig repository should be a directory module, not a mixed accounts and proposals file"
    );

    for module in [
        "mod.rs",
        "accounts.rs",
        "owners.rs",
        "proposals.rs",
        "signatures.rs",
    ] {
        assert!(
            multisig_root.join(module).exists(),
            "multisig repository responsibility is missing {module}"
        );
    }
}

#[test]
fn app_security_repository_is_split_by_storage_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let repositories_root = manifest_dir.join("src/storage/repositories");
    let app_security_root = repositories_root.join("app_security");

    assert!(
        !repositories_root.join("app_security.rs").exists(),
        "app security repository should be a directory module split by verifier, biometrics, and wipe responsibilities"
    );

    for module in ["mod.rs", "biometrics.rs", "verifiers.rs", "wipe.rs"] {
        assert!(
            app_security_root.join(module).exists(),
            "app security repository responsibility is missing {module}"
        );
    }
}

#[test]
fn storage_mappers_are_split_by_mapping_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let storage_root = manifest_dir.join("src/storage");

    assert!(
        !storage_root.join("mappers.rs").exists(),
        "storage mappers should be a directory module, not a mixed responsibility file"
    );

    for module in [
        "mappers/mod.rs",
        "mappers/codecs.rs",
        "mappers/errors.rs",
        "mappers/primitives.rs",
        "mappers/rows.rs",
    ] {
        assert!(
            storage_root.join(module).exists(),
            "storage mapper responsibility is missing {module}"
        );
    }
}

#[test]
fn storage_schema_separates_bootstrap_from_compatibility_upgrades() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let storage_root = manifest_dir.join("src/storage");

    assert!(
        !storage_root.join("schema.rs").exists(),
        "schema initialization should be a directory module, not a mixed DDL and compatibility file"
    );

    for module in [
        "schema/mod.rs",
        "schema/bootstrap/mod.rs",
        "schema/compatibility.rs",
    ] {
        assert!(
            storage_root.join(module).exists(),
            "schema responsibility is missing {module}"
        );
    }
}

#[test]
fn storage_schema_bootstrap_is_split_by_storage_domain() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let schema_root = manifest_dir.join("src/storage/schema");
    let bootstrap_root = schema_root.join("bootstrap");

    assert!(
        !schema_root.join("bootstrap.rs").exists(),
        "schema bootstrap should be split by storage domain instead of keeping all DDL in one file"
    );

    for module in [
        "mod.rs",
        "executor.rs",
        "migrations.rs",
        "security.rs",
        "wallets.rs",
        "network.rs",
        "assets.rs",
        "activity.rs",
        "multisig.rs",
        "preferences.rs",
        "indexes.rs",
    ] {
        assert!(
            bootstrap_root.join(module).exists(),
            "schema bootstrap responsibility is missing {module}"
        );
    }
}

#[test]
fn storage_tests_are_split_by_storage_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let storage_root = manifest_dir.join("src/storage");

    assert!(
        !storage_root.join("tests.rs").exists(),
        "storage tests should be split by schema, constraints, and repository smoke coverage"
    );

    for module in [
        "tests/mod.rs",
        "tests/schema.rs",
        "tests/constraints.rs",
        "tests/security.rs",
        "tests/network.rs",
        "tests/support.rs",
    ] {
        assert!(
            storage_root.join(module).exists(),
            "storage test responsibility is missing {module}"
        );
    }
}
