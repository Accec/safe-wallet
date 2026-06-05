use crate::models::KeystoreExport;

use super::EncryptedKeystore;

pub fn encrypted_keystore_from_export(export: &KeystoreExport) -> EncryptedKeystore {
    EncryptedKeystore {
        secret_kind: export.secret_kind.clone(),
        ciphertext_b64: export.ciphertext_b64.clone(),
        nonce_b64: export.nonce_b64.clone(),
        salt_b64: export.salt_b64.clone(),
        kdf_name: export.kdf_name.clone(),
        kdf_params_json: export.kdf_params_json.clone(),
        cipher_name: export.cipher_name.clone(),
        version: export.version,
    }
}
