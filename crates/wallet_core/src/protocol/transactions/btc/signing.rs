use crate::error::WalletError;
use bitcoin::secp256k1::{Message, Secp256k1, SecretKey};
use bitcoin::sighash::{EcdsaSighashType, SighashCache};
use bitcoin::{Address, Amount, Transaction, Witness};
use k256::ecdsa::SigningKey;

use super::utxos::BtcUtxo;

pub(super) fn sign_p2wpkh_inputs(
    transaction: &mut Transaction,
    from_address: &Address,
    selected_utxos: &[BtcUtxo],
    signing_key: &SigningKey,
) -> Result<(), WalletError> {
    let key_bytes = signing_key.to_bytes();
    let secret_key =
        SecretKey::from_slice(key_bytes.as_slice()).map_err(|_| WalletError::Crypto)?;
    let secp = Secp256k1::new();
    let public_key = secret_key.public_key(&secp);
    let script_pubkey = from_address.script_pubkey();
    let sighash_type = EcdsaSighashType::All;
    let mut sighasher = SighashCache::new(transaction);
    for (input_index, utxo) in selected_utxos.iter().enumerate() {
        let sighash = sighasher
            .p2wpkh_signature_hash(
                input_index,
                &script_pubkey,
                Amount::from_sat(utxo.value_sats),
                sighash_type,
            )
            .map_err(|_| WalletError::Crypto)?;
        let message = Message::from(sighash);
        let signature = secp.sign_ecdsa(&message, &secret_key);
        let signature = bitcoin::ecdsa::Signature {
            signature,
            sighash_type,
        };
        *sighasher
            .witness_mut(input_index)
            .ok_or(WalletError::Crypto)? = Witness::p2wpkh(&signature, &public_key);
    }
    Ok(())
}
