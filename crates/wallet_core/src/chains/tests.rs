use super::*;
use crate::models::{ChainId, ChainSettings};

#[test]
fn default_settings_include_supported_mainnets() {
    let settings = default_chain_settings();

    assert_eq!(settings.len(), 7);
    assert_settings(
        &settings,
        ChainId::Btc,
        "Bitcoin",
        None,
        "https://blockstream.info/api",
        None,
        "BTC",
        8,
    );
    assert_settings(
        &settings,
        ChainId::Ethereum,
        "Ethereum",
        Some("1"),
        "https://ethereum-rpc.publicnode.com",
        Some("https://etherscan.io"),
        "ETH",
        18,
    );
    assert_settings(
        &settings,
        ChainId::Bsc,
        "BNB Smart Chain",
        Some("56"),
        "https://bsc-rpc.publicnode.com",
        Some("https://bscscan.com"),
        "BNB",
        18,
    );
    assert_settings(
        &settings,
        ChainId::Polygon,
        "Polygon",
        Some("137"),
        "https://polygon-bor-rpc.publicnode.com",
        Some("https://polygonscan.com"),
        "POL",
        18,
    );
    assert_settings(
        &settings,
        ChainId::Arbitrum,
        "Arbitrum One",
        Some("42161"),
        "https://arbitrum-one-rpc.publicnode.com",
        Some("https://arbiscan.io"),
        "ETH",
        18,
    );
    assert_settings(
        &settings,
        ChainId::Optimism,
        "OP Mainnet",
        Some("10"),
        "https://optimism-rpc.publicnode.com",
        Some("https://optimistic.etherscan.io"),
        "ETH",
        18,
    );
    assert_settings(
        &settings,
        ChainId::Tron,
        "Tron",
        Some("728126428"),
        "https://tron-rpc.publicnode.com",
        Some("https://tronscan.org"),
        "TRX",
        6,
    );
    assert!(settings
        .iter()
        .all(|setting| setting.enabled && setting.user_rpc_url.is_none()));
}

fn assert_settings(
    settings: &[ChainSettings],
    chain: ChainId,
    network_name: &str,
    chain_id: Option<&str>,
    default_rpc_url: &str,
    explorer_url: Option<&str>,
    native_symbol: &str,
    native_decimals: u8,
) {
    let setting = settings
        .iter()
        .find(|setting| setting.chain == chain)
        .unwrap();
    assert_eq!(setting.network_name, network_name);
    assert_eq!(setting.chain_id.as_deref(), chain_id);
    assert_eq!(setting.default_rpc_url, default_rpc_url);
    assert_eq!(setting.explorer_url.as_deref(), explorer_url);
    assert_eq!(setting.native_symbol, native_symbol);
    assert_eq!(setting.native_decimals, native_decimals);
}
