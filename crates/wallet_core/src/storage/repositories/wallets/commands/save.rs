use super::super::super::super::mappers::*;
use super::super::super::WalletRepository;
use crate::domains::wallets::keystore::EncryptedKeystore;
use crate::error::WalletError;
use crate::models::{Account, ChainSettings, WalletSummary};
use chrono::Utc;
use rusqlite::params;
use uuid::Uuid;

impl WalletRepository {
    pub fn save_wallet(
        &self,
        wallet: &WalletSummary,
        keystore: &EncryptedKeystore,
        accounts: &[Account],
        chain_settings: &[ChainSettings],
    ) -> Result<(), WalletError> {
        let mut connection = self.database.connect()?;
        let transaction = connection.transaction().map_err(|_| WalletError::Storage)?;
        let now = Utc::now().to_rfc3339();
        let wallet_id = wallet.id.to_string();
        let created_at = wallet.created_at.to_rfc3339();

        transaction
            .execute(
                "insert into keystore_items (
                    id, secret_kind, ciphertext, nonce, salt, kdf_name, kdf_params_json,
                    cipher_name, version, created_at, updated_at
                ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    wallet_id,
                    keystore.secret_kind,
                    keystore.ciphertext_b64,
                    keystore.nonce_b64,
                    keystore.salt_b64,
                    keystore.kdf_name,
                    keystore.kdf_params_json,
                    keystore.cipher_name,
                    keystore.version,
                    now,
                    now
                ],
            )
            .map_err(|_| WalletError::Storage)?;

        transaction
            .execute(
                "insert into wallets (
                    id, label, keystore_id, hidden, created_at, updated_at
                ) values (?1, ?2, ?3, 0, ?4, ?5)",
                params![wallet_id, wallet.label, wallet_id, created_at, now],
            )
            .map_err(|_| WalletError::Storage)?;

        for account in accounts {
            transaction
                .execute(
                    "insert into accounts (
                        id, wallet_id, chain, address, derivation_path, account_index, created_at
                    ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![
                        Uuid::new_v4().to_string(),
                        wallet_id,
                        chain_to_db(account.chain),
                        account.address,
                        account.derivation_path,
                        account.account_index,
                        now
                    ],
                )
                .map_err(|_| WalletError::Storage)?;
        }

        for setting in chain_settings {
            transaction
                .execute(
                    "insert or ignore into chain_settings (
                        chain, network_name, chain_id, enabled, default_rpc_url,
                        user_rpc_url, explorer_url, native_symbol, native_decimals,
                        updated_at
                    ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                    params![
                        chain_to_db(setting.chain),
                        setting.network_name,
                        setting.chain_id,
                        if setting.enabled { 1 } else { 0 },
                        setting.default_rpc_url,
                        setting.user_rpc_url,
                        setting.explorer_url,
                        setting.native_symbol,
                        setting.native_decimals,
                        now
                    ],
                )
                .map_err(|_| WalletError::Storage)?;
            transaction
                .execute(
                    "insert or ignore into tokens (
                        id, chain, contract_address, kind, symbol, name, decimals,
                        source, visible, updated_at
                    ) values (?1, ?2, null, 'native', ?3, ?4, ?5, 'system', 1, ?6)",
                    params![
                        Uuid::new_v4().to_string(),
                        chain_to_db(setting.chain),
                        setting.native_symbol,
                        setting.native_symbol,
                        setting.native_decimals,
                        now
                    ],
                )
                .map_err(|_| WalletError::Storage)?;
        }

        transaction.commit().map_err(|_| WalletError::Storage)?;
        Ok(())
    }
}
