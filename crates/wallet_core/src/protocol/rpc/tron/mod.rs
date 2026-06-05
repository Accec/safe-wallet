use crate::error::WalletError;
use crate::models::ChainId;
use reqwest::blocking::Client;

mod address;
mod balances;
mod calldata;
mod chains;
mod rpc;
mod urls;

pub(super) fn supports_chain(chain: ChainId) -> bool {
    chains::supports_chain(chain)
}

pub(super) fn fetch_native_balance(
    client: &Client,
    rpc_url: &str,
    address: &str,
) -> Result<String, WalletError> {
    balances::fetch_native_balance(client, rpc_url, address)
}

pub(super) fn fetch_token_balance(
    client: &Client,
    rpc_url: &str,
    owner_address: &str,
    contract_address: &str,
    decimals: u8,
) -> Result<String, WalletError> {
    balances::fetch_token_balance(client, rpc_url, owner_address, contract_address, decimals)
}

#[cfg(test)]
mod tests;
