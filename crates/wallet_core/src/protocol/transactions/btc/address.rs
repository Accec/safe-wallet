use crate::error::WalletError;
use bitcoin::{Address, Network};
use std::str::FromStr;

pub(super) fn address(address: &str) -> Result<Address, WalletError> {
    Address::from_str(address)
        .map_err(|_| WalletError::InvalidAddress)?
        .require_network(Network::Bitcoin)
        .map_err(|_| WalletError::InvalidAddress)
}
