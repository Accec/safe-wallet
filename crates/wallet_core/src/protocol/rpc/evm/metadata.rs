use super::{abi, rpc};
use crate::error::WalletError;
use crate::models::{AssetKind, ChainId, TokenMetadata};
use reqwest::blocking::Client;

pub(super) fn fetch_token_metadata(
    client: &Client,
    chain: ChainId,
    rpc_url: &str,
    contract_address: &str,
) -> Result<TokenMetadata, WalletError> {
    let decimals = abi::decode_abi_u8(&rpc::eth_call(
        client,
        rpc_url,
        contract_address,
        "0x313ce567",
    )?)?;
    let symbol = abi::decode_abi_string(&rpc::eth_call(
        client,
        rpc_url,
        contract_address,
        "0x95d89b41",
    )?)?;
    let name = rpc::eth_call(client, rpc_url, contract_address, "0x06fdde03")
        .and_then(|result| abi::decode_abi_string(&result))
        .unwrap_or_else(|_| symbol.clone());
    Ok(TokenMetadata {
        chain,
        kind: AssetKind::Erc20,
        contract_address: contract_address.to_string(),
        symbol,
        name,
        decimals,
    })
}
