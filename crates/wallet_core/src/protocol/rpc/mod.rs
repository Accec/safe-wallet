use crate::error::WalletError;
use crate::models::{ChainId, TokenMetadata};
use reqwest::blocking::Client;
use std::time::Duration;

mod evm;
mod tron;
mod units;

pub trait AssetBalanceClient {
    fn supports_chain(&self, chain: ChainId) -> bool;

    fn fetch_native_balance(
        &self,
        chain: ChainId,
        rpc_url: &str,
        address: &str,
    ) -> Result<String, WalletError>;

    fn fetch_token_balance(
        &self,
        chain: ChainId,
        rpc_url: &str,
        owner_address: &str,
        contract_address: &str,
        decimals: u8,
    ) -> Result<String, WalletError>;

    fn fetch_token_metadata(
        &self,
        _chain: ChainId,
        _rpc_url: &str,
        _contract_address: &str,
    ) -> Result<TokenMetadata, WalletError> {
        Err(WalletError::InvalidTokenContract)
    }
}

pub struct RpcNativeBalanceClient {
    client: Client,
}

impl RpcNativeBalanceClient {
    pub fn new(settings: &crate::models::NetworkPrivacySettings) -> Result<Self, WalletError> {
        let client =
            crate::protocol::network::build_http_client(settings, Duration::from_secs(12))?;
        Ok(Self { client })
    }
}

impl AssetBalanceClient for RpcNativeBalanceClient {
    fn supports_chain(&self, chain: ChainId) -> bool {
        evm::supports_chain(chain) || tron::supports_chain(chain)
    }

    fn fetch_native_balance(
        &self,
        chain: ChainId,
        rpc_url: &str,
        address: &str,
    ) -> Result<String, WalletError> {
        if tron::supports_chain(chain) {
            return tron::fetch_native_balance(&self.client, rpc_url, address);
        }
        evm::fetch_native_balance(&self.client, chain, rpc_url, address)
    }

    fn fetch_token_balance(
        &self,
        chain: ChainId,
        rpc_url: &str,
        owner_address: &str,
        contract_address: &str,
        decimals: u8,
    ) -> Result<String, WalletError> {
        if tron::supports_chain(chain) {
            return tron::fetch_token_balance(
                &self.client,
                rpc_url,
                owner_address,
                contract_address,
                decimals,
            );
        }
        evm::fetch_token_balance(
            &self.client,
            chain,
            rpc_url,
            owner_address,
            contract_address,
            decimals,
        )
    }

    fn fetch_token_metadata(
        &self,
        chain: ChainId,
        rpc_url: &str,
        contract_address: &str,
    ) -> Result<TokenMetadata, WalletError> {
        evm::fetch_token_metadata(&self.client, chain, rpc_url, contract_address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rpc_client_accepts_tor_proxy_settings() {
        let client = RpcNativeBalanceClient::new(&crate::models::NetworkPrivacySettings {
            proxy_enabled: true,
            proxy_mode: crate::models::ProxyMode::Tor,
            proxy_url: None,
        });

        assert!(client.is_ok());
    }
}
