use crate::error::WalletError;
use crate::models::{ChainId, TokenMetadata};
use reqwest::blocking::Client;

mod abi;
mod balances;
mod calldata;
mod chains;
mod metadata;
mod rpc;

pub(super) fn supports_chain(chain: ChainId) -> bool {
    chains::supports_chain(chain)
}

pub(super) fn fetch_native_balance(
    client: &Client,
    chain: ChainId,
    rpc_url: &str,
    address: &str,
) -> Result<String, WalletError> {
    if !supports_chain(chain) {
        return Err(WalletError::NetworkUnavailable);
    }
    balances::fetch_native_balance(client, rpc_url, address)
}

pub(super) fn fetch_token_balance(
    client: &Client,
    chain: ChainId,
    rpc_url: &str,
    owner_address: &str,
    contract_address: &str,
    decimals: u8,
) -> Result<String, WalletError> {
    if !supports_chain(chain) {
        return Err(WalletError::InvalidTokenContract);
    }
    balances::fetch_token_balance(client, rpc_url, owner_address, contract_address, decimals)
}

pub(super) fn fetch_token_metadata(
    client: &Client,
    chain: ChainId,
    rpc_url: &str,
    contract_address: &str,
) -> Result<TokenMetadata, WalletError> {
    if !supports_chain(chain) {
        return Err(WalletError::InvalidTokenContract);
    }
    metadata::fetch_token_metadata(client, chain, rpc_url, contract_address)
}

#[cfg(test)]
mod tests;
