use super::{calldata, rpc};
use crate::error::WalletError;
use reqwest::blocking::Client;

pub(super) fn fetch_native_balance(
    client: &Client,
    rpc_url: &str,
    address: &str,
) -> Result<String, WalletError> {
    let result = rpc::eth_get_balance(client, rpc_url, address)?;
    format_evm_wei(&result)
}

pub(super) fn fetch_token_balance(
    client: &Client,
    rpc_url: &str,
    owner_address: &str,
    contract_address: &str,
    decimals: u8,
) -> Result<String, WalletError> {
    let data = calldata::encode_balance_of(owner_address)?;
    let result = rpc::eth_call(client, rpc_url, contract_address, &data)?;
    super::super::units::format_hex_units(&result, decimals)
}

pub(super) fn format_evm_wei(hex_value: &str) -> Result<String, WalletError> {
    super::super::units::format_hex_units(hex_value, 18)
}
