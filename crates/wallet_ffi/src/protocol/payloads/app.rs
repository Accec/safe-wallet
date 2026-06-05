use serde::Deserialize;

#[derive(Deserialize)]
pub(in crate::protocol) struct AppStatusPayload {
    pub(in crate::protocol) db_path: Option<String>,
}
