use std::fmt;

pub const CIPHER_NAME: &str = "AES-256-GCM";
pub const SECRET_KIND_MNEMONIC: &str = "mnemonic";
pub const SECRET_KIND_PRIVATE_KEY: &str = "private_key";

#[derive(Clone)]
pub struct EncryptedKeystore {
    pub secret_kind: String,
    pub ciphertext_b64: String,
    pub nonce_b64: String,
    pub salt_b64: String,
    pub kdf_name: String,
    pub kdf_params_json: String,
    pub cipher_name: String,
    pub version: u32,
}

impl fmt::Debug for EncryptedKeystore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EncryptedKeystore")
            .field("secret_kind", &self.secret_kind)
            .field("ciphertext_b64", &"<redacted>")
            .field("nonce_b64", &"<redacted>")
            .field("salt_b64", &"<redacted>")
            .field("kdf_name", &self.kdf_name)
            .field("kdf_params_json", &self.kdf_params_json)
            .field("cipher_name", &self.cipher_name)
            .field("version", &self.version)
            .finish()
    }
}
