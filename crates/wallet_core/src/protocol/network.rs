use crate::error::WalletError;
use crate::models::{NetworkPrivacySettings, ProxyMode};
use reqwest::blocking::Client;
use reqwest::{Proxy, Url};
use std::time::Duration;

pub const DEFAULT_TOR_PROXY_URL: &str = "socks5h://127.0.0.1:9050";

pub fn default_network_privacy_settings() -> NetworkPrivacySettings {
    NetworkPrivacySettings {
        proxy_enabled: false,
        proxy_mode: ProxyMode::Custom,
        proxy_url: None,
    }
}

pub fn normalize_network_privacy_settings(
    mut settings: NetworkPrivacySettings,
) -> Result<NetworkPrivacySettings, WalletError> {
    if !settings.proxy_enabled {
        settings.proxy_url = None;
        return Ok(settings);
    }
    let proxy_url = match settings.proxy_mode {
        ProxyMode::Tor => settings
            .proxy_url
            .as_deref()
            .map(str::trim)
            .filter(|url| !url.is_empty())
            .unwrap_or(DEFAULT_TOR_PROXY_URL)
            .to_string(),
        ProxyMode::Custom => settings
            .proxy_url
            .as_deref()
            .map(str::trim)
            .filter(|url| !url.is_empty())
            .ok_or(WalletError::InvalidProxySettings)?
            .to_string(),
    };
    validate_proxy_url(&proxy_url)?;
    settings.proxy_url = Some(proxy_url);
    Ok(settings)
}

pub fn validate_proxy_url(value: &str) -> Result<(), WalletError> {
    let url = Url::parse(value.trim()).map_err(|_| WalletError::InvalidProxySettings)?;
    match url.scheme() {
        "http" | "https" | "socks5" | "socks5h" => {}
        _ => return Err(WalletError::InvalidProxySettings),
    }
    if url.host_str().is_none() {
        return Err(WalletError::InvalidProxySettings);
    }
    Ok(())
}

pub fn build_http_client(
    settings: &NetworkPrivacySettings,
    timeout: Duration,
) -> Result<Client, WalletError> {
    let settings = normalize_network_privacy_settings(settings.clone())?;
    let mut builder = Client::builder().timeout(timeout);
    if settings.proxy_enabled {
        let proxy_url = settings
            .proxy_url
            .as_deref()
            .ok_or(WalletError::InvalidProxySettings)?;
        let proxy = Proxy::all(proxy_url).map_err(|_| WalletError::InvalidProxySettings)?;
        builder = builder.proxy(proxy);
    }
    builder.build().map_err(|_| WalletError::NetworkUnavailable)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::WalletError;
    use crate::models::{NetworkPrivacySettings, ProxyMode};

    #[test]
    fn proxy_url_validation_accepts_http_https_and_socks() {
        for url in [
            "http://127.0.0.1:8080",
            "https://127.0.0.1:8443",
            "socks5://127.0.0.1:9050",
            "socks5h://127.0.0.1:9050",
        ] {
            assert_eq!(validate_proxy_url(url), Ok(()));
        }
    }

    #[test]
    fn proxy_url_validation_rejects_invalid_values() {
        for url in ["", "127.0.0.1:9050", "ftp://127.0.0.1:21", "socks5://"] {
            assert_eq!(
                validate_proxy_url(url),
                Err(WalletError::InvalidProxySettings)
            );
        }
    }

    #[test]
    fn tor_mode_uses_default_socks_proxy_with_remote_dns_when_url_is_empty() {
        let settings = normalize_network_privacy_settings(NetworkPrivacySettings {
            proxy_enabled: true,
            proxy_mode: ProxyMode::Tor,
            proxy_url: None,
        })
        .unwrap();

        assert_eq!(settings.proxy_url.as_deref(), Some(DEFAULT_TOR_PROXY_URL));
        assert_eq!(
            settings.proxy_url.as_deref(),
            Some("socks5h://127.0.0.1:9050")
        );
    }

    #[test]
    fn disabled_proxy_clears_url() {
        let settings = normalize_network_privacy_settings(NetworkPrivacySettings {
            proxy_enabled: false,
            proxy_mode: ProxyMode::Custom,
            proxy_url: Some("socks5://127.0.0.1:9050".to_string()),
        })
        .unwrap();

        assert!(!settings.proxy_enabled);
        assert_eq!(settings.proxy_url, None);
    }
}
