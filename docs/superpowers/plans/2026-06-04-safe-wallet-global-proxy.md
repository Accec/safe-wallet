# Safe Wallet Global Proxy Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a persisted global proxy setting for Safe Wallet so all Rust RPC and indexer HTTP requests can route through ordinary HTTP/HTTPS proxies or Tor SOCKS proxies.

**Architecture:** Store network privacy settings in SQLite, expose them through `WalletService` and FFI, and build all Rust `reqwest::blocking::Client` instances through one shared network module. Flutter adds a settings UI that saves and tests proxy settings while keeping privacy claims precise.

**Tech Stack:** Rust, rusqlite, reqwest blocking client with rustls and socks support, Flutter/Dart, existing FFI JSON command bridge, existing widget tests.

---

## File Structure

- Create `crates/wallet_core/src/network.rs`: proxy URL validation, Tor defaults, and `reqwest` client construction.
- Modify `crates/wallet_core/src/models.rs`: add `ProxyMode` and `NetworkPrivacySettings`.
- Modify `crates/wallet_core/src/error.rs`: add safe proxy errors.
- Modify `crates/wallet_core/src/storage.rs`: create and read/write the single-row `network_privacy_settings` table.
- Modify `crates/wallet_core/src/service.rs`: expose settings APIs and construct RPC/indexer clients from persisted settings.
- Modify `crates/wallet_core/src/rpc.rs`: accept network privacy settings in the client constructor.
- Modify `crates/wallet_core/src/indexers.rs`: accept network privacy settings in the client constructor.
- Modify `Cargo.toml`: enable the `reqwest` `socks` feature.
- Modify `crates/wallet_core/src/lib.rs`: export the new `network` module.
- Modify `crates/wallet_ffi/src/lib.rs`: add settings/test commands and JSON tests.
- Modify `apps/flutter_wallet/lib/src/models.dart`: add Dart proxy settings models.
- Modify `apps/flutter_wallet/lib/src/wallet_api.dart`: add abstract/Demo API methods.
- Modify `apps/flutter_wallet/lib/src/native_wallet_api.dart`: add native commands and JSON parsing.
- Modify `apps/flutter_wallet/lib/src/screens/settings_screen.dart`: add `Network privacy` UI and proxy dialog.
- Modify `apps/flutter_wallet/lib/src/screens/unlock_screen.dart`, `apps/flutter_wallet/lib/main.dart`, and `apps/flutter_wallet/README.md`: rename visible product surface to Safe Wallet.
- Modify `apps/flutter_wallet/test/native_wallet_api_test.dart` and `apps/flutter_wallet/test/wallet_workflow_test.dart`: add FFI/Dart/UI tests.

## Task 1: Rust Proxy Models and Validation

**Files:**
- Create: `crates/wallet_core/src/network.rs`
- Modify: `crates/wallet_core/src/models.rs`
- Modify: `crates/wallet_core/src/error.rs`
- Modify: `crates/wallet_core/src/lib.rs`
- Modify: `Cargo.toml`

- [ ] **Step 1: Write failing network validation tests**

Add tests to the new `crates/wallet_core/src/network.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{NetworkPrivacySettings, ProxyMode};

    #[test]
    fn proxy_url_validation_accepts_http_https_and_socks() {
        for url in [
            "http://127.0.0.1:8080",
            "https://127.0.0.1:8443",
            "socks5://127.0.0.1:9050",
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
    fn tor_mode_uses_default_socks_proxy_when_url_is_empty() {
        let settings = normalize_network_privacy_settings(NetworkPrivacySettings {
            proxy_enabled: true,
            proxy_mode: ProxyMode::Tor,
            proxy_url: None,
        })
        .unwrap();

        assert_eq!(settings.proxy_url.as_deref(), Some(DEFAULT_TOR_PROXY_URL));
    }

    #[test]
    fn disabled_proxy_clears_url() {
        let settings = normalize_network_privacy_settings(NetworkPrivacySettings {
            proxy_enabled: false,
            proxy_mode: ProxyMode::Custom,
            proxy_url: Some("socks5://127.0.0.1:9050".to_string()),
        })
        .unwrap();

        assert_eq!(settings.proxy_enabled, false);
        assert_eq!(settings.proxy_url, None);
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p wallet_core network::tests::proxy_url_validation_accepts_http_https_and_socks`

Expected: FAIL because `network` module and proxy models do not exist.

- [ ] **Step 3: Implement proxy models, validation, and client builder**

Add to `crates/wallet_core/src/models.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyMode {
    Custom,
    Tor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkPrivacySettings {
    pub proxy_enabled: bool,
    pub proxy_mode: ProxyMode,
    pub proxy_url: Option<String>,
}
```

Add errors to `crates/wallet_core/src/error.rs`:

```rust
#[error("Invalid proxy settings")]
InvalidProxySettings,
#[error("Proxy connection failed")]
ProxyConnectionFailed,
```

Add safe messages:

```rust
WalletError::InvalidProxySettings => "Invalid proxy settings",
WalletError::ProxyConnectionFailed => "Proxy connection failed",
```

Enable SOCKS support in root `Cargo.toml`:

```toml
reqwest = { version = "0.12", default-features = false, features = ["blocking", "json", "rustls-tls", "socks"] }
```

Add `pub mod network;` to `crates/wallet_core/src/lib.rs`.

Create `crates/wallet_core/src/network.rs`:

```rust
use crate::error::WalletError;
use crate::models::{NetworkPrivacySettings, ProxyMode};
use reqwest::blocking::Client;
use reqwest::{Proxy, Url};
use std::time::Duration;

pub const DEFAULT_TOR_PROXY_URL: &str = "socks5://127.0.0.1:9050";

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
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p wallet_core network::tests`

Expected: PASS for all network module tests.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml Cargo.lock crates/wallet_core/src/error.rs crates/wallet_core/src/lib.rs crates/wallet_core/src/models.rs crates/wallet_core/src/network.rs
git commit -m "feat: add network proxy settings model"
```

## Task 2: Persist Network Privacy Settings

**Files:**
- Modify: `crates/wallet_core/src/storage.rs`
- Modify: `crates/wallet_core/src/service.rs`

- [ ] **Step 1: Write failing storage/service tests**

Add to `crates/wallet_core/src/storage.rs` tests:

```rust
#[test]
fn network_privacy_settings_persist_to_sqlite() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let database = WalletDatabase::new(&db_path);
    database.initialize().unwrap();

    let settings = crate::models::NetworkPrivacySettings {
        proxy_enabled: true,
        proxy_mode: crate::models::ProxyMode::Custom,
        proxy_url: Some("http://127.0.0.1:8080".to_string()),
    };
    database.save_network_privacy_settings(&settings).unwrap();

    let reopened = WalletDatabase::new(&db_path);
    reopened.initialize().unwrap();
    assert_eq!(reopened.network_privacy_settings().unwrap(), settings);
}
```

Add to `crates/wallet_core/src/service.rs` tests:

```rust
#[test]
fn tor_network_privacy_settings_default_to_local_socks_proxy() {
    let fixture = service_fixture();

    fixture
        .service
        .save_network_privacy_settings(crate::models::NetworkPrivacySettings {
            proxy_enabled: true,
            proxy_mode: crate::models::ProxyMode::Tor,
            proxy_url: None,
        })
        .unwrap();

    let settings = fixture.service.network_privacy_settings().unwrap();
    assert_eq!(settings.proxy_enabled, true);
    assert_eq!(settings.proxy_mode, crate::models::ProxyMode::Tor);
    assert_eq!(
        settings.proxy_url.as_deref(),
        Some(crate::network::DEFAULT_TOR_PROXY_URL)
    );
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p wallet_core network_privacy_settings_persist_to_sqlite tor_network_privacy_settings_default_to_local_socks_proxy`

Expected: FAIL because storage and service methods do not exist.

- [ ] **Step 3: Implement storage table and methods**

In `WalletDatabase::initialize`, add the table:

```sql
create table if not exists network_privacy_settings (
    id integer primary key check (id = 1),
    proxy_enabled integer not null,
    proxy_mode text not null,
    proxy_url text,
    updated_at text not null
);
```

Add methods to `impl WalletDatabase`:

```rust
pub fn network_privacy_settings(
    &self,
) -> Result<crate::models::NetworkPrivacySettings, WalletError> {
    let connection = self.connect()?;
    connection
        .query_row(
            "select proxy_enabled, proxy_mode, proxy_url from network_privacy_settings where id = 1",
            [],
            |row| {
                let proxy_enabled: i64 = row.get(0)?;
                let proxy_mode: String = row.get(1)?;
                Ok(crate::models::NetworkPrivacySettings {
                    proxy_enabled: proxy_enabled != 0,
                    proxy_mode: parse_proxy_mode(&proxy_mode),
                    proxy_url: row.get(2)?,
                })
            },
        )
        .optional()
        .map_err(|_| WalletError::Storage)
        .map(|settings| settings.unwrap_or_else(crate::network::default_network_privacy_settings))
}

pub fn save_network_privacy_settings(
    &self,
    settings: &crate::models::NetworkPrivacySettings,
) -> Result<(), WalletError> {
    let connection = self.connect()?;
    let now = Utc::now().to_rfc3339();
    connection
        .execute(
            "insert into network_privacy_settings (
                id, proxy_enabled, proxy_mode, proxy_url, updated_at
            ) values (1, ?1, ?2, ?3, ?4)
            on conflict(id) do update set
                proxy_enabled = excluded.proxy_enabled,
                proxy_mode = excluded.proxy_mode,
                proxy_url = excluded.proxy_url,
                updated_at = excluded.updated_at",
            params![
                if settings.proxy_enabled { 1 } else { 0 },
                proxy_mode_label(settings.proxy_mode),
                settings.proxy_url,
                now
            ],
        )
        .map_err(|_| WalletError::Storage)?;
    Ok(())
}
```

Add helpers near other storage helpers:

```rust
fn proxy_mode_label(mode: crate::models::ProxyMode) -> &'static str {
    match mode {
        crate::models::ProxyMode::Custom => "custom",
        crate::models::ProxyMode::Tor => "tor",
    }
}

fn parse_proxy_mode(value: &str) -> crate::models::ProxyMode {
    match value {
        "tor" => crate::models::ProxyMode::Tor,
        _ => crate::models::ProxyMode::Custom,
    }
}
```

Add service methods:

```rust
pub fn network_privacy_settings(
    &self,
) -> Result<crate::models::NetworkPrivacySettings, WalletError> {
    self.database.network_privacy_settings()
}

pub fn save_network_privacy_settings(
    &self,
    settings: crate::models::NetworkPrivacySettings,
) -> Result<(), WalletError> {
    let settings = crate::network::normalize_network_privacy_settings(settings)?;
    self.database.save_network_privacy_settings(&settings)
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p wallet_core network_privacy_settings_persist_to_sqlite tor_network_privacy_settings_default_to_local_socks_proxy`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/wallet_core/src/storage.rs crates/wallet_core/src/service.rs
git commit -m "feat: persist network privacy settings"
```

## Task 3: Route RPC and Indexer Clients Through Proxy Settings

**Files:**
- Modify: `crates/wallet_core/src/rpc.rs`
- Modify: `crates/wallet_core/src/indexers.rs`
- Modify: `crates/wallet_core/src/service.rs`

- [ ] **Step 1: Write failing constructor tests**

Add to `crates/wallet_core/src/rpc.rs` tests:

```rust
#[test]
fn rpc_client_accepts_tor_proxy_settings() {
    let client = RpcNativeBalanceClient::new(&crate::models::NetworkPrivacySettings {
        proxy_enabled: true,
        proxy_mode: crate::models::ProxyMode::Tor,
        proxy_url: None,
    });

    assert!(client.is_ok());
}
```

Add to `crates/wallet_core/src/indexers.rs` tests:

```rust
#[test]
fn activity_indexer_accepts_socks_proxy_settings() {
    let indexer = HttpActivityIndexer::new(&crate::models::NetworkPrivacySettings {
        proxy_enabled: true,
        proxy_mode: crate::models::ProxyMode::Custom,
        proxy_url: Some("socks5://127.0.0.1:9050".to_string()),
    });

    assert!(indexer.is_ok());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p wallet_core rpc_client_accepts_tor_proxy_settings activity_indexer_accepts_socks_proxy_settings`

Expected: FAIL because constructors currently take no settings.

- [ ] **Step 3: Implement settings-aware constructors**

In `rpc.rs`, change:

```rust
impl RpcNativeBalanceClient {
    pub fn new(settings: &crate::models::NetworkPrivacySettings) -> Result<Self, WalletError> {
        let client = crate::network::build_http_client(settings, Duration::from_secs(12))?;
        Ok(Self { client })
    }
}
```

In `indexers.rs`, change:

```rust
impl HttpActivityIndexer {
    pub fn new(settings: &crate::models::NetworkPrivacySettings) -> Result<Self, WalletError> {
        let client = crate::network::build_http_client(settings, Duration::from_secs(20))?;
        Ok(Self { client })
    }
}
```

In `service.rs`, change external client creation:

```rust
let settings = self.database.network_privacy_settings()?;
let client = RpcNativeBalanceClient::new(&settings)?;
```

Use that in `refresh_native_balances`, `refresh_chain_balances`, and `add_custom_token`.

Change `sync_activity`:

```rust
let settings = self.database.network_privacy_settings()?;
let indexer = HttpActivityIndexer::new(&settings)?;
```

- [ ] **Step 4: Run targeted tests**

Run: `cargo test -p wallet_core rpc_client_accepts_tor_proxy_settings activity_indexer_accepts_socks_proxy_settings`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/wallet_core/src/rpc.rs crates/wallet_core/src/indexers.rs crates/wallet_core/src/service.rs
git commit -m "feat: route rust http clients through proxy settings"
```

## Task 4: Add FFI Commands

**Files:**
- Modify: `crates/wallet_ffi/src/lib.rs`

- [ ] **Step 1: Write failing FFI tests**

Add to `crates/wallet_ffi/src/lib.rs` tests:

```rust
#[test]
fn network_privacy_commands_save_and_load_proxy_settings() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let db_path = db_path.to_string_lossy();

    let save = response(&format!(
        r#"{{"command":"save_network_privacy_settings","db_path":"{db_path}","proxy_enabled":true,"proxy_mode":"tor","proxy_url":null}}"#
    ));
    assert!(save.ok);

    let loaded = response(&format!(
        r#"{{"command":"get_network_privacy_settings","db_path":"{db_path}"}}"#
    ));
    assert!(loaded.ok);
    let body: serde_json::Value = serde_json::from_str(&loaded.body_json).unwrap();
    assert_eq!(body["proxy_enabled"], true);
    assert_eq!(body["proxy_mode"], "tor");
    assert_eq!(body["proxy_url"], "socks5://127.0.0.1:9050");
}

#[test]
fn invalid_proxy_settings_return_safe_error() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let db_path = db_path.to_string_lossy();

    let response = response(&format!(
        r#"{{"command":"save_network_privacy_settings","db_path":"{db_path}","proxy_enabled":true,"proxy_mode":"custom","proxy_url":"ftp://127.0.0.1:21"}}"#
    ));

    assert!(!response.ok);
    assert_eq!(response.error.as_deref(), Some("Invalid proxy settings"));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p wallet_ffi network_privacy_commands_save_and_load_proxy_settings invalid_proxy_settings_return_safe_error`

Expected: FAIL because commands do not exist.

- [ ] **Step 3: Implement FFI commands**

Add command variants:

```rust
GetNetworkPrivacySettings {
    db_path: String,
},
SaveNetworkPrivacySettings {
    db_path: String,
    proxy_enabled: bool,
    proxy_mode: String,
    proxy_url: Option<String>,
},
TestProxyConnection {
    db_path: String,
    proxy_enabled: bool,
    proxy_mode: String,
    proxy_url: Option<String>,
},
```

Add match arms:

```rust
WalletCommand::GetNetworkPrivacySettings { db_path } => {
    let service = service_for_path(&db_path);
    match service
        .initialize()
        .and_then(|_| service.network_privacy_settings())
    {
        Ok(settings) => ok_json(settings),
        Err(error) => wallet_error_response(error),
    }
}
WalletCommand::SaveNetworkPrivacySettings {
    db_path,
    proxy_enabled,
    proxy_mode,
    proxy_url,
} => {
    let service = service_for_path(&db_path);
    match parse_proxy_mode(&proxy_mode).and_then(|proxy_mode| {
        service.initialize().and_then(|_| {
            service.save_network_privacy_settings(wallet_core::models::NetworkPrivacySettings {
                proxy_enabled,
                proxy_mode,
                proxy_url,
            })
        })
    }) {
        Ok(()) => ok_json(json!({ "saved": true })),
        Err(error) => wallet_error_response(error),
    }
}
WalletCommand::TestProxyConnection {
    db_path: _,
    proxy_enabled,
    proxy_mode,
    proxy_url,
} => match parse_proxy_mode(&proxy_mode).and_then(|proxy_mode| {
    let settings = wallet_core::models::NetworkPrivacySettings {
        proxy_enabled,
        proxy_mode,
        proxy_url,
    };
    wallet_core::network::build_http_client(&settings, std::time::Duration::from_secs(8)).map(|_| ())
}) {
    Ok(()) => ok_json(json!({ "ok": true })),
    Err(error) => wallet_error_response(error),
},
```

Add parser:

```rust
fn parse_proxy_mode(value: &str) -> Result<wallet_core::models::ProxyMode, WalletError> {
    match value {
        "custom" => Ok(wallet_core::models::ProxyMode::Custom),
        "tor" => Ok(wallet_core::models::ProxyMode::Tor),
        _ => Err(WalletError::InvalidProxySettings),
    }
}
```

- [ ] **Step 4: Run FFI tests**

Run: `cargo test -p wallet_ffi network_privacy_commands_save_and_load_proxy_settings invalid_proxy_settings_return_safe_error`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/wallet_ffi/src/lib.rs
git commit -m "feat: expose proxy settings through ffi"
```

## Task 5: Add Dart API and Native Parsing

**Files:**
- Modify: `apps/flutter_wallet/lib/src/models.dart`
- Modify: `apps/flutter_wallet/lib/src/wallet_api.dart`
- Modify: `apps/flutter_wallet/lib/src/native_wallet_api.dart`
- Modify: `apps/flutter_wallet/test/native_wallet_api_test.dart`

- [ ] **Step 1: Write failing Dart native API test**

Add to `apps/flutter_wallet/test/native_wallet_api_test.dart`:

```dart
test('network privacy commands round trip through native wallet api', () async {
  final commands = <Map<String, Object?>>[];
  final api = NativeWalletApi(
    databasePathProvider: () async => '/tmp/wallet.sqlite',
    commandRunner: (command) async {
      commands.add(command);
      switch (command['command']) {
        case 'get_network_privacy_settings':
          return {
            'proxy_enabled': true,
            'proxy_mode': 'tor',
            'proxy_url': 'socks5://127.0.0.1:9050',
          };
        case 'save_network_privacy_settings':
        case 'test_proxy_connection':
          return <String, Object?>{};
      }
      throw const WalletApiException('unexpected command');
    },
  );

  final settings = await api.networkPrivacySettings();
  expect(settings.proxyEnabled, isTrue);
  expect(settings.proxyMode, 'tor');
  expect(settings.proxyUrl, 'socks5://127.0.0.1:9050');

  await api.saveNetworkPrivacySettings(
    const NetworkPrivacySettingsDraft(
      proxyEnabled: true,
      proxyMode: 'custom',
      proxyUrl: 'http://127.0.0.1:8080',
    ),
  );
  await api.testProxyConnection(
    const NetworkPrivacySettingsDraft(
      proxyEnabled: true,
      proxyMode: 'tor',
    ),
  );

  expect(commands[0]['command'], 'get_network_privacy_settings');
  expect(commands[1]['command'], 'save_network_privacy_settings');
  expect(commands[1]['proxy_url'], 'http://127.0.0.1:8080');
  expect(commands[2]['command'], 'test_proxy_connection');
  expect(commands[2]['proxy_mode'], 'tor');
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd apps/flutter_wallet && flutter test test/native_wallet_api_test.dart --plain-name "network privacy commands round trip through native wallet api"`

Expected: FAIL because models and methods do not exist.

- [ ] **Step 3: Implement Dart models and API methods**

Add to `models.dart`:

```dart
class NetworkPrivacySettingsDraft {
  const NetworkPrivacySettingsDraft({
    required this.proxyEnabled,
    required this.proxyMode,
    this.proxyUrl,
  });

  final bool proxyEnabled;
  final String proxyMode;
  final String? proxyUrl;
}

class NetworkPrivacySettings {
  const NetworkPrivacySettings({
    required this.proxyEnabled,
    required this.proxyMode,
    this.proxyUrl,
  });

  final bool proxyEnabled;
  final String proxyMode;
  final String? proxyUrl;
}
```

Add abstract methods to `WalletApi`:

```dart
Future<NetworkPrivacySettings> networkPrivacySettings();
Future<void> saveNetworkPrivacySettings(NetworkPrivacySettingsDraft draft);
Future<void> testProxyConnection(NetworkPrivacySettingsDraft draft);
```

Add Demo implementation storing an in-memory field:

```dart
NetworkPrivacySettings _networkPrivacySettings =
    const NetworkPrivacySettings(proxyEnabled: false, proxyMode: 'custom');

@override
Future<NetworkPrivacySettings> networkPrivacySettings() async {
  return _networkPrivacySettings;
}

@override
Future<void> saveNetworkPrivacySettings(NetworkPrivacySettingsDraft draft) async {
  _networkPrivacySettings = NetworkPrivacySettings(
    proxyEnabled: draft.proxyEnabled,
    proxyMode: draft.proxyMode,
    proxyUrl: draft.proxyEnabled && draft.proxyMode == 'tor'
        ? (draft.proxyUrl ?? 'socks5://127.0.0.1:9050')
        : draft.proxyUrl,
  );
}

@override
Future<void> testProxyConnection(NetworkPrivacySettingsDraft draft) async {}
```

Add `NativeWalletApi` methods and parser:

```dart
@override
Future<NetworkPrivacySettings> networkPrivacySettings() async {
  final body = await _runCommand({
    'command': 'get_network_privacy_settings',
    'db_path': await _databasePath(),
  });
  return _networkPrivacySettingsFromJson(body);
}

@override
Future<void> saveNetworkPrivacySettings(NetworkPrivacySettingsDraft draft) async {
  await _runCommand({
    'command': 'save_network_privacy_settings',
    'db_path': await _databasePath(),
    'proxy_enabled': draft.proxyEnabled,
    'proxy_mode': draft.proxyMode,
    'proxy_url': draft.proxyUrl,
  });
}

@override
Future<void> testProxyConnection(NetworkPrivacySettingsDraft draft) async {
  await _runCommand({
    'command': 'test_proxy_connection',
    'db_path': await _databasePath(),
    'proxy_enabled': draft.proxyEnabled,
    'proxy_mode': draft.proxyMode,
    'proxy_url': draft.proxyUrl,
  });
}
```

Parser:

```dart
NetworkPrivacySettings _networkPrivacySettingsFromJson(Object? value) {
  if (value is! Map<String, Object?>) {
    throw const WalletApiException('Invalid network privacy settings');
  }
  return NetworkPrivacySettings(
    proxyEnabled: _boolField(value, 'proxy_enabled'),
    proxyMode: _stringField(value, 'proxy_mode'),
    proxyUrl: _optionalNativeString(value, 'proxy_url'),
  );
}
```

- [ ] **Step 4: Run Dart native API test**

Run: `cd apps/flutter_wallet && flutter test test/native_wallet_api_test.dart --plain-name "network privacy commands round trip through native wallet api"`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add apps/flutter_wallet/lib/src/models.dart apps/flutter_wallet/lib/src/wallet_api.dart apps/flutter_wallet/lib/src/native_wallet_api.dart apps/flutter_wallet/test/native_wallet_api_test.dart
git commit -m "feat: add flutter proxy api"
```

## Task 6: Add Flutter Network Privacy UI and Safe Wallet Branding

**Files:**
- Modify: `apps/flutter_wallet/lib/src/screens/settings_screen.dart`
- Modify: `apps/flutter_wallet/lib/src/screens/unlock_screen.dart`
- Modify: `apps/flutter_wallet/lib/main.dart`
- Modify: `apps/flutter_wallet/README.md`
- Modify: `apps/flutter_wallet/test/wallet_workflow_test.dart`

- [ ] **Step 1: Write failing widget test**

Add to `apps/flutter_wallet/test/wallet_workflow_test.dart`:

```dart
testWidgets('settings saves and tests tor proxy settings', (tester) async {
  final api = _WorkflowApi();
  api.initialized = true;

  await tester.pumpWidget(MaterialApp(home: SettingsScreen(api: api, onLock: () {})));
  await tester.pumpAndSettle();

  await tester.tap(find.text('Network privacy'));
  await tester.pumpAndSettle();
  await tester.tap(find.byType(Switch).last);
  await tester.pumpAndSettle();
  await tester.tap(find.text('Tor'));
  await tester.pumpAndSettle();

  await tester.tap(find.widgetWithText(OutlinedButton, 'Test connection'));
  await tester.pumpAndSettle();
  expect(find.text('Proxy connection succeeded.'), findsOneWidget);

  await tester.tap(find.widgetWithText(FilledButton, 'Save'));
  await tester.pumpAndSettle();

  expect(api.savedNetworkPrivacy?.proxyEnabled, isTrue);
  expect(api.savedNetworkPrivacy?.proxyMode, 'tor');
  expect(api.savedNetworkPrivacy?.proxyUrl, 'socks5://127.0.0.1:9050');
  expect(api.proxyTested, isTrue);
});
```

Add fields/methods to `_WorkflowApi`:

```dart
NetworkPrivacySettings networkPrivacy = const NetworkPrivacySettings(
  proxyEnabled: false,
  proxyMode: 'custom',
);
NetworkPrivacySettingsDraft? savedNetworkPrivacy;
bool proxyTested = false;

@override
Future<NetworkPrivacySettings> networkPrivacySettings() async => networkPrivacy;

@override
Future<void> saveNetworkPrivacySettings(NetworkPrivacySettingsDraft draft) async {
  savedNetworkPrivacy = draft;
  networkPrivacy = NetworkPrivacySettings(
    proxyEnabled: draft.proxyEnabled,
    proxyMode: draft.proxyMode,
    proxyUrl: draft.proxyUrl,
  );
}

@override
Future<void> testProxyConnection(NetworkPrivacySettingsDraft draft) async {
  proxyTested = true;
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd apps/flutter_wallet && flutter test test/wallet_workflow_test.dart --plain-name "settings saves and tests tor proxy settings"`

Expected: FAIL because UI is not present.

- [ ] **Step 3: Implement UI**

Add state to `SettingsScreen`:

```dart
late Future<NetworkPrivacySettings> _networkPrivacy =
    widget.api.networkPrivacySettings();

void _reloadNetworkPrivacy() {
  setState(() {
    _networkPrivacy = widget.api.networkPrivacySettings();
  });
}
```

Add a `Network privacy` row under settings:

```dart
FutureBuilder<NetworkPrivacySettings>(
  future: _networkPrivacy,
  builder: (context, snapshot) {
    final settings = snapshot.data;
    return ListTile(
      leading: const Icon(Icons.shield_outlined),
      title: const Text('Network privacy'),
      subtitle: Text(
        settings == null
            ? 'Loading proxy settings'
            : settings.proxyEnabled
                ? 'Proxy enabled'
                : 'Direct connection',
      ),
      trailing: const Icon(Icons.chevron_right),
      onTap: settings == null
          ? null
          : () => _showNetworkPrivacyDialog(
                context: context,
                api: widget.api,
                settings: settings,
                onSaved: _reloadNetworkPrivacy,
              ),
    );
  },
),
```

Create `_showNetworkPrivacyDialog` with:

```dart
const _defaultTorProxyUrl = 'socks5://127.0.0.1:9050';
```

The dialog must:

- Use a `SwitchListTile` titled `Use proxy`.
- Use a segmented control or two choice chips for `Custom proxy` and `Tor`.
- Keep a URL `TextField` labeled `Proxy URL`.
- When Tor is selected and the field is empty, fill `socks5://127.0.0.1:9050`.
- Call `api.testProxyConnection(draft)` from `Test connection`.
- Show `Proxy connection succeeded.` or the safe API error.
- Call `api.saveNetworkPrivacySettings(draft)` from `Save`.

Rename visible product text:

```dart
widget.initialized ? 'Safe Wallet' : 'Set master password'
```

Update `README.md` heading to `# Safe Wallet` and description to:

```md
Privacy-focused Flutter shell for the Rust-backed local wallet.
```

- [ ] **Step 4: Run widget test**

Run: `cd apps/flutter_wallet && flutter test test/wallet_workflow_test.dart --plain-name "settings saves and tests tor proxy settings"`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add apps/flutter_wallet/lib/src/screens/settings_screen.dart apps/flutter_wallet/lib/src/screens/unlock_screen.dart apps/flutter_wallet/lib/main.dart apps/flutter_wallet/README.md apps/flutter_wallet/test/wallet_workflow_test.dart
git commit -m "feat: add proxy settings ui"
```

## Task 7: Full Verification

**Files:**
- Verify all changed files.

- [ ] **Step 1: Format Rust and Flutter**

Run:

```bash
cargo fmt
cd apps/flutter_wallet && dart format lib test
```

Expected: both commands exit 0.

- [ ] **Step 2: Run Rust tests**

Run: `cargo test --workspace`

Expected: all Rust tests pass.

- [ ] **Step 3: Run Flutter analyzer and tests**

Run:

```bash
cd apps/flutter_wallet && flutter analyze && flutter test
```

Expected: analyzer reports no issues and all Flutter tests pass.

- [ ] **Step 4: Final status**

Summarize:

- Proxy settings persist locally.
- RPC and indexer clients share the same proxy settings.
- HTTP/HTTPS/SOCKS5 and Tor preset are supported.
- Safe Wallet branding is visible.
- Verification commands and pass/fail outcomes.
