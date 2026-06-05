use crate::error::WalletError;
use crate::models::{TransferPreview, TransferRequest};

pub trait TransferPreviewClient {
    fn preview_transfer(&self, request: &TransferRequest) -> Result<TransferPreview, WalletError>;
}

pub fn preview_transfer<C: TransferPreviewClient>(
    client: &C,
    request: &TransferRequest,
) -> Result<TransferPreview, WalletError> {
    client.preview_transfer(request)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ChainId;
    use uuid::Uuid;

    struct FakeClient;

    impl TransferPreviewClient for FakeClient {
        fn preview_transfer(
            &self,
            request: &TransferRequest,
        ) -> Result<TransferPreview, WalletError> {
            Ok(TransferPreview {
                chain: request.chain,
                from_address: "0x1111111111111111111111111111111111111111".to_string(),
                to_address: request.to_address.clone(),
                asset_symbol: "ETH".to_string(),
                amount: request.amount.clone(),
                fee_estimate: "0.00042".to_string(),
                rpc_url: "https://ethereum-rpc.publicnode.com".to_string(),
            })
        }
    }

    #[test]
    fn transfer_preview_is_loaded_from_client() {
        let request = TransferRequest {
            wallet_id: Uuid::new_v4(),
            chain: ChainId::Ethereum,
            asset_id: Uuid::new_v4(),
            to_address: "0x0000000000000000000000000000000000000000".to_string(),
            amount: "1.25".to_string(),
        };

        let preview = preview_transfer(&FakeClient, &request).unwrap();

        assert_eq!(preview.chain, ChainId::Ethereum);
        assert_eq!(preview.to_address, request.to_address);
        assert_eq!(preview.amount, "1.25");
        assert_eq!(preview.asset_symbol, "ETH");
    }
}
