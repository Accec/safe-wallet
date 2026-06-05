use crate::error::WalletError;
use crate::models::ChainId;

pub(in crate::protocol::transactions) fn expected_evm_chain_id(
    chain: ChainId,
) -> Result<u64, WalletError> {
    match chain {
        ChainId::Ethereum => Ok(1),
        ChainId::Bsc => Ok(56),
        ChainId::Polygon => Ok(137),
        ChainId::Arbitrum => Ok(42161),
        ChainId::Optimism => Ok(10),
        ChainId::Btc | ChainId::Tron => Err(WalletError::InvalidNetworkSettings),
    }
}

pub(in crate::protocol::transactions) fn evm_prefers_eip1559(chain: ChainId) -> bool {
    matches!(
        chain,
        ChainId::Ethereum | ChainId::Polygon | ChainId::Arbitrum | ChainId::Optimism
    )
}
