use serde_json::Value;
use wallet_core::protocol::{ProtocolRequest, ProtocolResponse, PROTOCOL_VERSION};

use crate::response::{ok_json, WalletResponse};

mod activity;
mod app;
mod assets;
mod auth;
mod error;
mod multisig;
mod network;
mod payloads;
mod support;
mod transfers;
mod wallets;

use error::{CommandError, CommandResult};

pub(crate) fn handle_v2_command_value(value: Value) -> WalletResponse {
    let response = match serde_json::from_value::<ProtocolRequest>(value) {
        Ok(request) => handle_v2_command(request),
        Err(_) => ProtocolResponse::error("unknown", "invalid_request", "Invalid request", false),
    };
    ok_json(response)
}

fn handle_v2_command(request: ProtocolRequest) -> ProtocolResponse {
    let request_id = request.request_id.clone();
    if request.protocol_version != PROTOCOL_VERSION {
        return ProtocolResponse::error(
            request_id,
            "unsupported_protocol",
            "Unsupported protocol version",
            false,
        );
    }

    match dispatch_v2_command(&request) {
        Ok(data) => ProtocolResponse::ok(request_id, data),
        Err(error) => {
            ProtocolResponse::error(request_id, error.code, error.message, error.retryable)
        }
    }
}

fn dispatch_v2_command(request: &ProtocolRequest) -> CommandResult {
    match (request.domain.as_str(), request.action.as_str()) {
        ("app", "status") => app::status(&request.payload),
        ("auth", "set_master_password") => auth::set_master_password(&request.payload),
        ("auth", "unlock") => auth::unlock_app(&request.payload),
        ("auth", "set_duress_password") => auth::set_duress_password(&request.payload),
        ("auth", "update_biometric_unlock") => auth::update_biometric_unlock(&request.payload),
        ("wallets", "generate_mnemonic") => wallets::generate_mnemonic(),
        ("wallets", "create") | ("wallets", "import_mnemonic") => {
            wallets::create_wallet(&request.payload)
        }
        ("wallets", "import_private_key") => wallets::import_private_key(&request.payload),
        ("wallets", "import_keystore") => wallets::import_keystore(&request.payload),
        ("wallets", "export_keystore") => wallets::export_keystore(&request.payload),
        ("wallets", "delete") => wallets::delete_wallet(&request.payload),
        ("wallets", "list") => wallets::list_wallets(&request.payload),
        ("wallets", "list_accounts") => wallets::list_accounts(&request.payload),
        ("assets", "list") => assets::list_assets(&request.payload),
        ("assets", "refresh") => assets::refresh_assets(&request.payload),
        ("assets", "discover") => assets::discover_assets(&request.payload),
        ("assets", "add_custom_token") => assets::add_custom_token(&request.payload),
        ("assets", "remove_custom_token") => assets::remove_custom_token(&request.payload),
        ("transfers", "parse_payment_uri") => transfers::parse_payment_uri(&request.payload),
        ("transfers", "preview") => transfers::preview_transfer(&request.payload),
        ("transfers", "send") => transfers::send_transfer(&request.payload),
        ("activity", "list") => activity::list_activity(&request.payload),
        ("activity", "sync") => activity::sync_activity(&request.payload),
        ("multisig", "import_account") => multisig::import_account(&request.payload),
        ("multisig", "list_accounts") => multisig::list_accounts(&request.payload),
        ("multisig", "create_proposal") => multisig::create_proposal(&request.payload),
        ("multisig", "list_proposals") => multisig::list_proposals(&request.payload),
        ("multisig", "add_signature") => multisig::add_signature(&request.payload),
        ("network", "get_privacy") => network::get_privacy(&request.payload),
        ("network", "save_privacy") => network::save_privacy(&request.payload),
        ("network", "test_proxy") => network::test_proxy_connection(&request.payload),
        ("network", "list_settings") => network::list_settings(&request.payload),
        ("network", "save_settings") => network::save_settings(&request.payload),
        ("network", "update_chain_rpc") => network::update_chain_rpc(&request.payload),
        ("network", "update_indexer") => network::update_indexer_settings(&request.payload),
        _ => Err(CommandError::new(
            "unknown_action",
            "Unknown wallet action",
            false,
        )),
    }
}
