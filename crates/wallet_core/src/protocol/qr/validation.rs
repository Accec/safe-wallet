use crate::chains::btc::BtcValidator;
use crate::chains::evm::EvmValidator;
use crate::chains::tron::TronValidator;
use crate::chains::ChainAddressValidator;
use crate::error::WalletError;
use crate::models::ChainId;

pub(super) fn validate_chain_address(chain: ChainId, address: &str) -> Result<(), WalletError> {
    match chain {
        ChainId::Btc => validate_btc_address(address),
        ChainId::Ethereum
        | ChainId::Bsc
        | ChainId::Polygon
        | ChainId::Arbitrum
        | ChainId::Optimism => validate_evm_address(address),
        ChainId::Tron => validate_tron_address(address),
    }
}

pub(super) fn validate_btc_address(address: &str) -> Result<(), WalletError> {
    BtcValidator.validate_address(address)
}

pub(super) fn validate_evm_address(address: &str) -> Result<(), WalletError> {
    EvmValidator::new(ChainId::Ethereum).validate_address(address)
}

pub(super) fn validate_tron_address(address: &str) -> Result<(), WalletError> {
    TronValidator.validate_address(address)
}
