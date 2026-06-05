use super::super::super::mappers::*;
use super::super::WalletRepository;
use crate::domains::wallets::keystore::EncryptedKeystore;
use crate::error::WalletError;
use crate::models::WalletSummary;
use rusqlite::{params, OptionalExtension};
use uuid::Uuid;

impl WalletRepository {
    pub fn list_wallets(&self) -> Result<Vec<WalletSummary>, WalletError> {
        let connection = self.database.connect()?;
        let mut statement = connection
            .prepare(
                "select id, label, created_at
                from wallets
                where hidden = 0
                order by created_at asc, id asc",
            )
            .map_err(|_| WalletError::Storage)?;
        let wallets = statement
            .query_map([], wallet_summary_from_row)
            .map_err(|_| WalletError::Storage)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| WalletError::Storage)?;
        Ok(wallets)
    }

    pub fn load_wallet_summary(&self, wallet_id: Uuid) -> Result<WalletSummary, WalletError> {
        let connection = self.database.connect()?;
        connection
            .query_row(
                "select id, label, created_at
                from wallets
                where id = ?1 and hidden = 0",
                params![wallet_id.to_string()],
                wallet_summary_from_row,
            )
            .optional()
            .map_err(|_| WalletError::Storage)?
            .ok_or(WalletError::WalletNotFound)
    }

    pub fn load_wallet_keystore(&self, wallet_id: Uuid) -> Result<EncryptedKeystore, WalletError> {
        if !self.wallet_exists(wallet_id)? {
            return Err(WalletError::WalletNotFound);
        }
        let connection = self.database.connect()?;
        connection
            .query_row(
                "select secret_kind, ciphertext, nonce, salt, kdf_name, kdf_params_json, cipher_name, version
                from keystore_items
                where id = ?1",
                params![wallet_id.to_string()],
                |row| {
                    Ok(EncryptedKeystore {
                        secret_kind: row.get(0)?,
                        ciphertext_b64: row.get(1)?,
                        nonce_b64: row.get(2)?,
                        salt_b64: row.get(3)?,
                        kdf_name: row.get(4)?,
                        kdf_params_json: row.get(5)?,
                        cipher_name: row.get(6)?,
                        version: row.get(7)?,
                    })
                },
            )
            .optional()
            .map_err(|_| WalletError::Storage)?
            .ok_or(WalletError::WalletNotFound)
    }

    pub fn wallet_exists(&self, wallet_id: Uuid) -> Result<bool, WalletError> {
        let connection = self.database.connect()?;
        let exists = connection
            .query_row(
                "select exists(select 1 from wallets where id = ?1 and hidden = 0)",
                params![wallet_id.to_string()],
                |row| row.get::<_, i64>(0),
            )
            .map_err(|_| WalletError::Storage)?;
        Ok(exists == 1)
    }
}
