use super::AssetsDomain;
use crate::error::WalletError;
use crate::models::{AssetKind, ChainId};
use crate::protocol::rpc::{AssetBalanceClient, RpcNativeBalanceClient};
use uuid::Uuid;

impl AssetsDomain {
    pub fn refresh_balances(
        &self,
        wallet_id: Uuid,
        chain: Option<ChainId>,
    ) -> Result<(), WalletError> {
        match chain {
            Some(chain) => self.refresh_chain_balances(wallet_id, chain),
            None => self.refresh_native_balances(wallet_id),
        }
    }

    pub fn refresh_native_balances(&self, wallet_id: Uuid) -> Result<(), WalletError> {
        let settings = self.database.network().network_privacy_settings()?;
        let client = RpcNativeBalanceClient::new(&settings)?;
        self.refresh_native_balances_with(wallet_id, &client)
    }

    pub fn refresh_chain_balances(
        &self,
        wallet_id: Uuid,
        chain: ChainId,
    ) -> Result<(), WalletError> {
        let settings = self.database.network().network_privacy_settings()?;
        let client = RpcNativeBalanceClient::new(&settings)?;
        self.refresh_chain_balances_with(wallet_id, chain, &client)
    }

    pub fn refresh_native_balances_with<C: AssetBalanceClient>(
        &self,
        wallet_id: Uuid,
        client: &C,
    ) -> Result<(), WalletError> {
        self.refresh_balances_with(wallet_id, None, client)
    }

    pub fn refresh_chain_balances_with<C: AssetBalanceClient>(
        &self,
        wallet_id: Uuid,
        chain: ChainId,
        client: &C,
    ) -> Result<(), WalletError> {
        self.refresh_balances_with(wallet_id, Some(chain), client)
    }

    fn refresh_balances_with<C: AssetBalanceClient>(
        &self,
        wallet_id: Uuid,
        selected_chain: Option<ChainId>,
        client: &C,
    ) -> Result<(), WalletError> {
        let accounts = self.database.wallets().list_accounts(wallet_id)?;
        let assets = self.database.assets().list_assets(wallet_id)?;
        for account in accounts {
            if selected_chain.is_some_and(|chain| account.chain != chain) {
                continue;
            }
            if !client.supports_chain(account.chain) {
                continue;
            }
            let settings = self.database.network().chain_settings(account.chain)?;
            let rpc_url = settings
                .user_rpc_url
                .as_deref()
                .unwrap_or(settings.default_rpc_url.as_str());
            for asset in assets.iter().filter(|asset| asset.chain == account.chain) {
                let balance = match asset.kind {
                    AssetKind::Native => {
                        client.fetch_native_balance(account.chain, rpc_url, &account.address)?
                    }
                    AssetKind::Erc20 | AssetKind::Trc20 => {
                        let Some(contract_address) = asset.contract_address.as_deref() else {
                            continue;
                        };
                        client.fetch_token_balance(
                            account.chain,
                            rpc_url,
                            &account.address,
                            contract_address,
                            asset.decimals,
                        )?
                    }
                };
                self.database.assets().save_asset_balance(
                    wallet_id,
                    account.chain,
                    asset.id,
                    &balance,
                    "rpc",
                )?;
            }
        }
        Ok(())
    }
}
