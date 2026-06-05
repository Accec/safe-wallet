use std::fmt;

pub const KDF_NAME: &str = "scrypt";
pub const KDF_PARAMS_JSON: &str = r#"{"log_n":15,"r":8,"p":1,"output_len":32}"#;
pub const CRYPTO_VERSION: u32 = 1;

#[derive(Clone)]
pub struct MasterPasswordVerifier {
    pub salt_b64: String,
    pub verifier_b64: String,
    pub kdf_name: String,
    pub kdf_params_json: String,
    pub version: u32,
}

impl fmt::Debug for MasterPasswordVerifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MasterPasswordVerifier")
            .field("salt_b64", &"<redacted>")
            .field("verifier_b64", &"<redacted>")
            .field("kdf_name", &self.kdf_name)
            .field("kdf_params_json", &self.kdf_params_json)
            .field("version", &self.version)
            .finish()
    }
}

impl MasterPasswordVerifier {
    pub(super) fn has_supported_metadata(&self) -> bool {
        self.kdf_name == KDF_NAME
            && self.kdf_params_json == KDF_PARAMS_JSON
            && self.version == CRYPTO_VERSION
    }
}
