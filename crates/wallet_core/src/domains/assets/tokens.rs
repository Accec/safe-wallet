use super::AssetsDomain;
use crate::domains::validation::validate_chain_address;
use crate::error::WalletError;
use crate::models::{Asset, AssetKind, ChainId, TokenMetadata};
use crate::protocol::rpc::{AssetBalanceClient, RpcNativeBalanceClient};
use uuid::Uuid;

impl AssetsDomain {
    pub fn add_custom_token(
        &self,
        chain: ChainId,
        contract_address: &str,
        token_name: &str,
    ) -> Result<Asset, WalletError> {
        let settings = self.database.network().network_privacy_settings()?;
        let client = RpcNativeBalanceClient::new(&settings)?;
        self.add_custom_token_with(&client, chain, contract_address, token_name)
    }

    pub fn add_custom_token_with<C: AssetBalanceClient>(
        &self,
        client: &C,
        chain: ChainId,
        contract_address: &str,
        token_name: &str,
    ) -> Result<Asset, WalletError> {
        validate_chain_address(chain, contract_address)
            .map_err(|_| WalletError::InvalidTokenContract)?;
        let token_name = token_name.trim();
        if token_name.is_empty() {
            return Err(WalletError::InvalidTokenContract);
        }
        let kind = if chain == ChainId::Tron {
            AssetKind::Trc20
        } else if chain == ChainId::Btc {
            return Err(WalletError::InvalidTokenContract);
        } else {
            AssetKind::Erc20
        };
        if kind == AssetKind::Erc20 && client.supports_chain(chain) {
            let settings = self.database.network().chain_settings(chain)?;
            let rpc_url = settings
                .user_rpc_url
                .as_deref()
                .unwrap_or(settings.default_rpc_url.as_str());
            let mut metadata = client.fetch_token_metadata(chain, rpc_url, contract_address)?;
            metadata.name = token_name.to_string();
            metadata.contract_address = contract_address.to_string();
            return self.database.assets().save_custom_token(&metadata);
        }
        self.database.assets().save_custom_token(&TokenMetadata {
            chain,
            kind,
            contract_address: contract_address.to_string(),
            symbol: token_name.to_string(),
            name: token_name.to_string(),
            decimals: if chain == ChainId::Tron { 6 } else { 18 },
        })
    }

    pub fn remove_custom_token(&self, asset_id: Uuid) -> Result<(), WalletError> {
        self.database.assets().remove_custom_token(asset_id)
    }
}
