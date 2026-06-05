#[test]
fn chains_module_splits_registry_defaults_from_validation_contracts() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let chains_root = manifest_dir.join("src/chains");
    let module_root = std::fs::read_to_string(chains_root.join("mod.rs"))
        .expect("chains module root should exist");

    for module in ["defaults.rs", "validation.rs", "tests.rs"] {
        assert!(
            chains_root.join(module).exists(),
            "chains module responsibility is missing {module}"
        );
    }

    assert!(
        !module_root.contains("fn settings("),
        "chains root should not own default chain registry construction details"
    );
    assert!(
        !module_root.contains("trait ChainAddressValidator"),
        "chains root should re-export validation contracts instead of defining them inline"
    );
}
