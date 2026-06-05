use crate::error::WalletError;
use sha2::{Digest, Sha256};

pub(super) fn tron_base58_to_hex(address: &str) -> Result<String, WalletError> {
    let decoded = bs58::decode(address)
        .into_vec()
        .map_err(|_| WalletError::InvalidAddress)?;
    if decoded.len() != 25 {
        return Err(WalletError::InvalidAddress);
    }
    let (payload, checksum) = decoded.split_at(21);
    if payload.first() != Some(&0x41) {
        return Err(WalletError::InvalidAddress);
    }
    let expected = tron_checksum(payload);
    if checksum != &expected[..] {
        return Err(WalletError::InvalidAddress);
    }
    Ok(hex::encode(payload))
}

fn tron_checksum(payload: &[u8]) -> [u8; 4] {
    let first = Sha256::digest(payload);
    let second = Sha256::digest(first);
    [second[0], second[1], second[2], second[3]]
}
