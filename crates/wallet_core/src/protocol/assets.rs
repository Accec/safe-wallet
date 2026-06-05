use crate::error::WalletError;
use crate::models::{ChainId, TokenMetadata};

pub trait TokenMetadataClient {
    fn fetch_token_metadata(
        &self,
        chain: ChainId,
        contract_address: &str,
    ) -> Result<TokenMetadata, WalletError>;
}

pub fn add_custom_token<C: TokenMetadataClient>(
    client: &C,
    chain: ChainId,
    contract_address: &str,
) -> Result<TokenMetadata, WalletError> {
    client.fetch_token_metadata(chain, contract_address)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::AssetKind;

    struct FakeClient;

    impl TokenMetadataClient for FakeClient {
        fn fetch_token_metadata(
            &self,
            chain: ChainId,
            contract_address: &str,
        ) -> Result<TokenMetadata, WalletError> {
            Ok(TokenMetadata {
                chain,
                kind: AssetKind::Erc20,
                contract_address: contract_address.to_string(),
                symbol: "USDC".to_string(),
                name: "USD Coin".to_string(),
                decimals: 6,
            })
        }
    }

    #[test]
    fn custom_token_metadata_is_loaded_from_client() {
        let token = add_custom_token(
            &FakeClient,
            ChainId::Ethereum,
            "0x0000000000000000000000000000000000000000",
        )
        .unwrap();

        assert_eq!(token.symbol, "USDC");
        assert_eq!(token.decimals, 6);
    }
}
