use crate::error::WalletError;
use crate::models::TransferResourceStatus;
use reqwest::blocking::Client;
use serde_json::json;

use super::super::encoding::encode_trc20_transfer_parameter;
use super::{balances, rpc};

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
    let bandwidth_available = resource_available(&resource, "freeNetLimit", "freeNetUsed")
        .saturating_add(resource_available(&resource, "NetLimit", "NetUsed"));
    let energy_required =
        numeric_field(&estimate, "energy_used").ok_or(WalletError::NetworkUnavailable)?;
    Ok(TransferResourceStatus {
        energy_available,
        energy_required,
        bandwidth_available,
        trx_balance_sun: numeric_field(&account, "balance").unwrap_or(0),
        trx_fee_reserve_required_sun: balances::MIN_TRC20_FEE_RESERVE_SUN,
        can_send_without_burning_trx: energy_available >= energy_required,
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
