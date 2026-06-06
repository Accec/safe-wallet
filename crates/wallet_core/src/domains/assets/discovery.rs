use super::AssetsDomain;
use crate::domains::validation::validate_chain_address;
use crate::error::WalletError;
use crate::models::{Asset, ChainId, DiscoveredAsset};
use crate::protocol::discovery::{
    default_discovery_endpoint, AssetDiscoveryProvider, HttpAssetDiscoveryProvider,
};
use crate::protocol::rpc::{AssetBalanceClient, RpcNativeBalanceClient};
use uuid::Uuid;

impl AssetsDomain {
    pub fn discover_assets(
        &self,
        wallet_id: Uuid,
        chain: Option<ChainId>,
    ) -> Result<Vec<Asset>, WalletError> {
        let settings = self.database.network().network_privacy_settings()?;
        let provider = HttpAssetDiscoveryProvider::new(&settings)?;
        let balance_client = RpcNativeBalanceClient::new(&settings)?;
        match chain {
            Some(chain) => {
                self.discover_chain_assets_with(wallet_id, chain, &provider, &balance_client)
            }
            None => {
                let mut saved = Vec::new();
                for account in self.database.wallets().list_accounts(wallet_id)? {
                    if account.chain == ChainId::Btc {
                        continue;
                    }
                    saved.extend(self.discover_chain_assets_with(
                        wallet_id,
                        account.chain,
                        &provider,
                        &balance_client,
                    )?);
                }
                Ok(saved)
            }
        }
    }

    pub fn discover_chain_assets_with<D: AssetDiscoveryProvider, C: AssetBalanceClient>(
        &self,
        wallet_id: Uuid,
        chain: ChainId,
        provider: &D,
        balance_client: &C,
    ) -> Result<Vec<Asset>, WalletError> {
        if chain == ChainId::Btc {
            return Ok(Vec::new());
        }
        let account = self
            .database
            .wallets()
            .account_for_chain(wallet_id, chain)?;
        let settings = self.database.network().chain_settings(chain)?;
        let Some(endpoint) = settings
            .indexer_endpoint
            .as_deref()
            .or_else(|| default_discovery_endpoint(chain))
        else {
            return Ok(Vec::new());
        };
        let rpc_url = settings
            .user_rpc_url
            .as_deref()
            .unwrap_or(settings.default_rpc_url.as_str());
        let discovered = provider.discover_assets(chain, endpoint, &account.address)?;
        let mut saved_assets = Vec::new();
        for discovered in discovered {
            if validate_chain_address(chain, &discovered.contract_address).is_err() {
                continue;
            }
            let discovered = self.discovered_with_rpc_balance(
                discovered,
                rpc_url,
                &account.address,
                balance_client,
            );
            let asset = self
                .database
                .assets()
                .save_discovered_token(wallet_id, &discovered)?;
            saved_assets.push(asset);
        }
        Ok(saved_assets)
    }

    fn discovered_with_rpc_balance<C: AssetBalanceClient>(
        &self,
        mut discovered: DiscoveredAsset,
        rpc_url: &str,
        owner_address: &str,
        balance_client: &C,
    ) -> DiscoveredAsset {
        if balance_client.supports_chain(discovered.chain) {
            if let Ok(metadata) = balance_client.fetch_token_metadata(
                discovered.chain,
                rpc_url,
                &discovered.contract_address,
            ) {
                discovered.kind = metadata.kind;
                discovered.symbol = metadata.symbol;
                discovered.name = metadata.name;
                discovered.decimals = metadata.decimals;
            }
            if let Ok(balance) = balance_client.fetch_token_balance(
                discovered.chain,
                rpc_url,
                owner_address,
                &discovered.contract_address,
                discovered.decimals,
            ) {
                discovered.balance = balance;
            }
        }
        discovered
    }
}
