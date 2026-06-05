use crate::error::WalletError;
use crate::models::{
    ActivityKind, ActivityStatus, AssetKind, ChainId, MultisigKind, MultisigProposalStatus,
    ProxyMode,
};

pub fn chain_to_db(chain: ChainId) -> &'static str {
    match chain {
        ChainId::Btc => "btc",
        ChainId::Ethereum => "ethereum",
        ChainId::Bsc => "bsc",
        ChainId::Polygon => "polygon",
        ChainId::Arbitrum => "arbitrum",
        ChainId::Optimism => "optimism",
        ChainId::Tron => "tron",
    }
}

pub(crate) fn db_to_chain(value: &str) -> Result<ChainId, WalletError> {
    match value {
        "btc" => Ok(ChainId::Btc),
        "ethereum" => Ok(ChainId::Ethereum),
        "bsc" => Ok(ChainId::Bsc),
        "polygon" => Ok(ChainId::Polygon),
        "arbitrum" => Ok(ChainId::Arbitrum),
        "optimism" => Ok(ChainId::Optimism),
        "tron" => Ok(ChainId::Tron),
        _ => Err(WalletError::Storage),
    }
}

pub(crate) fn proxy_mode_to_db(mode: ProxyMode) -> &'static str {
    match mode {
        ProxyMode::Custom => "custom",
        ProxyMode::Tor => "tor",
    }
}

pub(crate) fn db_to_proxy_mode(value: &str) -> ProxyMode {
    match value {
        "tor" => ProxyMode::Tor,
        _ => ProxyMode::Custom,
    }
}

pub(crate) fn asset_kind_to_db(kind: AssetKind) -> &'static str {
    match kind {
        AssetKind::Native => "native",
        AssetKind::Erc20 => "erc20",
        AssetKind::Trc20 => "trc20",
    }
}

pub(crate) fn db_to_asset_kind(value: &str) -> Result<AssetKind, WalletError> {
    match value {
        "native" => Ok(AssetKind::Native),
        "erc20" => Ok(AssetKind::Erc20),
        "trc20" => Ok(AssetKind::Trc20),
        _ => Err(WalletError::Storage),
    }
}

pub(crate) fn activity_kind_to_db(kind: ActivityKind) -> &'static str {
    match kind {
        ActivityKind::NativeTransfer => "native_transfer",
        ActivityKind::TokenTransfer => "token_transfer",
        ActivityKind::Approval => "approval",
        ActivityKind::Swap => "swap",
        ActivityKind::ContractCall => "contract_call",
    }
}

pub(crate) fn db_to_activity_kind(value: &str) -> Result<ActivityKind, WalletError> {
    match value {
        "native_transfer" => Ok(ActivityKind::NativeTransfer),
        "token_transfer" => Ok(ActivityKind::TokenTransfer),
        "approval" => Ok(ActivityKind::Approval),
        "swap" => Ok(ActivityKind::Swap),
        "contract_call" => Ok(ActivityKind::ContractCall),
        _ => Err(WalletError::Storage),
    }
}

pub(crate) fn activity_status_to_db(status: ActivityStatus) -> &'static str {
    match status {
        ActivityStatus::Pending => "pending",
        ActivityStatus::Confirmed => "confirmed",
        ActivityStatus::Failed => "failed",
    }
}

pub(crate) fn db_to_activity_status(value: &str) -> Result<ActivityStatus, WalletError> {
    match value {
        "pending" => Ok(ActivityStatus::Pending),
        "confirmed" => Ok(ActivityStatus::Confirmed),
        "failed" => Ok(ActivityStatus::Failed),
        _ => Err(WalletError::Storage),
    }
}

pub(crate) fn multisig_kind_to_db(kind: MultisigKind) -> &'static str {
    match kind {
        MultisigKind::EvmSafe => "evm_safe",
        MultisigKind::TronPermission => "tron_permission",
    }
}

pub(crate) fn db_to_multisig_kind(value: &str) -> Result<MultisigKind, WalletError> {
    match value {
        "evm_safe" => Ok(MultisigKind::EvmSafe),
        "tron_permission" => Ok(MultisigKind::TronPermission),
        _ => Err(WalletError::Storage),
    }
}

pub(crate) fn multisig_status_to_db(status: MultisigProposalStatus) -> &'static str {
    match status {
        MultisigProposalStatus::PendingSignatures => "pending_signatures",
        MultisigProposalStatus::Ready => "ready",
        MultisigProposalStatus::Executed => "executed",
        MultisigProposalStatus::Failed => "failed",
    }
}

pub(crate) fn db_to_multisig_status(value: &str) -> Result<MultisigProposalStatus, WalletError> {
    match value {
        "pending_signatures" => Ok(MultisigProposalStatus::PendingSignatures),
        "ready" => Ok(MultisigProposalStatus::Ready),
        "executed" => Ok(MultisigProposalStatus::Executed),
        "failed" => Ok(MultisigProposalStatus::Failed),
        _ => Err(WalletError::Storage),
    }
}
