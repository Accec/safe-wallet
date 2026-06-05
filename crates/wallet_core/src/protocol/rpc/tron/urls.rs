pub(super) fn tron_rest_url(rpc_url: &str, path: &str) -> String {
    let mut base = rpc_url.trim().trim_end_matches('/');
    if let Some(rest_base) = base.strip_suffix("/jsonrpc") {
        base = rest_base;
    }
    if base.ends_with("/wallet") && path.starts_with("/wallet/") {
        let path = path.trim_start_matches("/wallet");
        return format!("{base}{path}");
    }
    if path.starts_with('/') {
        format!("{base}{path}")
    } else {
        format!("{base}/{path}")
    }
}
