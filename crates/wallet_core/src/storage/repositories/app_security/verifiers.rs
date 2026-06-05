use super::super::super::mappers::*;
use super::super::AppSecurityRepository;
use crate::error::WalletError;
use chrono::Utc;
use rusqlite::{params, OptionalExtension};

impl AppSecurityRepository {
    pub fn save_master_password_verifier(
        &self,
        verifier: &crate::domains::auth::security::MasterPasswordVerifier,
    ) -> Result<(), WalletError> {
        let connection = self.database.connect()?;
        let now = Utc::now().to_rfc3339();
        connection
            .execute(
                "insert into app_security (
                    id, password_salt, password_verifier, kdf_name, kdf_params_json,
                    security_version, created_at, updated_at
                ) values (1, ?1, ?2, ?3, ?4, ?5, ?6, ?6)",
                params![
                    verifier.salt_b64,
                    verifier.verifier_b64,
                    verifier.kdf_name,
                    verifier.kdf_params_json,
                    verifier.version,
                    now
                ],
            )
            .map_err(map_app_security_insert_error)?;
        Ok(())
    }

    pub fn load_master_password_verifier(
        &self,
    ) -> Result<Option<crate::domains::auth::security::MasterPasswordVerifier>, WalletError> {
        let connection = self.database.connect()?;
        connection
            .query_row(
                "select password_salt, password_verifier, kdf_name, kdf_params_json, security_version
                from app_security where id = 1",
                [],
                |row| {
                    Ok(crate::domains::auth::security::MasterPasswordVerifier {
                        salt_b64: row.get(0)?,
                        verifier_b64: row.get(1)?,
                        kdf_name: row.get(2)?,
                        kdf_params_json: row.get(3)?,
                        version: row.get(4)?,
                    })
                },
            )
            .optional()
            .map_err(|_| WalletError::Storage)
    }

    pub fn save_duress_password_verifier(
        &self,
        verifier: &crate::domains::auth::security::MasterPasswordVerifier,
    ) -> Result<(), WalletError> {
        let connection = self.database.connect()?;
        let updated = connection
            .execute(
                "update app_security
                set duress_salt = ?1,
                    duress_verifier = ?2,
                    duress_kdf_name = ?3,
                    duress_kdf_params_json = ?4,
                    duress_security_version = ?5,
                    updated_at = ?6
                where id = 1",
                params![
                    verifier.salt_b64,
                    verifier.verifier_b64,
                    verifier.kdf_name,
                    verifier.kdf_params_json,
                    verifier.version,
                    Utc::now().to_rfc3339(),
                ],
            )
            .map_err(|_| WalletError::Storage)?;
        if updated == 0 {
            return Err(WalletError::InvalidPassword);
        }
        Ok(())
    }

    pub fn load_duress_password_verifier(
        &self,
    ) -> Result<Option<crate::domains::auth::security::MasterPasswordVerifier>, WalletError> {
        let connection = self.database.connect()?;
        connection
            .query_row(
                "select duress_salt, duress_verifier, duress_kdf_name,
                    duress_kdf_params_json, duress_security_version
                from app_security where id = 1",
                [],
                |row| {
                    let salt_b64: Option<String> = row.get(0)?;
                    let verifier_b64: Option<String> = row.get(1)?;
                    let kdf_name: Option<String> = row.get(2)?;
                    let kdf_params_json: Option<String> = row.get(3)?;
                    let version: Option<u32> = row.get(4)?;
                    Ok(
                        match (salt_b64, verifier_b64, kdf_name, kdf_params_json, version) {
                            (
                                Some(salt_b64),
                                Some(verifier_b64),
                                Some(kdf_name),
                                Some(kdf_params_json),
                                Some(version),
                            ) => Some(crate::domains::auth::security::MasterPasswordVerifier {
                                salt_b64,
                                verifier_b64,
                                kdf_name,
                                kdf_params_json,
                                version,
                            }),
                            _ => None,
                        },
                    )
                },
            )
            .optional()
            .map(|value| value.flatten())
            .map_err(|_| WalletError::Storage)
    }
}
