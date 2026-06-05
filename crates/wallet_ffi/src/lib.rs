mod legacy;
mod protocol;
mod response;

use serde_json::Value;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use response::error_response;
pub use response::WalletResponse;

pub fn handle_command_json(command_json: &str) -> WalletResponse {
    let Ok(value) = serde_json::from_str::<Value>(command_json) else {
        return error_response("Invalid command");
    };
    if value.get("protocol_version").is_some() {
        return protocol::handle_v2_command_value(value);
    }
    match serde_json::from_value::<legacy::WalletCommand>(value) {
        Ok(command) => legacy::handle_command(command),
        Err(_) => error_response("Invalid command"),
    }
}

#[no_mangle]
/// # Safety
///
/// `command_json` must be either null or a valid, NUL-terminated C string.
pub unsafe extern "C" fn wallet_command_json_c(command_json: *const c_char) -> *mut c_char {
    let response = if command_json.is_null() {
        error_response("Invalid command")
    } else {
        let command = unsafe { CStr::from_ptr(command_json) }
            .to_string_lossy()
            .to_string();
        handle_command_json(&command)
    };

    let json = serde_json::to_string(&response).unwrap_or_else(|_| {
        r#"{"ok":false,"body_json":"{}","error":"Serialization failed"}"#.to_string()
    });
    CString::new(json)
        .unwrap_or_else(|_| {
            CString::new(r#"{"ok":false,"body_json":"{}","error":"Serialization failed"}"#).unwrap()
        })
        .into_raw()
}

#[no_mangle]
/// # Safety
///
/// `value` must be either null or a pointer returned by `wallet_command_json_c`.
pub unsafe extern "C" fn wallet_string_free(value: *mut c_char) {
    if value.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(value);
    }
}

#[cfg(test)]
mod tests;
