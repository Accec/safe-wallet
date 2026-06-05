#[test]
fn shared_models_are_split_by_domain() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_root = manifest_dir.join("src");
    let models_root = source_root.join("models");

    assert!(
        !source_root.join("models.rs").exists(),
        "shared models should be a directory module, not a cross-domain catch-all file"
    );

    for module in [
        "mod.rs",
        "activity.rs",
        "assets.rs",
        "chains.rs",
        "multisig.rs",
        "network.rs",
        "qr.rs",
        "transfers.rs",
        "wallets.rs",
    ] {
        assert!(
            models_root.join(module).exists(),
            "models responsibility is missing {module}"
        );
    }
}
