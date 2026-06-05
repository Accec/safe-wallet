#[test]
fn v2_protocol_handler_does_not_route_through_legacy_commands() {
    let source = include_str!("../protocol.rs");

    assert!(!source.contains("WalletCommand"));
    assert!(!source.contains("v2_legacy_command_name"));
}

#[test]
fn v2_protocol_payload_shapes_are_split_from_dispatcher() {
    let source = include_str!("../protocol.rs");

    assert!(source.contains("mod payloads;"));
    assert!(!source.contains("struct PasswordPayload"));
    assert!(!source.contains("struct TransferPayload"));
}

#[test]
fn v2_protocol_payload_shapes_are_split_by_domain() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let protocol_root = manifest_dir.join("src/protocol");
    let payloads_root = protocol_root.join("payloads");

    assert!(
        !protocol_root.join("payloads.rs").exists(),
        "v2 protocol payload DTOs should be split by API domain"
    );

    for module in [
        "activity.rs",
        "app.rs",
        "assets.rs",
        "auth.rs",
        "common.rs",
        "mod.rs",
        "multisig.rs",
        "network.rs",
        "transfers.rs",
        "wallets.rs",
    ] {
        assert!(
            payloads_root.join(module).exists(),
            "v2 protocol payloads are missing {module}"
        );
    }
}

#[test]
fn v2_protocol_handlers_are_split_by_domain() {
    let source = include_str!("../protocol.rs");

    for module in [
        "activity",
        "app",
        "assets",
        "auth",
        "multisig",
        "network",
        "transfers",
        "wallets",
    ] {
        assert!(
            source.contains(&format!("mod {module};")),
            "v2 protocol handler module `{module}` should be split from protocol.rs"
        );
    }
    assert!(!source.contains("fn set_master_password"));
    assert!(!source.contains("fn list_assets"));
    assert!(!source.contains("fn send_transfer"));
    assert!(source.lines().count() < 180);
}

#[test]
fn legacy_command_shapes_are_split_from_dispatcher() {
    let source = include_str!("../legacy.rs");

    assert!(source.contains("mod command;"));
    assert!(!source.contains("enum WalletCommand"));
}

#[test]
fn legacy_dispatcher_stays_bounded() {
    let source = include_str!("../legacy.rs");

    assert!(source.lines().count() < 650);
}

#[test]
fn legacy_command_handlers_are_split_by_domain() {
    let source = include_str!("../legacy.rs");

    for module in [
        "activity",
        "app",
        "assets",
        "auth",
        "multisig",
        "network",
        "transfers",
        "wallets",
    ] {
        assert!(
            source.contains(&format!("mod {module};")),
            "legacy command handler module `{module}` should be split from the dispatcher"
        );
    }
    assert!(source.lines().count() < 260);
}

#[test]
fn ffi_tests_are_split_by_protocol_and_domain() {
    let source = include_str!("../tests.rs");

    for module in [
        "architecture",
        "assets",
        "auth",
        "multisig",
        "network",
        "protocol",
        "support",
        "transfers",
        "wallets",
        "workflow",
    ] {
        assert!(
            source.contains(&format!("mod {module};")),
            "wallet_ffi test module `{module}` should be split from tests.rs"
        );
    }
    assert!(!source.contains("#[test]"));
    assert!(!source.contains("fn wallet_local_data_commands_return_public_models"));
    assert!(source.lines().count() < 80);
}
