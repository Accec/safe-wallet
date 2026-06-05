# Safe Wallet

🔐 A local-first, non-custodial crypto wallet built with a Rust core and a Flutter UI for macOS, Android, and iOS.

Safe Wallet keeps wallet data on the user's device, encrypts secret material before it reaches SQLite, and routes all wallet workflows through a native Rust engine. The Flutter app owns the user experience, while Rust owns mnemonic handling, keystore encryption, address derivation, balance refreshes, transaction previews, signing, broadcast boundaries, indexer integration, and local storage.

> ⚠️ This project is an early-stage wallet implementation. It has not been independently audited. Review the code, run the tests, and use test funds until the security model and chain integrations are fully verified.

## ✨ Highlights

- 🔐 **Local custody**: mnemonic and private key material stay on-device and are encrypted into a local SQLite-backed keystore.
- 🧠 **Rust wallet core**: crypto, storage, chain adapters, transaction logic, QR parsing, RPC, indexer, and FFI boundaries live in Rust.
- 📱 **Flutter app shell**: cross-platform UI for macOS, Android, and iOS with wallet, assets, transfer, multisig, activity, and settings screens.
- 🌐 **Multi-chain support**: Bitcoin, Ethereum, BNB Smart Chain, Polygon, Arbitrum One, OP Mainnet, and TRON.
- 💼 **Flexible wallet import**: create BIP39 wallets, import recovery phrases, import private keys, and import/export encrypted keystore JSON.
- 💰 **Asset management**: native asset balances, ERC20/TRC20 custom tokens, manual token discovery, per-chain refreshes, and removable custom tokens.
- 📤 **Transfer flow**: recipient and amount entry, QR scan/import, payment URI parsing, transfer preview, local signing, broadcast, and local transfer records.
- 🧾 **Activity sync**: pull indexed activity from configured indexer endpoints and store transaction history locally.
- 🛡️ **Privacy controls**: master password unlock, optional biometric unlock, duress password decoy flow, and HTTP/HTTPS/SOCKS/Tor proxy settings for network calls.
- 👥 **Multisig workspace**: import existing EVM Safe or TRON permission accounts, create proposals, collect signatures, and track threshold progress.
- ⚙️ **Configurable networks**: edit RPC URLs, chain IDs, explorer URLs, native symbols, and indexer endpoints from the app.

## 🧭 Supported Networks

| Network | Native Asset | Notes |
| --- | --- | --- |
| Bitcoin | BTC | Native account and balance support |
| Ethereum | ETH | EVM RPC, ERC20 tokens, Safe-style multisig metadata |
| BNB Smart Chain | BNB | EVM RPC and ERC20-style token support |
| Polygon | POL | EVM RPC and ERC20-style token support |
| Arbitrum One | ETH | EVM RPC and ERC20-style token support |
| OP Mainnet | ETH | EVM RPC and ERC20-style token support |
| TRON | TRX | TRX, TRC20 tokens, and TRON permission multisig metadata |

## 🏗️ Architecture

```text
.
├── apps/flutter_wallet     # Flutter app for macOS, Android, and iOS
├── crates/wallet_core      # Rust wallet engine, storage, crypto, chains, RPC, indexers
├── crates/wallet_ffi       # Native JSON command bridge for Flutter FFI
├── crates/wallet_cli       # Small CLI entry point for core version checks
├── fixtures                # Test vectors
└── docs/superpowers        # Design notes and implementation plans
```

### Core Boundaries

- **Rust owns sensitive wallet operations**: mnemonic validation, keystore encryption/decryption, private-key import, address derivation, transaction preview, signing, broadcast, balance refresh, asset discovery, and storage.
- **Flutter owns UX state**: screens, forms, navigation, QR actions, settings dialogs, wallet selection, and platform-specific UI.
- **SQLite stores local state**: encrypted keystore rows, public account metadata, chain settings, assets, balances, transfer records, activity records, multisig accounts, proposals, and signatures.
- **FFI uses JSON commands**: Flutter sends structured command payloads to `wallet_ffi`, and Rust returns structured JSON responses.

## 🔒 Security Model

Safe Wallet is designed around a local security boundary:

- No plaintext mnemonic, seed, or private key is stored on disk.
- Keystore secrets are encrypted with **AES-256-GCM**.
- Encryption keys are derived with **scrypt** using per-record salts.
- Master password verifiers and keystore metadata are stored separately from ciphertext.
- Sensitive debug output is redacted in Rust types.
- QR/payment parsing and address validation happen before transfer preview.
- Transfer previews show the chain, source address, recipient, asset, amount, fee estimate, and RPC endpoint before signing.
- Network requests can be routed through custom proxies or the default Tor SOCKS proxy URL.
- A duress password can replace wallet data with a decoy wallet after unlock.

## 🛠️ Requirements

- Rust toolchain: `rustc`, `cargo`, and `rustup`
- Flutter SDK compatible with Dart `^3.12.1`
- macOS builds: Xcode
- Android builds: Android Studio, Android SDK/NDK, `cargo-ndk`
- iOS builds: Xcode, CocoaPods, and installed iOS platforms/runtimes

Check your local toolchain:

```bash
rustc --version
cargo --version
flutter doctor
```

## 🚀 Getting Started

After cloning the repository, install Flutter dependencies from the Flutter app directory:

```bash
cd wallet
cd apps/flutter_wallet
flutter pub get
```

Run Rust tests from the repository root:

```bash
cargo test --workspace
```

Run Flutter checks:

```bash
cd apps/flutter_wallet
flutter test
flutter analyze
```

Run the macOS debug app:

```bash
cd apps/flutter_wallet
tool/run_macos_debug.sh
```

You can also run Flutter directly on macOS:

```bash
cd apps/flutter_wallet
flutter run -d macos
```

## 📦 Native Builds

The Flutter app talks to Rust through the `wallet_ffi` native library.

Build Rust FFI artifacts for the active platform:

```bash
cd apps/flutter_wallet
tool/build_wallet_ffi.sh
```

Android native builds require `cargo-ndk` and Android Rust targets:

```bash
cargo install cargo-ndk
rustup target add armv7-linux-androideabi aarch64-linux-android x86_64-linux-android

cd apps/flutter_wallet
tool/build_wallet_ffi.sh android
flutter build apk --debug
```

iOS native builds require iOS Rust targets:

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios

cd apps/flutter_wallet
flutter build ios --debug --no-codesign
```

## ✅ Testing

The project includes both Rust and Flutter coverage:

- Rust unit tests for mnemonic generation, keystore encryption, password verification, storage, address validation, RPC parsing, QR parsing, token discovery, transfer previews, transaction flows, proxy settings, activity sync, and multisig persistence.
- FFI tests for JSON command serialization and native command responses.
- Flutter tests for wallet workflows, QR transfer flows, native API mapping, screen behavior, settings, and app navigation.

Recommended full check:

```bash
cargo test --workspace

cd apps/flutter_wallet
flutter test
flutter analyze
```

## 🧩 Current Scope

Safe Wallet currently focuses on a local, self-custodial wallet experience with configurable public RPC/indexer integrations. The multisig area stores account metadata, proposals, signatures, and threshold readiness, but does not claim full on-chain Safe execution or TRON permission execution in this first cut.

Production hardening should include independent security review, more chain-specific integration testing, signed release packaging, failure-mode testing with real RPC/indexer providers, and clear end-user backup guidance.

## 📄 License

The Rust workspace is currently marked `UNLICENSED`, and no repository license file is present yet. Add a `LICENSE` file before public distribution if you want to grant reuse rights.
