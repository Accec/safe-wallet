use serde::Deserialize;

#[derive(Deserialize)]
pub(in crate::protocol) struct PasswordPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) password: String,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct DuressPasswordPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) master_password: String,
    pub(in crate::protocol) duress_password: String,
}

#[derive(Deserialize)]
pub(in crate::protocol) struct BiometricUnlockPayload {
    pub(in crate::protocol) db_path: String,
    pub(in crate::protocol) password: String,
    pub(in crate::protocol) enabled: bool,
}
