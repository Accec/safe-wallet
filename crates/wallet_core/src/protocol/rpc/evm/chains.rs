use crate::models::ChainId;

pub(super) fn supports_chain(chain: ChainId) -> bool {
    matches!(
        chain,
        ChainId::Ethereum | ChainId::Bsc | ChainId::Polygon | ChainId::Arbitrum | ChainId::Optimism
    )
}
