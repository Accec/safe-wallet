mod addresses;
mod derivation;
mod private_keys;

#[cfg(test)]
mod tests;

use crate::error::WalletError;
use crate::models::{Account, ChainId};
use bip39::{Language, Mnemonic};
use k256::ecdsa::SigningKey;
use uuid::Uuid;

use addresses::{
    btc_address_from_signing_key, derive_btc_address, derive_evm_address, derive_tron_address,
    evm_address_bytes_from_signing_key, tron_address_from_evm_address,
};
use derivation::derive_xprv;
use private_keys::private_key_signing_key;

const BTC_PATH: &str = "m/84'/0'/0'/0/0";
const EVM_PATH: &str = "m/44'/60'/0'/0/0";
const TRON_PATH: &str = "m/44'/195'/0'/0/0";
pub const PRIVATE_KEY_PATH: &str = "private_key:secp256k1";

pub fn derive_default_accounts(
    wallet_id: Uuid,
    mnemonic: &str,
) -> Result<Vec<Account>, WalletError> {
    let mnemonic = Mnemonic::parse_in_normalized(Language::English, mnemonic)
        .map_err(|_| WalletError::InvalidMnemonic)?;
    let seed = mnemonic.to_seed("");

    let btc_address = derive_btc_address(&seed)?;
    let evm_address = derive_evm_address(&seed, EVM_PATH)?;
    let tron_address = derive_tron_address(&seed)?;

    Ok(vec![
        account(wallet_id, ChainId::Btc, btc_address, BTC_PATH),
        account(wallet_id, ChainId::Ethereum, evm_address.clone(), EVM_PATH),
        account(wallet_id, ChainId::Bsc, evm_address.clone(), EVM_PATH),
        account(wallet_id, ChainId::Polygon, evm_address.clone(), EVM_PATH),
        account(wallet_id, ChainId::Arbitrum, evm_address.clone(), EVM_PATH),
        account(wallet_id, ChainId::Optimism, evm_address, EVM_PATH),
        account(wallet_id, ChainId::Tron, tron_address, TRON_PATH),
    ])
}

pub fn derive_private_key_accounts(
    wallet_id: Uuid,
    private_key: &str,
) -> Result<Vec<Account>, WalletError> {
    let signing_key = private_key_signing_key(private_key)?;
    let evm_address_bytes = evm_address_bytes_from_signing_key(&signing_key);
    let evm_address = format!("0x{}", hex::encode(evm_address_bytes));
    let btc_address = btc_address_from_signing_key(&signing_key)?;
    let tron_address = tron_address_from_evm_address(evm_address_bytes);

    Ok(vec![
        account(wallet_id, ChainId::Btc, btc_address, PRIVATE_KEY_PATH),
        account(
            wallet_id,
            ChainId::Ethereum,
            evm_address.clone(),
            PRIVATE_KEY_PATH,
        ),
        account(
            wallet_id,
            ChainId::Bsc,
            evm_address.clone(),
            PRIVATE_KEY_PATH,
        ),
        account(
            wallet_id,
            ChainId::Polygon,
            evm_address.clone(),
            PRIVATE_KEY_PATH,
        ),
        account(
            wallet_id,
            ChainId::Arbitrum,
            evm_address.clone(),
            PRIVATE_KEY_PATH,
        ),
        account(wallet_id, ChainId::Optimism, evm_address, PRIVATE_KEY_PATH),
        account(wallet_id, ChainId::Tron, tron_address, PRIVATE_KEY_PATH),
    ])
}

pub fn signing_key_for_mnemonic_path(
    mnemonic: &str,
    derivation_path: &str,
) -> Result<SigningKey, WalletError> {
    let mnemonic = Mnemonic::parse_in_normalized(Language::English, mnemonic)
        .map_err(|_| WalletError::InvalidMnemonic)?;
    let seed = mnemonic.to_seed("");
    let xprv = derive_xprv(&seed, derivation_path)?;
    Ok(xprv.private_key().clone())
}

pub fn signing_key_for_private_key(private_key: &str) -> Result<SigningKey, WalletError> {
    private_key_signing_key(private_key)
}

fn account(wallet_id: Uuid, chain: ChainId, address: String, derivation_path: &str) -> Account {
    Account {
        wallet_id,
        chain,
        address,
        derivation_path: derivation_path.to_string(),
        account_index: 0,
    }
}
