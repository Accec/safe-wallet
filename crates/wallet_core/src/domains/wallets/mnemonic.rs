use crate::error::WalletError;
use bip39::{Language, Mnemonic};

pub fn generate_mnemonic() -> Result<String, WalletError> {
    let mnemonic =
        Mnemonic::generate_in(Language::English, 12).map_err(|_| WalletError::InvalidMnemonic)?;
    Ok(mnemonic.to_string())
}

pub fn validate_mnemonic(mnemonic: &str) -> Result<(), WalletError> {
    Mnemonic::parse_in_normalized(Language::English, mnemonic)
        .map(|_| ())
        .map_err(|_| WalletError::InvalidMnemonic)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_mnemonic_is_valid() {
        let mnemonic = generate_mnemonic().unwrap();
        validate_mnemonic(&mnemonic).unwrap();
        assert_eq!(mnemonic.split_whitespace().count(), 12);
    }

    #[test]
    fn known_valid_mnemonic_is_accepted() {
        validate_mnemonic(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
        )
        .unwrap();
    }
}
