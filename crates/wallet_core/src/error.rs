use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WalletError {
    #[error("Invalid mnemonic")]
    InvalidMnemonic,
    #[error("Invalid private key")]
    InvalidPrivateKey,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Master password already set")]
    MasterPasswordAlreadySet,
    #[error("Wallet is locked")]
    Locked,
    #[error("Wallet not found")]
    WalletNotFound,
    #[error("Invalid address")]
    InvalidAddress,
    #[error("Invalid token contract")]
    InvalidTokenContract,
    #[error("Invalid network settings")]
    InvalidNetworkSettings,
    #[error("Invalid proxy settings")]
    InvalidProxySettings,
    #[error("Invalid multisig settings")]
    InvalidMultisigSettings,
    #[error("Multisig account not found")]
    MultisigNotFound,
    #[error("Duplicate multisig signature")]
    DuplicateMultisigSignature,
    #[error("Unauthorized multisig signer")]
    UnauthorizedMultisigSigner,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Insufficient energy")]
    InsufficientEnergy,
    #[error("Network unavailable")]
    NetworkUnavailable,
    #[error("Chain transaction broadcast is not supported")]
    ChainBroadcastUnsupported,
    #[error("Proxy connection failed")]
    ProxyConnectionFailed,
    #[error("Storage error")]
    Storage,
    #[error("Crypto error")]
    Crypto,
}

impl WalletError {
    pub fn safe_message(&self) -> &'static str {
        match self {
            WalletError::InvalidPassword => "Unlock failed",
            WalletError::MasterPasswordAlreadySet => "Master password is already set",
            WalletError::Crypto => "Security operation failed",
            WalletError::Storage => "Local wallet storage failed",
            WalletError::NetworkUnavailable => "Network request failed",
            WalletError::ChainBroadcastUnsupported => {
                "This chain does not support direct broadcast yet"
            }
            WalletError::InvalidMnemonic => "Invalid recovery phrase",
            WalletError::InvalidPrivateKey => "Invalid private key",
            WalletError::Locked => "Wallet is locked",
            WalletError::WalletNotFound => "Wallet not found",
            WalletError::InvalidAddress => "Invalid recipient address",
            WalletError::InvalidTokenContract => "Invalid token contract",
            WalletError::InvalidNetworkSettings => "Invalid network settings",
            WalletError::InvalidProxySettings => "Invalid proxy settings",
            WalletError::InvalidMultisigSettings => "Invalid multisig settings",
            WalletError::MultisigNotFound => "Multisig account not found",
            WalletError::DuplicateMultisigSignature => "Signature already exists",
            WalletError::UnauthorizedMultisigSigner => "Signer is not an owner",
            WalletError::InsufficientFunds => "Insufficient funds",
            WalletError::InsufficientEnergy => "Insufficient energy",
            WalletError::ProxyConnectionFailed => "Proxy connection failed",
        }
    }
}
