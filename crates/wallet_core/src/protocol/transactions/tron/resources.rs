use crate::error::WalletError;
use crate::models::TransferResourceStatus;
use reqwest::blocking::Client;
use serde_json::json;

use super::super::encoding::encode_trc20_transfer_parameter;
use super::rpc;

const TRON_ENERGY_FEE_SUN: u64 = 100;
const TRON_BANDWIDTH_FEE_SUN: u64 = 1_000;
const TRON_SIGNATURE_BYTES: u64 = 65;
const TRON_TRANSACTION_RESULT_BYTES: u64 = 64;
const TRON_TRANSACTION_OVERHEAD_BYTES: u64 = 10;
const FALLBACK_TRC20_BANDWIDTH_REQUIRED: u64 = 350;

pub(super) struct Trc20ResourceRequest<'a> {
    pub(super) rpc_url: &'a str,
    pub(super) owner_address: &'a str,
    pub(super) contract_address: &'a str,
    pub(super) to_address: &'a str,
    pub(super) amount: &'a str,
    pub(super) decimals: u8,
}

pub(super) fn trc20_transfer_status(
    client: &Client,
    request: &Trc20ResourceRequest<'_>,
) -> Result<TransferResourceStatus, WalletError> {
    let account = rpc::post(
        client,
        request.rpc_url,
        "/wallet/getaccount",
        json!({
            "address": request.owner_address,
            "visible": false,
        }),
    )?;
    let resource = rpc::post(
        client,
        request.rpc_url,
        "/wallet/getaccountresource",
        json!({
            "address": request.owner_address,
            "visible": false,
        }),
    )?;
    let estimate = rpc::post(
        client,
        request.rpc_url,
        "/wallet/triggerconstantcontract",
        json!({
            "owner_address": request.owner_address,
            "contract_address": request.contract_address,
            "function_selector": "transfer(address,uint256)",
            "parameter": encode_trc20_transfer_parameter(
                request.to_address,
                request.amount,
                request.decimals,
            )?,
            "visible": false,
        }),
    )?;

    let energy_available = resource_available(&resource, "EnergyLimit", "EnergyUsed");
    let free_bandwidth_available = resource_available(&resource, "freeNetLimit", "freeNetUsed");
    let staked_bandwidth_available = resource_available(&resource, "NetLimit", "NetUsed");
    let bandwidth_available = free_bandwidth_available.max(staked_bandwidth_available);
    let energy_required =
        numeric_field(&estimate, "energy_used").ok_or(WalletError::NetworkUnavailable)?;
    let bandwidth_required = transaction_bandwidth_required(&estimate);
    let trx_fee_reserve_required_sun = resource_fee_reserve_sun(
        energy_available,
        energy_required,
        free_bandwidth_available,
        staked_bandwidth_available,
        bandwidth_required,
    );
    Ok(TransferResourceStatus {
        energy_available,
        energy_required,
        bandwidth_available,
        bandwidth_required,
        trx_balance_sun: numeric_field(&account, "balance").unwrap_or(0),
        trx_fee_reserve_required_sun,
        can_send_without_burning_trx: trx_fee_reserve_required_sun == 0,
    })
}

fn resource_available(body: &serde_json::Value, limit_key: &str, used_key: &str) -> u64 {
    numeric_field(body, limit_key)
        .unwrap_or(0)
        .saturating_sub(numeric_field(body, used_key).unwrap_or(0))
}

fn numeric_field(body: &serde_json::Value, key: &str) -> Option<u64> {
    body.get(key).and_then(|value| {
        value
            .as_u64()
            .or_else(|| value.as_str().and_then(|text| text.parse::<u64>().ok()))
    })
}

fn resource_fee_reserve_sun(
    energy_available: u64,
    energy_required: u64,
    free_bandwidth_available: u64,
    staked_bandwidth_available: u64,
    bandwidth_required: u64,
) -> u64 {
    let energy_fee = energy_required
        .saturating_sub(energy_available)
        .saturating_mul(TRON_ENERGY_FEE_SUN);
    let bandwidth_fee = if staked_bandwidth_available >= bandwidth_required
        || free_bandwidth_available >= bandwidth_required
    {
        0
    } else {
        bandwidth_required.saturating_mul(TRON_BANDWIDTH_FEE_SUN)
    };
    energy_fee.saturating_add(bandwidth_fee)
}

fn transaction_bandwidth_required(body: &serde_json::Value) -> u64 {
    body.get("transaction")
        .and_then(|transaction| transaction.get("raw_data_hex"))
        .and_then(|value| value.as_str())
        .and_then(raw_data_hex_bytes)
        .map(|raw_data_bytes| {
            raw_data_bytes
                .saturating_add(TRON_SIGNATURE_BYTES)
                .saturating_add(TRON_TRANSACTION_RESULT_BYTES)
                .saturating_add(TRON_TRANSACTION_OVERHEAD_BYTES)
        })
        .unwrap_or(FALLBACK_TRC20_BANDWIDTH_REQUIRED)
}

fn raw_data_hex_bytes(value: &str) -> Option<u64> {
    let hex = value.strip_prefix("0x").unwrap_or(value);
    if hex.is_empty() || hex.len() % 2 != 0 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    u64::try_from(hex.len() / 2).ok()
}
