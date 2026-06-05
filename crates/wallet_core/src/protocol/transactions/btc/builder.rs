use crate::error::WalletError;
use bitcoin::locktime::absolute;
use bitcoin::{
    transaction, Address, Amount, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness,
};
use k256::ecdsa::SigningKey;

use super::signing::sign_p2wpkh_inputs;
use super::utxos::BtcUtxo;

pub(super) fn build_signed_transaction(
    from_address: &Address,
    to_address: &Address,
    amount_sats: u64,
    fee_rate_sat_vb: u64,
    available_utxos: &[BtcUtxo],
    signing_key: &SigningKey,
) -> Result<Transaction, WalletError> {
    if amount_sats == 0 {
        return Err(WalletError::InsufficientFunds);
    }
    let selected = select_utxos(available_utxos, amount_sats, fee_rate_sat_vb)?;
    let selected_value = selected
        .iter()
        .try_fold(0_u64, |total, utxo| total.checked_add(utxo.value_sats))
        .ok_or(WalletError::InsufficientFunds)?;
    let mut output_count = 2_usize;
    let mut fee = estimate_fee(selected.len(), output_count, fee_rate_sat_vb)?;
    let mut change = selected_value
        .checked_sub(amount_sats)
        .and_then(|value| value.checked_sub(fee))
        .ok_or(WalletError::InsufficientFunds)?;
    if change < 546 {
        output_count = 1;
        fee = estimate_fee(selected.len(), output_count, fee_rate_sat_vb)?;
        change = selected_value
            .checked_sub(amount_sats)
            .and_then(|value| value.checked_sub(fee))
            .ok_or(WalletError::InsufficientFunds)?;
    }

    let mut outputs = vec![TxOut {
        value: Amount::from_sat(amount_sats),
        script_pubkey: to_address.script_pubkey(),
    }];
    if output_count == 2 && change >= 546 {
        outputs.push(TxOut {
            value: Amount::from_sat(change),
            script_pubkey: from_address.script_pubkey(),
        });
    }
    let mut unsigned_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: selected
            .iter()
            .map(|utxo| TxIn {
                previous_output: utxo.outpoint,
                script_sig: ScriptBuf::new(),
                sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
                witness: Witness::new(),
            })
            .collect(),
        output: outputs,
    };
    sign_p2wpkh_inputs(&mut unsigned_tx, from_address, &selected, signing_key)?;
    Ok(unsigned_tx)
}

fn select_utxos(
    available_utxos: &[BtcUtxo],
    amount_sats: u64,
    fee_rate_sat_vb: u64,
) -> Result<Vec<BtcUtxo>, WalletError> {
    let mut utxos = available_utxos.to_vec();
    utxos.sort_by_key(|utxo| utxo.value_sats);
    let mut selected = Vec::new();
    let mut selected_value = 0_u64;
    for utxo in utxos {
        selected_value = selected_value
            .checked_add(utxo.value_sats)
            .ok_or(WalletError::InsufficientFunds)?;
        selected.push(utxo);
        let fee = estimate_fee(selected.len(), 2, fee_rate_sat_vb)?;
        if selected_value >= amount_sats.saturating_add(fee) {
            return Ok(selected);
        }
    }
    Err(WalletError::InsufficientFunds)
}

fn estimate_fee(
    input_count: usize,
    output_count: usize,
    fee_rate_sat_vb: u64,
) -> Result<u64, WalletError> {
    let vbytes = 10_u64
        .checked_add(
            (input_count as u64)
                .checked_mul(68)
                .ok_or(WalletError::InsufficientFunds)?,
        )
        .and_then(|value| value.checked_add((output_count as u64).checked_mul(31)?))
        .ok_or(WalletError::InsufficientFunds)?;
    vbytes
        .checked_mul(fee_rate_sat_vb.max(1))
        .ok_or(WalletError::InsufficientFunds)
}
