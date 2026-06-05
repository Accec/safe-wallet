use crate::error::WalletError;
use bitcoin::{OutPoint, Txid};
use reqwest::blocking::Client;
use std::str::FromStr;

#[derive(Debug, serde::Deserialize)]
struct BtcUtxoResponse {
    txid: String,
    vout: u32,
    value: u64,
}

#[derive(Debug, Clone)]
pub(super) struct BtcUtxo {
    pub(super) outpoint: OutPoint,
    pub(super) value_sats: u64,
}

pub(super) fn address_utxos(
    client: &Client,
    rpc_url: &str,
    address: &str,
) -> Result<Vec<BtcUtxo>, WalletError> {
    let url = format!(
        "{}/address/{}/utxo",
        rpc_url.trim().trim_end_matches('/'),
        address
    );
    let utxos = client
        .get(url)
        .send()
        .map_err(|_| WalletError::NetworkUnavailable)?
        .error_for_status()
        .map_err(|_| WalletError::NetworkUnavailable)?
        .json::<Vec<BtcUtxoResponse>>()
        .map_err(|_| WalletError::NetworkUnavailable)?;
    utxos
        .into_iter()
        .map(|utxo| {
            let txid = Txid::from_str(&utxo.txid).map_err(|_| WalletError::NetworkUnavailable)?;
            Ok(BtcUtxo {
                outpoint: OutPoint {
                    txid,
                    vout: utxo.vout,
                },
                value_sats: utxo.value,
            })
        })
        .collect()
}
