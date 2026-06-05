use crate::chains;
use crate::chains::ChainAddressValidator;
use crate::error::WalletError;
use crate::models::{
    ChainId, CreateMultisigProposalRequest, ImportMultisigAccountRequest, MultisigAccount,
    MultisigKind,
};

pub(crate) fn validate_chain_address(chain: ChainId, address: &str) -> Result<(), WalletError> {
    match chain {
        ChainId::Btc => chains::btc::BtcValidator.validate_address(address),
        ChainId::Ethereum
        | ChainId::Bsc
        | ChainId::Polygon
        | ChainId::Arbitrum
        | ChainId::Optimism => chains::evm::EvmValidator::new(chain).validate_address(address),
        ChainId::Tron => chains::tron::TronValidator.validate_address(address),
    }
}

pub(crate) fn fee_estimate_for_chain(chain: ChainId) -> &'static str {
    match chain {
        ChainId::Btc => "0.00001",
        ChainId::Tron => "1.000000",
        ChainId::Ethereum
        | ChainId::Bsc
        | ChainId::Polygon
        | ChainId::Arbitrum
        | ChainId::Optimism => "0.00042",
    }
}

pub(crate) fn validate_multisig_request(
    request: &ImportMultisigAccountRequest,
) -> Result<(), WalletError> {
    if request.threshold == 0 || request.owners.is_empty() {
        return Err(WalletError::InvalidMultisigSettings);
    }
    match request.kind {
        MultisigKind::EvmSafe if is_evm_chain(request.chain) => {
            if request.permission_id.is_some() {
                return Err(WalletError::InvalidMultisigSettings);
            }
        }
        MultisigKind::TronPermission if request.chain == ChainId::Tron => {
            if request.permission_id.is_none() {
                return Err(WalletError::InvalidMultisigSettings);
            }
        }
        _ => return Err(WalletError::InvalidMultisigSettings),
    }
    validate_chain_address(request.chain, &request.address)?;
    let mut total_weight = 0u32;
    let mut seen = std::collections::HashSet::new();
    for owner in &request.owners {
        if owner.weight == 0 {
            return Err(WalletError::InvalidMultisigSettings);
        }
        validate_chain_address(request.chain, &owner.address)?;
        if !seen.insert(owner.address.trim().to_ascii_lowercase()) {
            return Err(WalletError::InvalidMultisigSettings);
        }
        total_weight = total_weight
            .checked_add(owner.weight)
            .ok_or(WalletError::InvalidMultisigSettings)?;
    }
    if total_weight < request.threshold {
        return Err(WalletError::InvalidMultisigSettings);
    }
    Ok(())
}

pub(crate) fn multisig_payload_json(
    account: &MultisigAccount,
    request: &CreateMultisigProposalRequest,
) -> Result<String, WalletError> {
    let payload = match account.kind {
        MultisigKind::EvmSafe => serde_json::json!({
            "kind": "evm_safe",
            "safe_address": account.address,
            "chain": account.chain,
            "to": request.to_address.trim(),
            "asset_symbol": request.asset_symbol.trim(),
            "amount": request.amount.trim(),
            "operation": 0,
            "data": "0x",
            "nonce": null,
            "safe_tx_gas": "0",
            "base_gas": "0",
            "gas_price": "0",
            "refund_receiver": "0x0000000000000000000000000000000000000000"
        }),
        MultisigKind::TronPermission => serde_json::json!({
            "kind": "tron_permission",
            "account_address": account.address,
            "permission_id": account.permission_id,
            "chain": account.chain,
            "to": request.to_address.trim(),
            "asset_symbol": request.asset_symbol.trim(),
            "amount": request.amount.trim()
        }),
    };
    serde_json::to_string(&payload).map_err(|_| WalletError::Storage)
}

fn is_evm_chain(chain: ChainId) -> bool {
    matches!(
        chain,
        ChainId::Ethereum | ChainId::Bsc | ChainId::Polygon | ChainId::Arbitrum | ChainId::Optimism
    )
}
