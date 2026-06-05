# Rust Flutter Wallet Rewrite Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the current Python plus Electron wallet with a Rust wallet core and Flutter app for macOS, Android, and iOS, covering multi-wallet keystore security, BTC/EVM/TRON accounts, native coins, ERC20/TRC20 tokens, configurable RPC/indexers, QR transfers, activity history, and verified cutover.

**Architecture:** Rust owns mnemonic handling, master password security, encrypted keystores, SQLite, account derivation, RPC/indexer adapters, transfer preview, signing, broadcast, and safe FFI models. Flutter owns the app shell, wallet workflows, forms, camera/image QR scanning, settings, and platform-specific UI. Python and Electron remain only as local reference until the Rust/Flutter skeleton and replacement tests exist; removal happens at an explicit cutover checkpoint.

**Tech Stack:** Rust 1.94, Cargo workspace, Flutter, Dart, SQLite via `rusqlite`, `serde`, `thiserror`, `zeroize`, `aes-gcm`, `scrypt`, `bip39`, `bip32`, `bitcoin`, `k256`, `tiny-keccak`, `bs58`, `flutter_rust_bridge` unless it blocks platform builds, mocked RPC/indexer clients for stable tests.

---

## Scope Check

The approved spec covers several subsystems: Rust core, Flutter UI, FFI, storage, cryptography, chain adapters, tokens, indexers, QR parsing, biometrics, and cutover. This plan keeps one ordered master plan so the hard rewrite has a single source of truth, but each task must be committed independently and must produce a testable slice.

Current local evidence:

- `rustc --version`: `rustc 1.94.0 (4a4ef493e 2026-03-02)`
- `cargo --version`: `cargo 1.94.0 (85eff7c80 2026-01-15)`
- `flutter --version`: command not found
- Existing dirty file before this rewrite plan: `docs/superpowers/plans/2026-06-03-flutter-rust-wallet-migration.md`

## Target File Structure

- Create `Cargo.toml`: root Rust workspace.
- Create `crates/wallet_core/Cargo.toml`: Rust core crate metadata.
- Create `crates/wallet_core/src/lib.rs`: public module exports.
- Create `crates/wallet_core/src/error.rs`: safe error type.
- Create `crates/wallet_core/src/models.rs`: public wallet, chain, asset, activity, and transfer models.
- Create `crates/wallet_core/src/security.rs`: master password verifier and encrypted secret helpers.
- Create `crates/wallet_core/src/keystore.rs`: encrypted per-wallet mnemonic keystores.
- Create `crates/wallet_core/src/storage.rs`: SQLite schema and persistence.
- Create `crates/wallet_core/src/mnemonic.rs`: BIP39 generation and validation.
- Create `crates/wallet_core/src/accounts.rs`: wallet account derivation interface.
- Create `crates/wallet_core/src/chains/mod.rs`: chain adapter traits and exports.
- Create `crates/wallet_core/src/chains/btc.rs`: BTC validation and transfer adapter.
- Create `crates/wallet_core/src/chains/evm.rs`: EVM validation, ERC20 metadata, and transfer adapter.
- Create `crates/wallet_core/src/chains/tron.rs`: TRON validation, TRC20 metadata, and transfer adapter.
- Create `crates/wallet_core/src/indexers.rs`: indexer provider traits and activity normalization.
- Create `crates/wallet_core/src/qr.rs`: payment URI and QR payload parsing.
- Create `crates/wallet_core/src/service.rs`: high-level wallet service consumed by FFI.
- Create `crates/wallet_ffi/Cargo.toml`: FFI crate metadata.
- Create `crates/wallet_ffi/src/lib.rs`: Flutter-facing command surface.
- Create `crates/wallet_cli/Cargo.toml`: developer CLI metadata.
- Create `crates/wallet_cli/src/main.rs`: deterministic fixture and smoke command entrypoint.
- Create `fixtures/wallet_vectors.json`: deterministic mnemonic, account, token, QR, and activity fixtures.
- Create `apps/flutter_wallet`: Flutter application.
- Create `apps/flutter_wallet/lib/main.dart`: Flutter app entrypoint.
- Create `apps/flutter_wallet/lib/src/wallet_api.dart`: Dart API abstraction.
- Create `apps/flutter_wallet/lib/src/models.dart`: Dart public models.
- Create `apps/flutter_wallet/lib/src/screens/*.dart`: Flutter screens.
- Create `apps/flutter_wallet/test`: Flutter widget tests.
- Modify `README.md`: Rust/Flutter setup and run commands.
- Remove or archive `apps/desktop`, root Node scripts, `wallet_core`, `wallet_api`, Python tests, and `pyproject.toml` only at the cutover task.

---

### Task 1: Toolchain And Baseline

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Confirm Rust and Flutter state**

Run:

```bash
rustc --version
cargo --version
flutter --version
```

Expected: Rust and Cargo print `1.94.0` or newer. Flutter currently fails with `command not found` until installed.

- [ ] **Step 2: Install Flutter**

Run on macOS:

```bash
brew install --cask flutter
flutter doctor
```

Expected: `flutter doctor` finds Flutter. Resolve Android Studio, Xcode, and CocoaPods issues before executing Flutter build tasks.

- [ ] **Step 3: Record rewrite setup commands**

Modify `README.md` so the development setup section contains:

````markdown
## Development Setup

This repository is being rewritten to Rust plus Flutter.

Required tools:

```bash
rustc --version
cargo --version
flutter doctor
```

Run Rust checks:

```bash
cargo test --workspace
```

Run Flutter checks:

```bash
flutter test apps/flutter_wallet
flutter build macos --debug
```

Android and iOS support require platform toolchains:

```bash
flutter build apk --debug
flutter build ios --debug --no-codesign
```
````

- [ ] **Step 4: Commit**

Run:

```bash
git add README.md
git commit -m "docs: document rust flutter toolchain"
```

---

### Task 2: Rust Workspace Scaffold

**Files:**
- Create: `Cargo.toml`
- Create: `crates/wallet_core/Cargo.toml`
- Create: `crates/wallet_core/src/lib.rs`
- Create: `crates/wallet_core/src/error.rs`
- Create: `crates/wallet_core/src/models.rs`
- Create: `crates/wallet_ffi/Cargo.toml`
- Create: `crates/wallet_ffi/src/lib.rs`
- Create: `crates/wallet_cli/Cargo.toml`
- Create: `crates/wallet_cli/src/main.rs`

- [ ] **Step 1: Verify Rust workspace is absent**

Run:

```bash
cargo test --workspace
```

Expected: FAIL with `could not find Cargo.toml`.

- [ ] **Step 2: Create root workspace**

Create `Cargo.toml`:

```toml
[workspace]
members = [
  "crates/wallet_core",
  "crates/wallet_ffi",
  "crates/wallet_cli"
]
resolver = "2"

[workspace.package]
edition = "2021"
license = "UNLICENSED"
version = "0.1.0"

[workspace.dependencies]
aes-gcm = "0.10"
base64 = "0.22"
bip39 = { version = "2", features = ["rand"] }
chrono = { version = "0.4", features = ["serde"] }
hex = "0.4"
rand = "0.8"
rusqlite = { version = "0.32", features = ["bundled"] }
scrypt = "0.11"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sha2 = "0.10"
tempfile = "3"
thiserror = "2"
uuid = { version = "1", features = ["v4", "serde"] }
zeroize = { version = "1", features = ["zeroize_derive"] }
```

- [ ] **Step 3: Create wallet core crate**

Create `crates/wallet_core/Cargo.toml`:

```toml
[package]
name = "wallet_core"
edition.workspace = true
license.workspace = true
version.workspace = true

[dependencies]
aes-gcm.workspace = true
base64.workspace = true
bip39.workspace = true
chrono.workspace = true
hex.workspace = true
rand.workspace = true
rusqlite.workspace = true
scrypt.workspace = true
serde.workspace = true
serde_json.workspace = true
sha2.workspace = true
thiserror.workspace = true
uuid.workspace = true
zeroize.workspace = true

[dev-dependencies]
tempfile.workspace = true
```

Create `crates/wallet_core/src/lib.rs`:

```rust
pub mod error;
pub mod models;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn exposes_version() {
        assert_eq!(crate::VERSION, "0.1.0");
    }
}
```

Create `crates/wallet_core/src/error.rs`:

```rust
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WalletError {
    #[error("Invalid mnemonic")]
    InvalidMnemonic,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Wallet is locked")]
    Locked,
    #[error("Wallet not found")]
    WalletNotFound,
    #[error("Invalid address")]
    InvalidAddress,
    #[error("Invalid token contract")]
    InvalidTokenContract,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Network unavailable")]
    NetworkUnavailable,
    #[error("Storage error")]
    Storage,
    #[error("Crypto error")]
    Crypto,
}

impl WalletError {
    pub fn safe_message(&self) -> &'static str {
        match self {
            WalletError::InvalidPassword => "Unlock failed",
            WalletError::Crypto => "Security operation failed",
            WalletError::Storage => "Local wallet storage failed",
            WalletError::NetworkUnavailable => "Network request failed",
            WalletError::InvalidMnemonic => "Invalid recovery phrase",
            WalletError::Locked => "Wallet is locked",
            WalletError::WalletNotFound => "Wallet not found",
            WalletError::InvalidAddress => "Invalid recipient address",
            WalletError::InvalidTokenContract => "Invalid token contract",
            WalletError::InsufficientFunds => "Insufficient funds",
        }
    }
}
```

Create `crates/wallet_core/src/models.rs`:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChainId {
    Btc,
    Ethereum,
    Bsc,
    Polygon,
    Arbitrum,
    Optimism,
    Tron,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetKind {
    Native,
    Erc20,
    Trc20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityKind {
    NativeTransfer,
    TokenTransfer,
    Approval,
    Swap,
    ContractCall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletSummary {
    pub id: Uuid,
    pub label: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub wallet_id: Uuid,
    pub chain: ChainId,
    pub address: String,
    pub derivation_path: String,
    pub account_index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asset {
    pub id: Uuid,
    pub chain: ChainId,
    pub kind: AssetKind,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub contract_address: Option<String>,
    pub visible: bool,
}
```

- [ ] **Step 4: Create placeholder workspace members**

Because the root workspace lists `crates/wallet_ffi` and `crates/wallet_cli`,
create minimal placeholder crates now so Cargo can load the workspace. Later
tasks replace these placeholders with the real FFI command surface and CLI.

Create `crates/wallet_ffi/Cargo.toml`:

```toml
[package]
name = "wallet_ffi"
edition.workspace = true
license.workspace = true
version.workspace = true

[dependencies]
wallet_core = { path = "../wallet_core" }
```

Create `crates/wallet_ffi/src/lib.rs`:

```rust
pub fn linked_wallet_core_version() -> &'static str {
    wallet_core::VERSION
}

#[cfg(test)]
mod tests {
    #[test]
    fn links_wallet_core() {
        assert_eq!(crate::linked_wallet_core_version(), "0.1.0");
    }
}
```

Create `crates/wallet_cli/Cargo.toml`:

```toml
[package]
name = "wallet_cli"
edition.workspace = true
license.workspace = true
version.workspace = true

[dependencies]
wallet_core = { path = "../wallet_core" }
```

Create `crates/wallet_cli/src/main.rs`:

```rust
fn main() {
    println!("wallet_cli {}", wallet_core::VERSION);
}
```

- [ ] **Step 5: Run scaffold tests**

Run:

```bash
cargo test -p wallet_core
cargo test --workspace
```

Expected: PASS with `exposes_version` and `links_wallet_core`.

- [ ] **Step 6: Commit**

Run:

```bash
git add Cargo.toml Cargo.lock crates/wallet_core crates/wallet_ffi crates/wallet_cli
git commit -m "chore: scaffold rust wallet core"
```

---

### Task 3: SQLite Storage Schema

**Files:**
- Modify: `crates/wallet_core/src/lib.rs`
- Create: `crates/wallet_core/src/storage.rs`

- [ ] **Step 1: Write storage schema tests**

Create `crates/wallet_core/src/storage.rs` with the failing tests first:

```rust
use crate::error::WalletError;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

pub struct WalletDatabase {
    path: PathBuf,
}

impl WalletDatabase {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn initialize(&self) -> Result<(), WalletError> {
        let _ = &self.path;
        Err(WalletError::Storage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_creates_required_tables() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("wallet.sqlite");
        let database = WalletDatabase::new(&db_path);

        database.initialize().unwrap();

        let connection = Connection::open(db_path).unwrap();
        let mut statement = connection
            .prepare("select name from sqlite_master where type = 'table' order by name")
            .unwrap();
        let tables = statement
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(tables.contains(&"app_security".to_string()));
        assert!(tables.contains(&"wallets".to_string()));
        assert!(tables.contains(&"keystore_items".to_string()));
        assert!(tables.contains(&"accounts".to_string()));
        assert!(tables.contains(&"chain_settings".to_string()));
        assert!(tables.contains(&"indexer_settings".to_string()));
        assert!(tables.contains(&"tokens".to_string()));
        assert!(tables.contains(&"asset_balances".to_string()));
        assert!(tables.contains(&"activities".to_string()));
        assert!(tables.contains(&"transactions".to_string()));
        assert!(tables.contains(&"ui_preferences".to_string()));
    }
}
```

Modify `crates/wallet_core/src/lib.rs`:

```rust
pub mod error;
pub mod models;
pub mod storage;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test -p wallet_core storage::tests::initialize_creates_required_tables
```

Expected: FAIL because `initialize` returns `WalletError::Storage`.

- [ ] **Step 3: Implement schema creation**

Replace the `initialize` body in `crates/wallet_core/src/storage.rs` with:

```rust
    pub fn initialize(&self) -> Result<(), WalletError> {
        let connection = Connection::open(&self.path).map_err(|_| WalletError::Storage)?;
        connection
            .execute_batch(
                r#"
                pragma foreign_keys = on;

                create table if not exists app_security (
                    id integer primary key check (id = 1),
                    password_salt text not null,
                    password_verifier text not null,
                    kdf_name text not null,
                    kdf_params_json text not null,
                    biometric_enabled integer not null default 0,
                    security_version integer not null,
                    created_at text not null,
                    updated_at text not null
                );

                create table if not exists wallets (
                    id text primary key,
                    label text not null,
                    keystore_id text not null,
                    hidden integer not null default 0,
                    created_at text not null,
                    updated_at text not null,
                    foreign key (keystore_id) references keystore_items(id)
                );

                create table if not exists keystore_items (
                    id text primary key,
                    ciphertext text not null,
                    nonce text not null,
                    salt text not null,
                    kdf_name text not null,
                    kdf_params_json text not null,
                    cipher_name text not null,
                    version integer not null,
                    created_at text not null,
                    updated_at text not null
                );

                create table if not exists accounts (
                    id text primary key,
                    wallet_id text not null,
                    chain text not null,
                    address text not null,
                    derivation_path text not null,
                    account_index integer not null,
                    created_at text not null,
                    unique(wallet_id, chain, account_index),
                    foreign key (wallet_id) references wallets(id)
                );

                create table if not exists chain_settings (
                    chain text primary key,
                    enabled integer not null,
                    default_rpc_url text not null,
                    user_rpc_url text,
                    explorer_url text,
                    native_symbol text not null,
                    native_decimals integer not null,
                    updated_at text not null
                );

                create table if not exists indexer_settings (
                    id text primary key,
                    chain text not null,
                    provider text not null,
                    endpoint text not null,
                    encrypted_api_key text,
                    enabled integer not null,
                    last_sync_at text,
                    updated_at text not null
                );

                create table if not exists tokens (
                    id text primary key,
                    chain text not null,
                    contract_address text,
                    kind text not null,
                    symbol text not null,
                    name text not null,
                    decimals integer not null,
                    source text not null,
                    visible integer not null,
                    updated_at text not null,
                    unique(chain, contract_address)
                );

                create table if not exists asset_balances (
                    id text primary key,
                    wallet_id text not null,
                    account_id text not null,
                    asset_id text not null,
                    balance text not null,
                    block_height integer,
                    source text not null,
                    refreshed_at text not null,
                    unique(wallet_id, account_id, asset_id),
                    foreign key (wallet_id) references wallets(id),
                    foreign key (account_id) references accounts(id),
                    foreign key (asset_id) references tokens(id)
                );

                create table if not exists activities (
                    id text primary key,
                    wallet_id text not null,
                    account_id text not null,
                    chain text not null,
                    provider_id text,
                    tx_hash text not null,
                    kind text not null,
                    status text not null,
                    from_address text,
                    to_address text,
                    contract_address text,
                    asset_symbol text,
                    amount text,
                    fee text,
                    block_height integer,
                    happened_at text,
                    decoded_summary text,
                    safe_provider_ref text,
                    unique(chain, tx_hash, kind, account_id),
                    foreign key (wallet_id) references wallets(id),
                    foreign key (account_id) references accounts(id)
                );

                create table if not exists transactions (
                    id text primary key,
                    wallet_id text not null,
                    account_id text not null,
                    chain text not null,
                    asset_id text not null,
                    to_address text not null,
                    amount text not null,
                    fee_estimate text,
                    status text not null,
                    tx_hash text,
                    created_at text not null,
                    updated_at text not null,
                    foreign key (wallet_id) references wallets(id),
                    foreign key (account_id) references accounts(id),
                    foreign key (asset_id) references tokens(id)
                );

                create table if not exists ui_preferences (
                    key text primary key,
                    value_json text not null,
                    updated_at text not null
                );
                "#,
            )
            .map_err(|_| WalletError::Storage)?;
        Ok(())
    }
```

- [ ] **Step 4: Run storage tests**

Run:

```bash
cargo test -p wallet_core storage
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```bash
git add crates/wallet_core/src/lib.rs crates/wallet_core/src/storage.rs
git commit -m "feat: add rust sqlite schema"
```

---

### Task 4: Master Password And Multi-Wallet Keystore

**Files:**
- Modify: `crates/wallet_core/src/lib.rs`
- Create: `crates/wallet_core/src/security.rs`
- Create: `crates/wallet_core/src/mnemonic.rs`
- Create: `crates/wallet_core/src/keystore.rs`
- Modify: `crates/wallet_core/src/storage.rs`

- [ ] **Step 1: Write security tests**

Create `crates/wallet_core/src/security.rs`:

```rust
use crate::error::WalletError;

#[derive(Debug, Clone)]
pub struct MasterPasswordVerifier {
    pub salt_b64: String,
    pub verifier_b64: String,
    pub kdf_name: String,
    pub kdf_params_json: String,
    pub version: u32,
}

pub fn create_master_password_verifier(_password: &str) -> Result<MasterPasswordVerifier, WalletError> {
    Err(WalletError::Crypto)
}

pub fn verify_master_password(_password: &str, _verifier: &MasterPasswordVerifier) -> Result<(), WalletError> {
    Err(WalletError::InvalidPassword)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn master_password_verifier_accepts_original_password() {
        let verifier = create_master_password_verifier("correct horse battery staple").unwrap();
        verify_master_password("correct horse battery staple", &verifier).unwrap();
    }

    #[test]
    fn master_password_verifier_rejects_wrong_password() {
        let verifier = create_master_password_verifier("correct horse battery staple").unwrap();
        let error = verify_master_password("wrong password", &verifier).unwrap_err();
        assert_eq!(error, WalletError::InvalidPassword);
    }
}
```

Modify `crates/wallet_core/src/lib.rs`:

```rust
pub mod error;
pub mod models;
pub mod security;
pub mod storage;
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test -p wallet_core security
```

Expected: FAIL because verifier creation returns `WalletError::Crypto`.

- [ ] **Step 3: Implement password verifier**

Replace `crates/wallet_core/src/security.rs` with:

```rust
use crate::error::WalletError;
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::{rngs::OsRng, RngCore};
use scrypt::{scrypt, Params};
use zeroize::Zeroizing;

const SCRYPT_LOG_N: u8 = 15;
const SCRYPT_R: u32 = 8;
const SCRYPT_P: u32 = 1;

#[derive(Debug, Clone)]
pub struct MasterPasswordVerifier {
    pub salt_b64: String,
    pub verifier_b64: String,
}

pub fn derive_key(password: &str, salt: &[u8]) -> Result<Zeroizing<[u8; 32]>, WalletError> {
    let params = Params::new(SCRYPT_LOG_N, SCRYPT_R, SCRYPT_P, 32).map_err(|_| WalletError::Crypto)?;
    let mut key = Zeroizing::new([0u8; 32]);
    scrypt(password.as_bytes(), salt, &params, key.as_mut()).map_err(|_| WalletError::Crypto)?;
    Ok(key)
}

pub fn create_master_password_verifier(password: &str) -> Result<MasterPasswordVerifier, WalletError> {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    let key = derive_key(password, &salt)?;
    Ok(MasterPasswordVerifier {
        salt_b64: STANDARD.encode(salt),
        verifier_b64: STANDARD.encode(&key[..]),
    })
}

pub fn verify_master_password(password: &str, verifier: &MasterPasswordVerifier) -> Result<(), WalletError> {
    let salt = STANDARD
        .decode(&verifier.salt_b64)
        .map_err(|_| WalletError::InvalidPassword)?;
    let expected = STANDARD
        .decode(&verifier.verifier_b64)
        .map_err(|_| WalletError::InvalidPassword)?;
    let actual = derive_key(password, &salt)?;
    if actual.as_slice() == expected.as_slice() {
        Ok(())
    } else {
        Err(WalletError::InvalidPassword)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn master_password_verifier_accepts_original_password() {
        let verifier = create_master_password_verifier("correct horse battery staple").unwrap();
        verify_master_password("correct horse battery staple", &verifier).unwrap();
    }

    #[test]
    fn master_password_verifier_rejects_wrong_password() {
        let verifier = create_master_password_verifier("correct horse battery staple").unwrap();
        let error = verify_master_password("wrong password", &verifier).unwrap_err();
        assert_eq!(error, WalletError::InvalidPassword);
    }
}
```

- [ ] **Step 4: Add mnemonic and keystore tests**

Create `crates/wallet_core/src/mnemonic.rs`:

```rust
use crate::error::WalletError;

pub fn generate_mnemonic() -> Result<String, WalletError> {
    Err(WalletError::InvalidMnemonic)
}

pub fn validate_mnemonic(_mnemonic: &str) -> Result<(), WalletError> {
    Err(WalletError::InvalidMnemonic)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_mnemonic_is_valid() {
        let mnemonic = generate_mnemonic().unwrap();
        validate_mnemonic(&mnemonic).unwrap();
        assert_eq!(mnemonic.split_whitespace().count(), 12);
    }

    #[test]
    fn known_valid_mnemonic_is_accepted() {
        validate_mnemonic("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about").unwrap();
    }
}
```

Create `crates/wallet_core/src/keystore.rs`:

```rust
use crate::error::WalletError;

#[derive(Debug, Clone)]
pub struct EncryptedKeystore {
    pub ciphertext_b64: String,
    pub nonce_b64: String,
    pub salt_b64: String,
    pub kdf_name: String,
    pub kdf_params_json: String,
    pub cipher_name: String,
    pub version: u32,
}

pub fn encrypt_mnemonic(_mnemonic: &str, _password: &str) -> Result<EncryptedKeystore, WalletError> {
    Err(WalletError::Crypto)
}

pub fn decrypt_mnemonic(_keystore: &EncryptedKeystore, _password: &str) -> Result<String, WalletError> {
    Err(WalletError::InvalidPassword)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    #[test]
    fn encrypted_keystore_round_trips_mnemonic() {
        let keystore = encrypt_mnemonic(MNEMONIC, "master-password").unwrap();
        let decrypted = decrypt_mnemonic(&keystore, "master-password").unwrap();
        assert_eq!(decrypted, MNEMONIC);
    }

    #[test]
    fn encrypted_keystore_does_not_contain_plaintext_mnemonic() {
        let keystore = encrypt_mnemonic(MNEMONIC, "master-password").unwrap();
        assert!(!keystore.ciphertext_b64.contains("abandon"));
    }

    #[test]
    fn wrong_password_cannot_decrypt_keystore() {
        let keystore = encrypt_mnemonic(MNEMONIC, "master-password").unwrap();
        let error = decrypt_mnemonic(&keystore, "wrong-password").unwrap_err();
        assert_eq!(error, WalletError::InvalidPassword);
    }
}
```

Modify `crates/wallet_core/src/lib.rs`:

```rust
pub mod error;
pub mod keystore;
pub mod mnemonic;
pub mod models;
pub mod security;
pub mod storage;
```

- [ ] **Step 5: Run tests to verify mnemonic and keystore fail**

Run:

```bash
cargo test -p wallet_core mnemonic
cargo test -p wallet_core keystore
```

Expected: FAIL because mnemonic generation and keystore encryption are not implemented yet.

- [ ] **Step 6: Implement mnemonic and keystore**

Implement `mnemonic.rs` using `bip39::Mnemonic::generate_in(Language::English, 12)` and `Mnemonic::parse_in_normalized`.

Implement `keystore.rs` using:

- `rand::rngs::OsRng` for salt and nonce.
- `security::derive_key` for a domain-separated AES key.
- `aes_gcm::Aes256Gcm`.
- `Aead::encrypt` and `Aead::decrypt`.
- `base64::STANDARD` for persistence-safe encoding.
- Map decrypt failures to `WalletError::InvalidPassword`.
- Fixed-length checks for decoded salt, nonce, and verifier bytes before use.
- KDF, cipher, and version metadata on persisted crypto structs.
- Exact metadata validation before verifier checks or keystore decrypt.
- AES-GCM additional authenticated data for keystore metadata.
- Redacted `Debug` implementations for verifier and keystore structs.

The public function signatures from Step 4 must not change.

- [ ] **Step 7: Add corrupted-data hardening tests**

Add tests that prove:

- repeated encryption of the same mnemonic and password produces different ciphertext and nonce.
- invalid mnemonic input is rejected before encryption.
- malformed nonce length returns a `WalletError` without panic.
- malformed salt length returns a `WalletError` without panic.
- empty ciphertext returns a `WalletError` without panic.
- malformed verifier length returns `WalletError::InvalidPassword`.
- verifier bytes are domain-separated from the keystore encryption key for the same password and salt.
- unsupported verifier metadata fails safely.
- unsupported or tampered keystore metadata fails safely.
- `Debug` output does not include verifier, salt, nonce, or ciphertext values.

- [ ] **Step 8: Run security tests**

Run:

```bash
cargo test -p wallet_core security
cargo test -p wallet_core mnemonic
cargo test -p wallet_core keystore
```

Expected: PASS.

- [ ] **Step 9: Commit**

Run:

```bash
git add crates/wallet_core/src/lib.rs crates/wallet_core/src/security.rs crates/wallet_core/src/mnemonic.rs crates/wallet_core/src/keystore.rs
git commit -m "feat: add master password and keystore crypto"
```

---

### Task 5: Wallet Service And Multi-Wallet Persistence

**Files:**
- Create: `crates/wallet_core/src/service.rs`
- Modify: `crates/wallet_core/src/storage.rs`
- Modify: `crates/wallet_core/src/lib.rs`
- Modify: `crates/wallet_core/src/models.rs`

- [ ] **Step 1: Add wallet service tests**

Create `crates/wallet_core/src/service.rs`:

```rust
use crate::error::WalletError;
use crate::models::WalletSummary;
use crate::storage::WalletDatabase;
use uuid::Uuid;

pub struct WalletService {
    database: WalletDatabase,
}

impl WalletService {
    pub fn new(database: WalletDatabase) -> Self {
        Self { database }
    }

    pub fn initialize(&self) -> Result<(), WalletError> {
        self.database.initialize()
    }

    pub fn set_master_password(&self, _password: &str) -> Result<(), WalletError> {
        Err(WalletError::Storage)
    }

    pub fn unlock_app(&self, _password: &str) -> Result<(), WalletError> {
        Err(WalletError::InvalidPassword)
    }

    pub fn create_wallet(&self, _label: &str, _mnemonic: &str, _password: &str) -> Result<WalletSummary, WalletError> {
        Err(WalletError::Storage)
    }

    pub fn list_wallets(&self) -> Result<Vec<WalletSummary>, WalletError> {
        Err(WalletError::Storage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    fn service() -> WalletService {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("wallet.sqlite");
        let database = WalletDatabase::new(db_path);
        let service = WalletService::new(database);
        service.initialize().unwrap();
        service
    }

    #[test]
    fn master_password_unlocks_app() {
        let service = service();
        service.set_master_password("master-password").unwrap();
        service.unlock_app("master-password").unwrap();
    }

    #[test]
    fn wrong_master_password_is_rejected() {
        let service = service();
        service.set_master_password("master-password").unwrap();
        let error = service.unlock_app("wrong-password").unwrap_err();
        assert_eq!(error, WalletError::InvalidPassword);
    }

    #[test]
    fn creates_multiple_wallets() {
        let service = service();
        service.set_master_password("master-password").unwrap();
        let first = service.create_wallet("Primary", MNEMONIC, "master-password").unwrap();
        let second = service.create_wallet("Trading", MNEMONIC, "master-password").unwrap();
        assert_ne!(first.id, Uuid::nil());
        assert_ne!(first.id, second.id);
        let wallets = service.list_wallets().unwrap();
        assert_eq!(wallets.len(), 2);
    }
}
```

Modify `crates/wallet_core/src/lib.rs`:

```rust
pub mod error;
pub mod keystore;
pub mod mnemonic;
pub mod models;
pub mod security;
pub mod service;
pub mod storage;
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test -p wallet_core service
```

Expected: FAIL because service methods return storage or password errors.

- [ ] **Step 3: Implement persistence methods**

Add these methods to `WalletDatabase` in `storage.rs`:

```rust
pub fn save_master_password_verifier(&self, verifier: &crate::security::MasterPasswordVerifier) -> Result<(), WalletError>;
pub fn load_master_password_verifier(&self) -> Result<Option<crate::security::MasterPasswordVerifier>, WalletError>;
pub fn save_wallet(&self, wallet: &WalletSummary, keystore: &crate::keystore::EncryptedKeystore) -> Result<(), WalletError>;
pub fn list_wallets(&self) -> Result<Vec<WalletSummary>, WalletError>;
```

The implementation must:

- Store `app_security.id = 1`.
- Store wallet and keystore rows in one transaction.
- Use RFC3339 timestamps from `chrono::Utc::now()`.
- Never store the plaintext mnemonic.

- [ ] **Step 4: Implement wallet service**

Implement `set_master_password`, `unlock_app`, `create_wallet`, and `list_wallets` using:

- `security::create_master_password_verifier`.
- `security::verify_master_password`.
- `mnemonic::validate_mnemonic`.
- `keystore::encrypt_mnemonic`.
- `WalletSummary { id: Uuid::new_v4(), label, created_at: Utc::now() }`.

Keep the public method signatures from Step 1.

- [ ] **Step 5: Add plaintext persistence test**

Append this test to `service.rs`:

```rust
#[test]
fn sqlite_does_not_contain_plaintext_mnemonic() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let database = WalletDatabase::new(&db_path);
    let service = WalletService::new(database);
    service.initialize().unwrap();
    service.set_master_password("master-password").unwrap();
    service.create_wallet("Primary", MNEMONIC, "master-password").unwrap();

    let bytes = std::fs::read(db_path).unwrap();
    let text = String::from_utf8_lossy(&bytes);
    assert!(!text.contains("abandon abandon"));
    assert!(!text.contains("master-password"));
}
```

- [ ] **Step 6: Run service tests**

Run:

```bash
cargo test -p wallet_core service
```

Expected: PASS.

- [ ] **Step 7: Commit**

Run:

```bash
git add crates/wallet_core/src/lib.rs crates/wallet_core/src/models.rs crates/wallet_core/src/service.rs crates/wallet_core/src/storage.rs
git commit -m "feat: add multi-wallet service"
```

---

### Task 6: Accounts, Chains, And Default Network Settings

**Files:**
- Create: `fixtures/wallet_vectors.json`
- Create: `crates/wallet_core/src/accounts.rs`
- Create: `crates/wallet_core/src/chains/mod.rs`
- Create: `crates/wallet_core/src/chains/btc.rs`
- Create: `crates/wallet_core/src/chains/evm.rs`
- Create: `crates/wallet_core/src/chains/tron.rs`
- Modify: `crates/wallet_core/src/lib.rs`
- Modify: `crates/wallet_core/src/models.rs`

- [ ] **Step 1: Create deterministic fixture**

Create `fixtures/wallet_vectors.json`:

```json
{
  "mnemonic": "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
  "accounts": [
    {
      "chain": "btc",
      "path": "m/84'/0'/0'/0/0",
      "address_prefix": "bc1"
    },
    {
      "chain": "ethereum",
      "path": "m/44'/60'/0'/0/0",
      "address_prefix": "0x"
    },
    {
      "chain": "tron",
      "path": "m/44'/195'/0'/0/0",
      "address_prefix": "T"
    }
  ],
  "qr_payloads": [
    "bitcoin:bc1qexample000000000000000000000000000000000?amount=0.001",
    "ethereum:0x0000000000000000000000000000000000000000?value=1000000000000000",
    "TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7"
  ]
}
```

- [ ] **Step 2: Write chain validation tests**

Create `crates/wallet_core/src/chains/mod.rs`:

```rust
pub mod btc;
pub mod evm;
pub mod tron;

use crate::error::WalletError;
use crate::models::ChainId;

pub trait ChainAddressValidator {
    fn chain(&self) -> ChainId;
    fn validate_address(&self, address: &str) -> Result<(), WalletError>;
}
```

Create `crates/wallet_core/src/chains/btc.rs`, `evm.rs`, and `tron.rs` with validators that return `Err(WalletError::InvalidAddress)`.

Add tests in each file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_chain_address_shape() {
        let validator = BtcValidator;
        validator.validate_address("bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu").unwrap();
    }
}
```

Use equivalent test addresses for EVM (`0x0000000000000000000000000000000000000000`) and TRON (`TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7`).

- [ ] **Step 3: Run tests to verify they fail**

Run:

```bash
cargo test -p wallet_core chains
```

Expected: FAIL because validators reject all addresses.

- [ ] **Step 4: Implement validators and default settings**

Implement:

- BTC validation: accept lowercase `bc1` bech32-looking addresses with length between 14 and 90.
- EVM validation: accept `0x` followed by 40 hex chars.
- TRON validation: accept `T` followed by 33 base58 chars for first release validation.

Add a `ChainSettings` model to `models.rs` with:

```rust
pub struct ChainSettings {
    pub chain: ChainId,
    pub enabled: bool,
    pub default_rpc_url: String,
    pub user_rpc_url: Option<String>,
    pub native_symbol: String,
    pub native_decimals: u8,
}
```

Add `default_chain_settings() -> Vec<ChainSettings>` in `chains/mod.rs` with public RPC defaults for all supported chains. Use conservative public defaults and keep them user-overridable.

- [ ] **Step 5: Add account derivation interface tests**

Create `crates/wallet_core/src/accounts.rs`:

```rust
use crate::error::WalletError;
use crate::models::{Account, ChainId};
use uuid::Uuid;

pub fn derive_default_accounts(_wallet_id: Uuid, _mnemonic: &str) -> Result<Vec<Account>, WalletError> {
    Err(WalletError::InvalidMnemonic)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    #[test]
    fn derives_one_account_per_supported_chain() {
        let wallet_id = Uuid::new_v4();
        let accounts = derive_default_accounts(wallet_id, MNEMONIC).unwrap();
        assert!(accounts.iter().any(|account| account.chain == ChainId::Btc));
        assert!(accounts.iter().any(|account| account.chain == ChainId::Ethereum));
        assert!(accounts.iter().any(|account| account.chain == ChainId::Bsc));
        assert!(accounts.iter().any(|account| account.chain == ChainId::Polygon));
        assert!(accounts.iter().any(|account| account.chain == ChainId::Arbitrum));
        assert!(accounts.iter().any(|account| account.chain == ChainId::Optimism));
        assert!(accounts.iter().any(|account| account.chain == ChainId::Tron));
    }
}
```

Modify `lib.rs` to include `pub mod accounts;` and `pub mod chains;`.

- [ ] **Step 6: Implement account derivation**

Add the derivation dependencies during this task:

```bash
cargo add -p wallet_core bip32 bitcoin k256 tiny-keccak bs58
```

Expected: `Cargo.toml` and `Cargo.lock` include the added crates.

Implement derivation with these crate roles:

- `bip32`: derive BIP32 child keys from the BIP39 seed.
- `bitcoin`: encode BTC native SegWit addresses.
- `k256`: derive EVM and TRON secp256k1 public keys.
- `tiny-keccak`: compute EVM address hashes.
- `bs58`: encode TRON addresses.

Required outputs:

- BTC: path `m/84'/0'/0'/0/0`, address starts with `bc1`.
- EVM chains: path `m/44'/60'/0'/0/0`, same address for Ethereum, BSC, Polygon, Arbitrum, and Optimism.
- TRON: path `m/44'/195'/0'/0/0`, address starts with `T`.

If a chain-specific crate cannot satisfy the path and address format, add a small adapter module and test it with the fixture mnemonic before proceeding.

- [ ] **Step 7: Run tests**

Run:

```bash
cargo test -p wallet_core accounts chains
```

Expected: PASS.

- [ ] **Step 8: Commit**

Run:

```bash
git add Cargo.toml Cargo.lock fixtures/wallet_vectors.json crates/wallet_core/src
git commit -m "feat: add chain validation and account derivation"
```

---

### Task 7: Assets, Tokens, Transfers, And Activity Interfaces

**Files:**
- Create: `crates/wallet_core/src/assets.rs`
- Create: `crates/wallet_core/src/transfers.rs`
- Create: `crates/wallet_core/src/indexers.rs`
- Modify: `crates/wallet_core/src/lib.rs`
- Modify: `crates/wallet_core/src/models.rs`

- [ ] **Step 1: Add public models**

Extend `models.rs` with:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenMetadata {
    pub chain: ChainId,
    pub kind: AssetKind,
    pub contract_address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferRequest {
    pub wallet_id: Uuid,
    pub chain: ChainId,
    pub asset_id: Uuid,
    pub to_address: String,
    pub amount: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferPreview {
    pub chain: ChainId,
    pub from_address: String,
    pub to_address: String,
    pub asset_symbol: String,
    pub amount: String,
    pub fee_estimate: String,
    pub rpc_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRecord {
    pub chain: ChainId,
    pub tx_hash: String,
    pub kind: ActivityKind,
    pub status: ActivityStatus,
    pub summary: String,
}
```

- [ ] **Step 2: Write trait tests with fake clients**

Create `assets.rs` with traits and tests:

```rust
use crate::error::WalletError;
use crate::models::{ChainId, TokenMetadata};

pub trait TokenMetadataClient {
    fn fetch_token_metadata(&self, chain: ChainId, contract_address: &str) -> Result<TokenMetadata, WalletError>;
}

pub fn add_custom_token<C: TokenMetadataClient>(
    client: &C,
    chain: ChainId,
    contract_address: &str,
) -> Result<TokenMetadata, WalletError> {
    client.fetch_token_metadata(chain, contract_address)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::AssetKind;

    struct FakeClient;

    impl TokenMetadataClient for FakeClient {
        fn fetch_token_metadata(&self, chain: ChainId, contract_address: &str) -> Result<TokenMetadata, WalletError> {
            Ok(TokenMetadata {
                chain,
                kind: AssetKind::Erc20,
                contract_address: contract_address.to_string(),
                symbol: "USDC".to_string(),
                name: "USD Coin".to_string(),
                decimals: 6,
            })
        }
    }

    #[test]
    fn custom_token_metadata_is_loaded_from_client() {
        let token = add_custom_token(&FakeClient, ChainId::Ethereum, "0x0000000000000000000000000000000000000000").unwrap();
        assert_eq!(token.symbol, "USDC");
        assert_eq!(token.decimals, 6);
    }
}
```

Create `transfers.rs` and `indexers.rs` with equivalent trait-first tests for `TransferPreviewClient` and `ActivityIndexer`.

- [ ] **Step 3: Run tests**

Run:

```bash
cargo test -p wallet_core assets transfers indexers
```

Expected: PASS for trait-level fake client behavior.

- [ ] **Step 4: Wire modules**

Modify `lib.rs`:

```rust
pub mod accounts;
pub mod assets;
pub mod chains;
pub mod error;
pub mod indexers;
pub mod keystore;
pub mod mnemonic;
pub mod models;
pub mod security;
pub mod service;
pub mod storage;
pub mod transfers;
```

- [ ] **Step 5: Commit**

Run:

```bash
git add crates/wallet_core/src/lib.rs crates/wallet_core/src/models.rs crates/wallet_core/src/assets.rs crates/wallet_core/src/transfers.rs crates/wallet_core/src/indexers.rs
git commit -m "feat: add asset transfer and activity interfaces"
```

---

### Task 8: QR Payment Parsing

**Files:**
- Create: `crates/wallet_core/src/qr.rs`
- Modify: `crates/wallet_core/src/lib.rs`
- Modify: `crates/wallet_core/src/models.rs`

- [ ] **Step 1: Write QR parsing tests**

Create `crates/wallet_core/src/qr.rs`:

```rust
use crate::error::WalletError;
use crate::models::ChainId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedPayment {
    pub chain: Option<ChainId>,
    pub address: String,
    pub amount: Option<String>,
    pub note: Option<String>,
}

pub fn parse_payment_uri(_payload: &str) -> Result<ParsedPayment, WalletError> {
    Err(WalletError::InvalidAddress)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bare_evm_address() {
        let parsed = parse_payment_uri("0x0000000000000000000000000000000000000000").unwrap();
        assert_eq!(parsed.chain, None);
        assert_eq!(parsed.address, "0x0000000000000000000000000000000000000000");
    }

    #[test]
    fn parses_bitcoin_uri_with_amount() {
        let parsed = parse_payment_uri("bitcoin:bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu?amount=0.001").unwrap();
        assert_eq!(parsed.chain, Some(ChainId::Btc));
        assert_eq!(parsed.amount.as_deref(), Some("0.001"));
    }

    #[test]
    fn parses_tron_address() {
        let parsed = parse_payment_uri("TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7").unwrap();
        assert_eq!(parsed.chain, Some(ChainId::Tron));
    }
}
```

Modify `lib.rs` with `pub mod qr;`.

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test -p wallet_core qr
```

Expected: FAIL because parser rejects all payloads.

- [ ] **Step 3: Implement parser**

Implement:

- Bare EVM address: `0x` plus 40 hex chars.
- Bare TRON address: `T` plus 33 base58 chars.
- Bare BTC address: starts with `bc1`.
- `bitcoin:` URI: parse address before `?`, parse `amount` and `message` query keys.
- `ethereum:` URI: parse address before `?`, parse `value` and `message` query keys.

The parser must not sign or broadcast. It only returns `ParsedPayment`.

- [ ] **Step 4: Run tests**

Run:

```bash
cargo test -p wallet_core qr
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```bash
git add crates/wallet_core/src/lib.rs crates/wallet_core/src/models.rs crates/wallet_core/src/qr.rs
git commit -m "feat: add payment qr parsing"
```

---

### Task 9: FFI Command Surface

**Files:**
- Modify: `crates/wallet_ffi/Cargo.toml`
- Replace: `crates/wallet_ffi/src/lib.rs`
- Modify: `Cargo.toml`

- [ ] **Step 1: Replace placeholder FFI crate**

Create `crates/wallet_ffi/Cargo.toml`:

```toml
[package]
name = "wallet_ffi"
edition.workspace = true
license.workspace = true
version.workspace = true

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
serde.workspace = true
serde_json.workspace = true
wallet_core = { path = "../wallet_core" }
```

Create `crates/wallet_ffi/src/lib.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[derive(Debug, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum WalletCommand {
    AppStatus,
}

#[derive(Debug, Serialize)]
pub struct WalletResponse {
    pub ok: bool,
    pub body_json: String,
    pub error: Option<String>,
}

pub fn handle_command_json(command_json: &str) -> WalletResponse {
    let parsed = serde_json::from_str::<WalletCommand>(command_json);
    match parsed {
        Ok(WalletCommand::AppStatus) => WalletResponse {
            ok: true,
            body_json: r#"{"locked":true}"#.to_string(),
            error: None,
        },
        Err(_) => WalletResponse {
            ok: false,
            body_json: "{}".to_string(),
            error: Some("Invalid command".to_string()),
        },
    }
}

#[no_mangle]
pub extern "C" fn wallet_command_json_c(command_json: *const c_char) -> *mut c_char {
    let command = unsafe { CStr::from_ptr(command_json) }.to_string_lossy().to_string();
    let response = handle_command_json(&command);
    let json = serde_json::to_string(&response).unwrap_or_else(|_| {
        r#"{"ok":false,"body_json":"{}","error":"Serialization failed"}"#.to_string()
    });
    CString::new(json).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn wallet_string_free(value: *mut c_char) {
    if value.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_status_returns_no_private_material() {
        let response = handle_command_json(r#"{"command":"app_status"}"#);
        assert!(response.ok);
        assert!(!response.body_json.contains("private"));
        assert!(!response.body_json.contains("mnemonic"));
        assert!(!response.body_json.contains("seed"));
    }
}
```

- [ ] **Step 2: Run FFI tests**

Run:

```bash
cargo test -p wallet_ffi
```

Expected: PASS.

- [ ] **Step 3: Expand commands**

Add typed command variants for:

```rust
SetMasterPassword { db_path: String, password: String }
UnlockApp { db_path: String, password: String }
LockApp { db_path: String }
ListWallets { db_path: String }
CreateWallet { db_path: String, label: String, mnemonic: String, password: String }
ImportWallet { db_path: String, label: String, mnemonic: String, password: String }
RevealMnemonic { db_path: String, wallet_id: String, password: String }
ListAssets { db_path: String, wallet_id: String }
AddCustomToken { db_path: String, chain: String, contract_address: String }
ParsePaymentUri { payload: String }
PreviewTransfer { db_path: String, request_json: String }
SendTransfer { db_path: String, request_json: String }
ListActivity { db_path: String, wallet_id: String }
UpdateChainRpc { db_path: String, chain: String, rpc_url: String }
UpdateIndexerSettings { db_path: String, chain: String, endpoint: String, api_key: Option<String> }
```

Each command must call `wallet_core::service::WalletService` or the appropriate parser/client interface and must return `WalletResponse` with user-safe errors from `WalletError::safe_message()`.

- [ ] **Step 4: Run workspace tests**

Run:

```bash
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```bash
git add Cargo.toml Cargo.lock crates/wallet_ffi
git commit -m "feat: add rust ffi command surface"
```

---

### Task 10: Flutter App Shell

**Files:**
- Create: `apps/flutter_wallet`
- Create: `apps/flutter_wallet/lib/main.dart`
- Create: `apps/flutter_wallet/lib/src/models.dart`
- Create: `apps/flutter_wallet/lib/src/wallet_api.dart`
- Create: `apps/flutter_wallet/lib/src/screens/unlock_screen.dart`
- Create: `apps/flutter_wallet/lib/src/screens/wallets_screen.dart`
- Create: `apps/flutter_wallet/lib/src/screens/assets_screen.dart`
- Create: `apps/flutter_wallet/lib/src/screens/transfer_screen.dart`
- Create: `apps/flutter_wallet/lib/src/screens/activity_screen.dart`
- Create: `apps/flutter_wallet/lib/src/screens/settings_screen.dart`
- Create: `apps/flutter_wallet/test/app_test.dart`

- [ ] **Step 1: Scaffold Flutter app**

Run:

```bash
flutter create apps/flutter_wallet --platforms=macos,android,ios
```

Expected: Flutter creates the project.

- [ ] **Step 2: Replace default app with wallet workbench shell**

Create `apps/flutter_wallet/lib/src/wallet_api.dart`:

```dart
abstract class WalletApi {
  Future<AppStatus> appStatus();
  Future<void> unlockApp(String password);
  Future<List<WalletSummary>> listWallets();
}
```

Create `apps/flutter_wallet/lib/src/models.dart`:

```dart
class AppStatus {
  const AppStatus({required this.locked});
  final bool locked;
}

class WalletSummary {
  const WalletSummary({required this.id, required this.label});
  final String id;
  final String label;
}
```

Create screen files with simple Material widgets:

- `UnlockScreen` with password field and unlock button.
- `WalletsScreen` with wallet list and create/import actions.
- `AssetsScreen` with asset list container.
- `TransferScreen` with recipient, amount, QR scan button, and preview button.
- `ActivityScreen` with activity list container.
- `SettingsScreen` with RPC, indexer, biometrics, and lock sections.

Create `main.dart` with a `MaterialApp` and bottom navigation between wallet, assets, transfer, activity, and settings once unlocked.

- [ ] **Step 3: Write widget test**

Create `apps/flutter_wallet/test/app_test.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_wallet/main.dart';

void main() {
  testWidgets('wallet app starts on unlock screen', (tester) async {
    await tester.pumpWidget(const WalletApp());
    expect(find.byType(TextField), findsOneWidget);
    expect(find.text('Unlock'), findsOneWidget);
  });
}
```

- [ ] **Step 4: Run Flutter tests**

Run:

```bash
flutter test apps/flutter_wallet
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```bash
git add apps/flutter_wallet
git commit -m "feat: scaffold flutter wallet app"
```

---

### Task 11: Dart Native API And QR UI

**Files:**
- Create: `apps/flutter_wallet/lib/src/native_wallet_api.dart`
- Modify: `apps/flutter_wallet/lib/src/wallet_api.dart`
- Modify: `apps/flutter_wallet/lib/src/screens/transfer_screen.dart`
- Create: `apps/flutter_wallet/test/qr_transfer_test.dart`

- [ ] **Step 1: Add native API adapter**

Create `native_wallet_api.dart` with a `DynamicLibrary` backed adapter that calls `wallet_command_json_c` and frees strings with `wallet_string_free`.

The adapter must:

- JSON encode command payloads.
- Decode `WalletResponse`.
- Throw `StateError` only with safe `error` strings.
- Never expose `mnemonic`, `seed`, or `private_key` fields except the explicit mnemonic reveal API that is added in the wallet-management task.

- [ ] **Step 2: Add QR parser API**

Extend `WalletApi`:

```dart
Future<ParsedPayment> parsePaymentUri(String payload);
```

Add `ParsedPayment` to `models.dart`:

```dart
class ParsedPayment {
  const ParsedPayment({
    required this.address,
    this.chain,
    this.amount,
    this.note,
  });

  final String address;
  final String? chain;
  final String? amount;
  final String? note;
}
```

- [ ] **Step 3: Add QR transfer widget test**

Create `qr_transfer_test.dart` with a fake `WalletApi` that returns a parsed TRON address and assert the transfer screen prefills the recipient field.

- [ ] **Step 4: Run tests**

Run:

```bash
flutter test apps/flutter_wallet
cargo test -p wallet_ffi
```

Expected: both pass.

- [ ] **Step 5: Commit**

Run:

```bash
git add apps/flutter_wallet crates/wallet_ffi
git commit -m "feat: connect flutter to rust commands"
```

---

### Task 12: Platform Builds And Cutover Cleanup

**Files:**
- Modify: `README.md`
- Modify: `.gitignore`
- Delete or archive only after verification: `apps/desktop`, `package.json`, `wallet_api`, `wallet_core`, `tests`, `pyproject.toml`, `local_wallet.egg-info`

- [ ] **Step 1: Run full Rust verification**

Run:

```bash
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 2: Run Flutter verification**

Run:

```bash
flutter test apps/flutter_wallet
flutter build macos --debug
flutter build apk --debug
flutter build ios --debug --no-codesign
```

Expected: all pass on a machine with macOS, Android, and iOS toolchains installed.

- [ ] **Step 3: Confirm old runtime replacement coverage**

Before deleting old runtime files, confirm these commands pass:

```bash
cargo test --workspace
flutter test apps/flutter_wallet
flutter build macos --debug
```

Expected: PASS. The Rust/Flutter app must cover master password setup, multi-wallet creation/import, address display, asset list, token addition, transfer preview, QR parsing, and settings shell before deletion.

- [ ] **Step 4: Remove old runtime files**

Run:

```bash
git rm -r apps/desktop wallet_api wallet_core tests local_wallet.egg-info
git rm package.json pyproject.toml
```

Update `README.md` so it no longer documents Python or Electron commands.

- [ ] **Step 5: Run final verification**

Run:

```bash
cargo test --workspace
flutter test apps/flutter_wallet
flutter build macos --debug
```

Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```bash
git add README.md .gitignore Cargo.toml Cargo.lock crates apps fixtures
git commit -m "chore: cut over to rust flutter wallet"
```

---

## Self-Review

- Spec coverage: The plan covers Rust workspace, Flutter app, FFI, SQLite, master password, multi-wallet keystore, BTC/EVM/TRON account boundaries, native/token asset interfaces, indexer activity interfaces, QR parsing, public RPC and indexer configuration, biometrics metadata, platform builds, and old runtime removal checkpoints.
- Depth check: Production signing and provider-specific indexer implementations are represented by adapter tasks and must be implemented behind tested traits before final cutover. Stable tests must use mocks and fixtures.
- Empty-section scan: This plan contains no unresolved conflict markers, no empty sections, and no unspecified file paths.
- Type consistency: Public model names are shared across Rust core, FFI, and Dart API tasks. Later tasks must preserve the command names listed in the approved design.
