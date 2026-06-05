# Safe Wallet Global Proxy Design

## Status

Approved for implementation on 2026-06-04.

## Context

Safe Wallet is a Rust-backed Flutter wallet that stores wallet data locally and
uses Rust `reqwest` clients for external network calls. The current code has two
primary outbound HTTP paths:

- `RpcNativeBalanceClient` in `crates/wallet_core/src/rpc.rs` for RPC balance
  reads, contract metadata reads, and token balance reads.
- `HttpActivityIndexer` in `crates/wallet_core/src/indexers.rs` for third-party
  activity history indexing.

The proxy feature must cover both paths. A Flutter-only setting would be easy to
miss when new Rust network calls are added, so proxy configuration belongs in a
shared Rust network configuration layer.

## Goals

- Rename the product surface to Safe Wallet for the open-source project.
- Add global proxy support for every Rust HTTP request used by RPC and indexer
  operations.
- Support ordinary proxies and Tor-style SOCKS proxies:
  - `http://host:port`
  - `https://host:port`
  - `socks5://host:port`
  - `socks5h://host:port` if supported cleanly by `reqwest`
  - Tor default preset: `socks5://127.0.0.1:9050`
- Persist proxy settings in the local SQLite database.
- Expose a settings UI for enabling, editing, and testing proxy connectivity.
- Keep privacy language accurate: proxy routing reduces IP exposure to RPC and
  indexer services, but it is not a guarantee of absolute anonymity or absolute
  safety.

## Non-Goals

- Bundling or running a Tor daemon inside the wallet.
- Guaranteeing that a proxy provider or Tor exit node is trustworthy.
- Hiding IPs from all operating-system or platform-level network activity
  outside Safe Wallet.
- Preventing address clustering, timing analysis, browser fingerprinting, or
  other non-IP privacy leaks.
- Proxying future native SDKs that bypass Rust `reqwest` unless they are
  explicitly routed through the same Rust network abstraction.

## User Experience

Settings adds a new section:

```text
Settings
  Network privacy
    Proxy
```

The proxy screen contains:

- `Use proxy` switch.
- Mode selector:
  - `Custom proxy`
  - `Tor`
- Proxy URL field for custom mode.
- Tor preset field showing `socks5://127.0.0.1:9050`.
- `Test connection` button.
- Status text for the last test result.

Suggested copy:

- Title: `Network privacy`
- Proxy row subtitle when disabled: `Direct connection`
- Proxy row subtitle when enabled: `Proxy enabled`
- Explanation: `Routes Safe Wallet RPC and indexer requests through a proxy. This
  can reduce IP exposure to network services, but it does not guarantee
  anonymity.`
- Tor mode help: `Requires Tor or a compatible SOCKS proxy running locally.`
- Test success: `Proxy connection succeeded.`
- Test failure: `Proxy connection failed.`

The app must not claim "absolute safety" in the UI or README. The project can
say it is a privacy-focused local wallet.

## Data Model

Add a persistent network privacy settings record:

```rust
pub enum ProxyMode {
    Custom,
    Tor,
}

pub struct NetworkPrivacySettings {
    pub proxy_enabled: bool,
    pub proxy_mode: ProxyMode,
    pub proxy_url: Option<String>,
}
```

SQLite can store this as a single-row table:

```sql
CREATE TABLE IF NOT EXISTS network_privacy_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    proxy_enabled INTEGER NOT NULL,
    proxy_mode TEXT NOT NULL,
    proxy_url TEXT
);
```

Default settings:

- `proxy_enabled = false`
- `proxy_mode = custom`
- `proxy_url = NULL`

When Tor mode is saved with an empty URL, the backend should persist
`socks5://127.0.0.1:9050`.

## Rust Architecture

Add a small network module, for example `crates/wallet_core/src/network.rs`, with
one responsibility: building HTTP clients from persisted network privacy
settings.

Proposed API:

```rust
pub fn build_http_client(
    settings: &NetworkPrivacySettings,
    timeout: Duration,
) -> Result<reqwest::blocking::Client, WalletError>;

pub fn validate_proxy_url(value: &str) -> Result<(), WalletError>;
```

Validation rules:

- Empty URL is invalid when proxy is enabled.
- Allowed schemes: `http`, `https`, `socks5`, and optionally `socks5h`.
- Host must be present.
- Port is recommended but not strictly required if `reqwest` accepts the URL.
- Credentials in proxy URLs are allowed only through standard URL syntax.

`Cargo.toml` must enable the reqwest `socks` feature so SOCKS/Tor proxies work.

`WalletService` should create network clients through this shared builder:

- `RpcNativeBalanceClient::new(settings)`
- `HttpActivityIndexer::new(settings)`

This makes the routing decision centralized and testable.

## FFI and Flutter API

Add commands:

- `get_network_privacy_settings`
- `save_network_privacy_settings`
- `test_proxy_connection`

Flutter models:

```dart
class NetworkPrivacySettings {
  final bool proxyEnabled;
  final String proxyMode;
  final String? proxyUrl;
}
```

Flutter API methods:

- `networkPrivacySettings()`
- `saveNetworkPrivacySettings(NetworkPrivacySettingsDraft draft)`
- `testProxyConnection(NetworkPrivacySettingsDraft draft)`

`testProxyConnection` should not need to persist settings before testing. It
builds a temporary client from the draft and performs a small GET request to a
stable HTTPS endpoint. If implementation needs a deterministic test without
external internet, unit tests should use a local HTTP listener and inspect that
the configured client attempts to route through the proxy.

## Error Handling

Add or reuse safe errors:

- `InvalidProxySettings`
- `ProxyConnectionFailed`
- Existing `NetworkUnavailable` for downstream request failures.

Do not expose raw proxy credentials or full URLs in logs or user-visible error
messages. If debug output is needed, redact username/password and query strings.

## Branding

Rename user-facing product text from `Local Wallet` or generic Flutter wallet
labels to `Safe Wallet`.

Initial scope:

- Unlock screen title.
- README project heading and short description.
- App title where it is visible in Flutter.

Platform bundle identifiers and package names can remain unchanged in this
iteration unless they block app launch or build output.

## Testing Strategy

Rust tests:

- Valid proxy URL accepts `http://127.0.0.1:8080`.
- Valid proxy URL accepts `https://127.0.0.1:8443`.
- Valid proxy URL accepts `socks5://127.0.0.1:9050`.
- Invalid proxy URL rejects missing scheme, unsupported scheme, and missing host.
- Saving privacy settings persists and reloads from SQLite.
- Tor mode with empty URL saves the default Tor SOCKS URL.
- RPC and indexer client constructors consume the same settings.

FFI tests:

- Settings can be saved and read back through JSON commands.
- Invalid proxy settings return a safe error.
- Proxy test command returns a safe success or failure response without exposing
  secrets.

Flutter tests:

- Settings page exposes `Network privacy`.
- Proxy dialog supports Custom and Tor modes.
- Saving proxy settings calls the wallet API with the selected mode and URL.
- Test connection button shows loading state and success/failure text.

Full verification:

```sh
cd apps/flutter_wallet && dart format lib test && flutter analyze && flutter test
cargo test --workspace
```

## Implementation Order

1. Add Rust models, validation, storage, and tests.
2. Add shared HTTP client builder and migrate RPC/indexer constructors.
3. Add FFI commands and Dart API methods.
4. Add Flutter settings UI and widget tests.
5. Rename visible app/project text to Safe Wallet.
6. Run full verification.
