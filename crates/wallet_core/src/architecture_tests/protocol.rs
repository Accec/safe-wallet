#[test]
fn protocol_infrastructure_is_grouped_under_protocol_module() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_root = manifest_dir.join("src");

    for root_module in [
        "assets.rs",
        "discovery.rs",
        "indexers.rs",
        "network.rs",
        "qr.rs",
        "rpc.rs",
        "transactions.rs",
        "transfers.rs",
    ] {
        assert!(
            !source_root.join(root_module).exists(),
            "{root_module} should live under src/protocol instead of src root"
        );
    }

    let protocol_mod = std::fs::read_to_string(source_root.join("protocol/mod.rs"))
        .expect("protocol module should be a directory module");
    for module in [
        "assets",
        "discovery",
        "indexers",
        "network",
        "qr",
        "rpc",
        "transactions",
        "transfers",
    ] {
        assert!(
            protocol_mod.contains(&format!("pub mod {module};")),
            "protocol module should expose {module}"
        );
    }
}

#[test]
fn payment_uri_parsing_lives_under_protocol_with_root_reexport() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_root = manifest_dir.join("src");
    let lib = std::fs::read_to_string(source_root.join("lib.rs")).expect("crate root exists");

    assert!(
        !source_root.join("qr.rs").exists(),
        "payment URI parsing should live under protocol instead of the crate root"
    );
    assert!(
        source_root.join("protocol/qr/mod.rs").exists(),
        "protocol should own payment URI parsing"
    );
    assert!(
        lib.contains("pub use protocol::qr;"),
        "crate root should preserve wallet_core::qr through a protocol re-export"
    );
}

#[test]
fn payment_uri_parser_is_split_by_parsing_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let protocol_root = manifest_dir.join("src/protocol");
    let qr_root = protocol_root.join("qr");

    assert!(
        !protocol_root.join("qr.rs").exists(),
        "payment URI parsing should be a directory module split by URI flow, query decoding, validation, and tests"
    );

    for module in ["mod.rs", "query.rs", "validation.rs", "tests.rs"] {
        assert!(
            qr_root.join(module).exists(),
            "payment URI parser split is missing {module}"
        );
    }
}

#[test]
fn protocol_clients_are_split_into_focused_directory_modules() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let protocol_root = manifest_dir.join("src/protocol");

    for root_module in ["discovery.rs", "rpc.rs", "indexers.rs"] {
        assert!(
            !protocol_root.join(root_module).exists(),
            "{root_module} should be a directory module with focused implementations"
        );
    }

    for module in ["rpc/mod.rs", "rpc/evm/mod.rs", "rpc/tron/mod.rs"] {
        assert!(
            protocol_root.join(module).exists(),
            "rpc module should split shared traits from chain-specific clients: missing {module}"
        );
    }

    for module in [
        "indexers/mod.rs",
        "indexers/urls.rs",
        "indexers/parsing/mod.rs",
    ] {
        assert!(
            protocol_root.join(module).exists(),
            "indexer module should split transport, URL construction, and parsing: missing {module}"
        );
    }

    for module in [
        "discovery/mod.rs",
        "discovery/urls.rs",
        "discovery/parsing/mod.rs",
    ] {
        assert!(
            protocol_root.join(module).exists(),
            "discovery module should split transport, URL construction, and parsing: missing {module}"
        );
    }
}

#[test]
fn tron_rpc_client_is_split_by_protocol_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let rpc_root = manifest_dir.join("src/protocol/rpc");
    let tron_root = rpc_root.join("tron");

    assert!(
        !rpc_root.join("tron.rs").exists(),
        "Tron RPC client should split REST URL handling, address/calldata encoding, RPC calls, balances, and tests"
    );

    for module in [
        "mod.rs",
        "address.rs",
        "balances.rs",
        "calldata.rs",
        "chains.rs",
        "rpc.rs",
        "tests.rs",
        "urls.rs",
    ] {
        assert!(
            tron_root.join(module).exists(),
            "Tron RPC split is missing {module}"
        );
    }
}

#[test]
fn evm_rpc_client_is_split_by_protocol_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let rpc_root = manifest_dir.join("src/protocol/rpc");
    let evm_root = rpc_root.join("evm");

    assert!(
        !rpc_root.join("evm.rs").exists(),
        "EVM RPC client should split chain support, JSON-RPC calls, balances, token metadata, ABI decoding, calldata, and tests"
    );

    for module in [
        "mod.rs",
        "abi.rs",
        "balances.rs",
        "calldata.rs",
        "chains.rs",
        "metadata.rs",
        "rpc.rs",
        "tests.rs",
    ] {
        assert!(
            evm_root.join(module).exists(),
            "EVM RPC split is missing {module}"
        );
    }
}

#[test]
fn asset_discovery_parsing_is_split_by_provider_shape() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let discovery_root = manifest_dir.join("src/protocol/discovery");
    let parsing_root = discovery_root.join("parsing");

    assert!(
        !discovery_root.join("parsing.rs").exists(),
        "asset discovery parsing should split common row handling from provider-specific token shapes"
    );

    for module in [
        "mod.rs",
        "common.rs",
        "etherscan.rs",
        "scan_pages.rs",
        "tronscan.rs",
        "tests.rs",
    ] {
        assert!(
            parsing_root.join(module).exists(),
            "asset discovery parsing split is missing {module}"
        );
    }
}

#[test]
fn indexer_activity_parsing_is_split_by_provider_shape() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let indexers_root = manifest_dir.join("src/protocol/indexers");
    let parsing_root = indexers_root.join("parsing");

    assert!(
        !indexers_root.join("parsing.rs").exists(),
        "activity indexer parsing should be split by common summary handling and provider-specific row shapes"
    );

    for module in [
        "mod.rs",
        "common.rs",
        "etherscan.rs",
        "tronscan.rs",
        "tests.rs",
    ] {
        assert!(
            parsing_root.join(module).exists(),
            "activity indexer parsing split is missing {module}"
        );
    }
}

#[test]
fn evm_transaction_broadcasting_is_split_by_protocol_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let transactions_root = manifest_dir.join("src/protocol/transactions");
    let evm_root = transactions_root.join("evm");

    assert!(
        !transactions_root.join("evm.rs").exists(),
        "EVM transaction broadcasting should be split by RPC, fees, balances, and signing"
    );

    for module in ["mod.rs", "balances.rs", "fees.rs", "rpc.rs", "signing.rs"] {
        assert!(
            evm_root.join(module).exists(),
            "EVM transaction responsibility is missing {module}"
        );
    }
}

#[test]
fn btc_transaction_broadcasting_is_split_by_protocol_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let transactions_root = manifest_dir.join("src/protocol/transactions");
    let btc_root = transactions_root.join("btc");

    assert!(
        !transactions_root.join("btc.rs").exists(),
        "BTC transaction broadcasting should be split by address parsing, RPC, UTXOs, transaction building, and signing"
    );

    for module in [
        "mod.rs",
        "address.rs",
        "builder.rs",
        "rpc.rs",
        "signing.rs",
        "utxos.rs",
    ] {
        assert!(
            btc_root.join(module).exists(),
            "BTC transaction responsibility is missing {module}"
        );
    }
}

#[test]
fn transaction_encoding_helpers_are_split_by_protocol_family() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let transactions_root = manifest_dir.join("src/protocol/transactions");
    let encoding_root = transactions_root.join("encoding");

    assert!(
        !transactions_root.join("encoding.rs").exists(),
        "transaction encoding helpers should be split by protocol family and numeric concerns"
    );

    for module in [
        "mod.rs",
        "chains.rs",
        "evm.rs",
        "numbers.rs",
        "padding.rs",
        "tests.rs",
        "tron.rs",
    ] {
        assert!(
            encoding_root.join(module).exists(),
            "transaction encoding responsibility is missing {module}"
        );
    }
}

#[test]
fn tron_transaction_broadcasting_is_split_by_protocol_responsibility() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let transactions_root = manifest_dir.join("src/protocol/transactions");
    let tron_root = transactions_root.join("tron");

    assert!(
        !transactions_root.join("tron.rs").exists(),
        "Tron transaction broadcasting should be split by balance checks, transaction building, RPC, signing, and tests"
    );

    for module in [
        "mod.rs",
        "balances.rs",
        "builder.rs",
        "rpc.rs",
        "signing.rs",
        "tests.rs",
    ] {
        assert!(
            tron_root.join(module).exists(),
            "Tron transaction responsibility is missing {module}"
        );
    }
}
