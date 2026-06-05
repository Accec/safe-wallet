use crate::error::WalletError;
use crate::models::ChainId;

pub trait ChainAddressValidator {
    fn chain(&self) -> ChainId;
    fn validate_address(&self, address: &str) -> Result<(), WalletError>;
}
