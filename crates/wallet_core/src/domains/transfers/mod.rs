use crate::error::WalletError;
use crate::models::{TransferPreview, TransferRequest, TransferResult};
use crate::protocol::transactions::{RpcTransactionBroadcastClient, TransactionBroadcastClient};
use crate::storage::WalletDatabase;

mod preview;
mod sending;
mod signing;

pub struct TransfersDomain {
    pub(super) database: WalletDatabase,
}

impl TransfersDomain {
    pub(crate) fn new(database: WalletDatabase) -> Self {
        Self { database }
    }

    pub fn preview_transfer(
        &self,
        request: &TransferRequest,
    ) -> Result<TransferPreview, WalletError> {
        preview::preview_transfer(&self.database, request)
    }

    pub fn send_transfer(
        &self,
        request: &TransferRequest,
        password: &str,
    ) -> Result<TransferResult, WalletError> {
        let settings = self.database.network().network_privacy_settings()?;
        let client = RpcTransactionBroadcastClient::new(&settings)?;
        self.send_transfer_with(request, password, &client)
    }

    pub fn send_transfer_with<C: TransactionBroadcastClient>(
        &self,
        request: &TransferRequest,
        password: &str,
        client: &C,
    ) -> Result<TransferResult, WalletError> {
        sending::send_transfer_with(&self.database, request, password, client)
    }
}
