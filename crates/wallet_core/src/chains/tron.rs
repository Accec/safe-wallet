use crate::chains::ChainAddressValidator;
use crate::error::WalletError;
use crate::models::ChainId;

pub struct TronValidator;

impl ChainAddressValidator for TronValidator {
    fn chain(&self) -> ChainId {
        ChainId::Tron
    }

    fn validate_address(&self, address: &str) -> Result<(), WalletError> {
        let valid = address
            .strip_prefix('T')
            .filter(|rest| rest.len() == 33)
            .map(|rest| rest.chars().all(is_base58_char))
            .unwrap_or(false);

        if valid {
            Ok(())
        } else {
            Err(WalletError::InvalidAddress)
        }
    }
}

fn is_base58_char(ch: char) -> bool {
    matches!(
        ch,
        '1'..='9' | 'A'..='H' | 'J'..='N' | 'P'..='Z' | 'a'..='k' | 'm'..='z'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_chain_address_shape() {
        let validator = TronValidator;
        validator
            .validate_address("TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7")
            .unwrap();
    }

    #[test]
    fn rejects_wrong_prefix_length_or_non_base58() {
        let validator = TronValidator;

        assert_eq!(
            validator.validate_address("ALa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7"),
            Err(WalletError::InvalidAddress)
        );
        assert_eq!(
            validator.validate_address("TLa2f6VPqDgRE67v1736s7bJ8Ray5wYj"),
            Err(WalletError::InvalidAddress)
        );
        assert_eq!(
            validator.validate_address("TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjI"),
            Err(WalletError::InvalidAddress)
        );
    }
}
