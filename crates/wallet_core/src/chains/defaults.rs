use crate::models::{ChainId, ChainSettings};

pub const TRON_PUBLICNODE_RPC_URL: &str = "https://tron-rpc.publicnode.com";
pub const LEGACY_TRONGRID_RPC_URL: &str = "https://api.trongrid.io";

pub fn default_chain_settings() -> Vec<ChainSettings> {
    vec![
        settings(
            ChainId::Btc,
            "Bitcoin",
            None,
            "https://blockstream.info/api",
            None,
            "BTC",
            8,
        ),
        settings(
            ChainId::Ethereum,
            "Ethereum",
            Some("1"),
            "https://ethereum-rpc.publicnode.com",
            Some("https://etherscan.io"),
            "ETH",
            18,
        ),
        settings(
            ChainId::Bsc,
            "BNB Smart Chain",
            Some("56"),
            "https://bsc-rpc.publicnode.com",
            Some("https://bscscan.com"),
            "BNB",
            18,
        ),
        settings(
            ChainId::Polygon,
            "Polygon",
            Some("137"),
            "https://polygon-bor-rpc.publicnode.com",
            Some("https://polygonscan.com"),
            "POL",
            18,
        ),
        settings(
            ChainId::Arbitrum,
            "Arbitrum One",
            Some("42161"),
            "https://arbitrum-one-rpc.publicnode.com",
            Some("https://arbiscan.io"),
            "ETH",
            18,
        ),
        settings(
            ChainId::Optimism,
            "OP Mainnet",
            Some("10"),
            "https://optimism-rpc.publicnode.com",
            Some("https://optimistic.etherscan.io"),
            "ETH",
            18,
        ),
        settings(
            ChainId::Tron,
            "Tron",
            Some("728126428"),
            TRON_PUBLICNODE_RPC_URL,
            Some("https://tronscan.org"),
            "TRX",
            6,
        ),
    ]
}

fn settings(
    chain: ChainId,
    network_name: &str,
    chain_id: Option<&str>,
    default_rpc_url: &str,
    explorer_url: Option<&str>,
    native_symbol: &str,
    native_decimals: u8,
) -> ChainSettings {
    ChainSettings {
        chain,
        network_name: network_name.to_string(),
        chain_id: chain_id.map(str::to_string),
        enabled: true,
        default_rpc_url: default_rpc_url.to_string(),
        user_rpc_url: None,
        indexer_endpoint: None,
        explorer_url: explorer_url.map(str::to_string),
        native_symbol: native_symbol.to_string(),
        native_decimals,
    }
}
