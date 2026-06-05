use crate::chains::ChainAddressValidator;
use crate::error::WalletError;
use crate::models::ChainId;

pub struct BtcValidator;

impl ChainAddressValidator for BtcValidator {
    fn chain(&self) -> ChainId {
        ChainId::Btc
    }

    fn validate_address(&self, address: &str) -> Result<(), WalletError> {
        let valid_length = (14..=90).contains(&address.len());
        let valid_shape = address
            .strip_prefix("bc1")
            .filter(|rest| !rest.is_empty())
            .map(|rest| rest.chars().all(is_bech32_payload_char))
            .unwrap_or(false);

        if valid_length && valid_shape {
            Ok(())
        } else {
            Err(WalletError::InvalidAddress)
        }
    }
}

fn is_bech32_payload_char(ch: char) -> bool {
    matches!(
        ch,
        'q' | 'p'
            | 'z'
            | 'r'
            | 'y'
            | '9'
            | 'x'
            | '8'
            | 'g'
            | 'f'
            | '2'
            | 't'
            | 'v'
            | 'd'
            | 'w'
            | '0'
            | 's'
            | '3'
            | 'j'
            | 'n'
            | '5'
            | '4'
            | 'k'
            | 'h'
            | 'c'
            | 'e'
            | '6'
            | 'm'
            | 'u'
            | 'a'
            | '7'
            | 'l'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_chain_address_shape() {
        let validator = BtcValidator;
        validator
            .validate_address("bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu")
            .unwrap();
    }

    #[test]
    fn rejects_non_mainnet_or_uppercase_shapes() {
        let validator = BtcValidator;

        assert_eq!(
            validator.validate_address("tb1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu"),
            Err(WalletError::InvalidAddress)
        );
        assert_eq!(
            validator.validate_address("BC1QCR8TE4KR609GCAWUTMRZA0J4XV80JY8Z306FYU"),
            Err(WalletError::InvalidAddress)
        );
    }

    #[test]
    fn rejects_invalid_bech32_payload_characters() {
        let validator = BtcValidator;

        assert_eq!(
            validator.validate_address("bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyi"),
            Err(WalletError::InvalidAddress)
        );
    }
}
