use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub(in crate::protocol) struct MnemonicWalletPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) label: String,
    pub(in crate::protocol) mnemonic: String,
    pub(in crate::protocol) password: String,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct PrivateKeyWalletPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) label: String,
    pub(in crate::protocol) private_key: String,
    pub(in crate::protocol) password: String,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct KeystoreWalletPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) label: String,
    pub(in crate::protocol) keystore_json: String,
    pub(in crate::protocol) keystore_password: String,
    pub(in crate::protocol) password: String,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct WalletPasswordPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) wallet_id: Uuid,
    pub(in crate::protocol) password: String,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct WalletIdPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) wallet_id: Uuid,
}
