use crate::error::WalletError;
use crate::models::{Asset, ChainId, NetworkPrivacySettings, TransferPreview};
mod amount;
mod btc;
mod encoding;
mod evm;
mod tron;

use k256::ecdsa::SigningKey;
use reqwest::blocking::Client;
use std::time::Duration;

pub struct BroadcastedTransaction {
    pub tx_hash: String,
}

pub struct TransferBroadcastDraft<'a> {
    pub chain: ChainId,
    pub rpc_url: &'a str,
    pub from_address: &'a str,
    pub to_address: &'a str,
    pub amount: &'a str,
    pub asset: &'a Asset,
    pub signing_key: &'a SigningKey,
}

pub trait TransactionBroadcastClient {
    fn broadcast_transfer(
        &self,
        draft: &TransferBroadcastDraft<'_>,
    ) -> Result<BroadcastedTransaction, WalletError>;
}

pub struct RpcTransactionBroadcastClient {
    client: Client,
}

impl RpcTransactionBroadcastClient {
    pub fn new(settings: &NetworkPrivacySettings) -> Result<Self, WalletError> {
        let client =
            crate::protocol::network::build_http_client(settings, Duration::from_secs(18))?;
        Ok(Self { client })
    }
}

impl TransactionBroadcastClient for RpcTransactionBroadcastClient {
    fn broadcast_transfer(
        &self,
        draft: &TransferBroadcastDraft<'_>,
    ) -> Result<BroadcastedTransaction, WalletError> {
        match draft.chain {
            ChainId::Ethereum
            | ChainId::Bsc
            | ChainId::Polygon
            | ChainId::Arbitrum
            | ChainId::Optimism => evm::broadcast_transfer(&self.client, draft),
            ChainId::Tron => tron::broadcast_transfer(&self.client, draft),
            ChainId::Btc => btc::broadcast_transfer(&self.client, draft),
        }
    }
}

pub fn preview_transfer_supports_chain(preview: &TransferPreview) -> bool {
    matches!(
        preview.chain,
        ChainId::Ethereum
            | ChainId::Bsc
            | ChainId::Polygon
            | ChainId::Arbitrum
            | ChainId::Optimism
            | ChainId::Tron
            | ChainId::Btc
    )
}
