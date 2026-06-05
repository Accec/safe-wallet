use crate::error::WalletError;
use bitcoin::address::KnownHrp;
use bitcoin::{Address, CompressedPublicKey};
use k256::ecdsa::SigningKey;
use sha2::{Digest, Sha256};
use tiny_keccak::{Hasher, Keccak};

use super::{derivation::derive_xprv, BTC_PATH, TRON_PATH};

pub(super) fn derive_btc_address(seed: &[u8; 64]) -> Result<String, WalletError> {
    let xprv = derive_xprv(seed, BTC_PATH)?;
    btc_address_from_signing_key(xprv.private_key())
}

pub(super) fn derive_evm_address(seed: &[u8; 64], path: &str) -> Result<String, WalletError> {
    let address = derive_evm_address_bytes(seed, path)?;
    Ok(format!("0x{}", hex::encode(address)))
}

pub(super) fn derive_tron_address(seed: &[u8; 64]) -> Result<String, WalletError> {
    let address = derive_evm_address_bytes(seed, TRON_PATH)?;
    Ok(tron_address_from_evm_address(address))
}

pub(super) fn tron_address_from_evm_address(address: [u8; 20]) -> String {
    let mut payload = Vec::with_capacity(21);
    payload.push(0x41);
    payload.extend_from_slice(&address);
    base58check_encode(&payload)
}

fn derive_evm_address_bytes(seed: &[u8; 64], path: &str) -> Result<[u8; 20], WalletError> {
    let xprv = derive_xprv(seed, path)?;
    Ok(evm_address_bytes_from_signing_key(xprv.private_key()))
}

pub(super) fn evm_address_bytes_from_signing_key(signing_key: &SigningKey) -> [u8; 20] {
    let public_key = signing_key.verifying_key().to_encoded_point(false);
    let public_key = public_key.as_bytes();

    let mut hash = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(&public_key[1..]);
    hasher.finalize(&mut hash);

    let mut address = [0u8; 20];
    address.copy_from_slice(&hash[12..]);
    address
}

pub(super) fn btc_address_from_signing_key(
    signing_key: &SigningKey,
) -> Result<String, WalletError> {
    let public_key = signing_key.verifying_key().to_encoded_point(true);
    let public_key = bitcoin::secp256k1::PublicKey::from_slice(public_key.as_bytes())
        .map_err(|_| WalletError::Crypto)?;
    let address = Address::p2wpkh(&CompressedPublicKey(public_key), KnownHrp::Mainnet);
    Ok(address.to_string())
}

fn base58check_encode(payload: &[u8]) -> String {
    let first_hash = Sha256::digest(payload);
    let second_hash = Sha256::digest(first_hash);

    let mut bytes = Vec::with_capacity(payload.len() + 4);
    bytes.extend_from_slice(payload);
    bytes.extend_from_slice(&second_hash[..4]);

    bs58::encode(bytes).into_string()
}
