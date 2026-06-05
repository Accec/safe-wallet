#[test]
fn crate_root_keeps_architecture_tests_in_dedicated_module() {
    let source = include_str!("../lib.rs");

    assert!(
        !source.contains("mod tests {"),
        "crate root should not inline architecture tests"
    );
}

#[test]
fn architecture_tests_are_split_by_boundary() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_root = manifest_dir.join("src");
    let tests_root = source_root.join("architecture_tests");

    assert!(
        !source_root.join("architecture_tests.rs").exists(),
        "architecture tests should be split by architectural boundary, not kept in one catch-all file"
    );

    for module in [
        "mod.rs",
        "application.rs",
        "chains.rs",
        "crate_root.rs",
        "domains.rs",
        "models.rs",
        "protocol.rs",
        "storage.rs",
    ] {
        assert!(
            tests_root.join(module).exists(),
            "architecture test boundary is missing {module}"
        );
    }
}

#[test]
fn exposes_version() {
    assert_eq!(crate::VERSION, "0.1.0");
}

#[test]
fn legacy_wallet_service_is_not_public_core_api() {
    let source = include_str!("../lib.rs");
    let public_service_module = ["pub mod ", "service;"].concat();

    assert!(!source.contains(&public_service_module));
}
