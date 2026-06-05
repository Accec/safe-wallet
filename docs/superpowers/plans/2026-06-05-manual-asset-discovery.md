# Manual Asset Discovery Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build explicit, user-triggered asset discovery for EVM ERC20 and TRON TRC20 assets.

**Architecture:** Add a core discovery provider, persist discovered token metadata and balances in SQLite, expose a `discover_assets` FFI command, and add a manual Flutter action. No app lifecycle event invokes discovery.

**Tech Stack:** Rust, rusqlite, reqwest, serde_json, Flutter/Dart, FFI JSON commands.

---

### Task 1: Core Discovery Model And Storage

**Files:**
- Modify: `crates/wallet_core/src/models.rs`
- Modify: `crates/wallet_core/src/storage.rs`

- [ ] Add `DiscoveredAsset` to the core model with chain, kind, contract address, symbol, name, decimals, and balance.
- [ ] Add `save_discovered_token` to SQLite storage using `source = 'auto_discovered'`.
- [ ] Add tests proving discovered tokens are visible, balances are saved, and hidden tokens remain hidden.

### Task 2: Discovery Provider

**Files:**
- Create: `crates/wallet_core/src/discovery.rs`
- Modify: `crates/wallet_core/src/lib.rs`

- [ ] Add `AssetDiscoveryProvider`.
- [ ] Add tolerant Etherscan/BscScan and TronScan parsers.
- [ ] Add `HttpAssetDiscoveryProvider` using existing proxy-aware HTTP client.
- [ ] Add parser tests for EVM holdings, EVM token transfers, and TRON account token payloads.

### Task 3: Service And FFI

**Files:**
- Modify: `crates/wallet_core/src/service.rs`
- Modify: `crates/wallet_ffi/src/lib.rs`

- [ ] Add `WalletService::discover_assets` and `discover_assets_with`.
- [ ] Add a `discover_assets` FFI command with optional chain.
- [ ] Add service tests with a fake provider.
- [ ] Add FFI command test.

### Task 4: Flutter Manual Action

**Files:**
- Modify: `apps/flutter_wallet/lib/src/wallet_api.dart`
- Modify: `apps/flutter_wallet/lib/src/native_wallet_api.dart`
- Modify: `apps/flutter_wallet/lib/main.dart`
- Modify: `apps/flutter_wallet/lib/src/screens/assets_screen.dart`
- Modify: `apps/flutter_wallet/test/wallet_workflow_test.dart`
- Modify: `apps/flutter_wallet/test/native_wallet_api_test.dart`

- [ ] Add `discoverAssets` to the Wallet API.
- [ ] Add a visible "Discover assets" icon action with a spinner.
- [ ] Wire the action through app state and reload wallet details after completion.
- [ ] Add Flutter tests proving discovery is only called from the button.

### Task 5: Verification

**Files:**
- No new files.

- [ ] Run `cargo test --workspace`.
- [ ] Run `flutter analyze`.
- [ ] Run `flutter test`.
- [ ] Run `flutter build macos --debug`.
