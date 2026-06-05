pub(super) fn is_http_url(value: &str) -> bool {
    value.starts_with("https://") || value.starts_with("http://")
}
