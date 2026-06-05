use crate::error::WalletError;
use bip32::{DerivationPath, ExtendedPrivateKey};
use k256::ecdsa::SigningKey;
use std::str::FromStr;

type WalletXPrv = ExtendedPrivateKey<SigningKey>;

pub(super) fn derive_xprv(seed: &[u8; 64], path: &str) -> Result<WalletXPrv, WalletError> {
    let path = DerivationPath::from_str(path).map_err(|_| WalletError::InvalidMnemonic)?;
    WalletXPrv::derive_from_path(seed, &path).map_err(|_| WalletError::InvalidMnemonic)
}
