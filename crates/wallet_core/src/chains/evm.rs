use crate::chains::ChainAddressValidator;
use crate::error::WalletError;
use crate::models::ChainId;

pub struct EvmValidator {
    chain: ChainId,
}

impl EvmValidator {
    pub fn new(chain: ChainId) -> Self {
        Self { chain }
    }
}

impl ChainAddressValidator for EvmValidator {
    fn chain(&self) -> ChainId {
        self.chain
    }

    fn validate_address(&self, address: &str) -> Result<(), WalletError> {
        let valid = address
            .strip_prefix("0x")
            .filter(|hex| hex.len() == 40)
            .map(|hex| hex.chars().all(|ch| ch.is_ascii_hexdigit()))
            .unwrap_or(false);

        if valid {
            Ok(())
        } else {
            Err(WalletError::InvalidAddress)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_chain_address_shape() {
        let validator = EvmValidator::new(ChainId::Ethereum);
        validator
            .validate_address("0x0000000000000000000000000000000000000000")
            .unwrap();
    }

    #[test]
    fn rejects_wrong_prefix_length_or_non_hex() {
        let validator = EvmValidator::new(ChainId::Ethereum);

        assert_eq!(
            validator.validate_address("0000000000000000000000000000000000000000"),
            Err(WalletError::InvalidAddress)
        );
        assert_eq!(
            validator.validate_address("0x000000000000000000000000000000000000000"),
            Err(WalletError::InvalidAddress)
        );
        assert_eq!(
            validator.validate_address("0x000000000000000000000000000000000000000g"),
            Err(WalletError::InvalidAddress)
        );
    }
}
