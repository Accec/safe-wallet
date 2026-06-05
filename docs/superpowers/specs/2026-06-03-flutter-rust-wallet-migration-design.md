# Rust Flutter Wallet Rewrite Design

## Scope

Rewrite the current local wallet as a Rust plus Flutter application. This is a
hard rewrite, not a staged runtime migration. The final runtime stack is Rust
for wallet core behavior and Flutter for macOS, Android, and iOS UI.

The existing Python and Electron code may be used as implementation reference,
but it is not retained as a runtime once the new Rust and Flutter app is in
place. The first release starts with a new Rust-owned SQLite database. It does
not migrate existing Python or Electron wallet databases.

The product remains a local, non-custodial wallet. Rust owns all sensitive
wallet operations. Flutter owns navigation, forms, display state, scanning, and
platform UI.

## Approved First Release Scope

The first rewrite release supports:

- Platforms: macOS, Android, and iOS.
- Wallet model: multiple wallets, with one independent mnemonic per wallet.
- Unlock model: one application-level master password for all local wallets.
- Optional biometrics: Touch ID, Face ID, and Android Biometric as local unlock
  convenience only.
- Chains: BTC, Ethereum, BSC, Polygon, Arbitrum, Optimism, and TRON.
- Assets: native coins, ERC20 tokens, and TRC20 tokens.
- Token discovery: automatic detection through configured indexers plus manual
  token addition by contract address.
- Activity history: complete indexed address activity where provider data
  exists, including transfers, approvals, swaps, contract calls, and failed
  transactions.
- Network configuration: built-in public RPC defaults, user-overridable RPC
  endpoints, and configurable indexer endpoints or API keys.
- Transfers: manual entry and QR-based transfer input.
- QR support: camera scanning and image import for bare addresses,
  `bitcoin:` URIs, EVM addresses or `ethereum:` URIs, and TRON addresses; amount
  and note fields are prefilled when present.
- Backup: viewing or exporting a wallet mnemonic after a second master-password
  verification.

The first release does not support:

- Migration of old Python or Electron databases.
- Private key export.
- WalletConnect.
- DApp connection.
- Built-in DApp browser.
- Hardware wallets.
- Cloud backup.
- Custodial accounts.
- Exchange integration.

## Architecture

The repository is rewritten around these top-level units:

- `apps/flutter_wallet`: Flutter application, UI state, platform integration,
  camera and image QR scanning, and Flutter tests.
- `crates/wallet_core`: Rust wallet domain, keystore, storage, derivation,
  chain adapters, RPC clients, indexer clients, token logic, transaction
  preview, signing, broadcast, and tests.
- `crates/wallet_ffi`: Rust-to-Dart boundary exposed to Flutter. The preferred
  implementation is `flutter_rust_bridge` if it works cleanly for macOS,
  Android, and iOS in this project. If it blocks platform builds, the fallback
  is a small JSON-over-C-ABI boundary.
- `crates/wallet_cli`: developer CLI for deterministic fixture generation,
  smoke checks, and local debugging.
- `fixtures`: deterministic cross-language fixtures for mnemonics, addresses,
  keystore invariants, token metadata, transaction preview shapes, QR payloads,
  and safe error responses.

Rust exposes a small explicit command API to Flutter:

- `app_status`
- `set_master_password`
- `unlock_app`
- `lock_app`
- `enable_biometric_unlock`
- `disable_biometric_unlock`
- `list_wallets`
- `create_wallet`
- `import_wallet`
- `rename_wallet`
- `delete_wallet`
- `verify_master_password`
- `reveal_mnemonic`
- `list_accounts`
- `list_assets`
- `refresh_assets`
- `add_custom_token`
- `hide_asset`
- `show_asset`
- `list_activity`
- `sync_activity`
- `parse_payment_uri`
- `preview_transfer`
- `send_transfer`
- `list_chain_settings`
- `update_chain_rpc`
- `update_indexer_settings`

Private key material, seed bytes, plaintext keystore payloads, raw decrypted
mnemonics outside explicit reveal flows, passwords, and raw signed transactions
are never returned to Flutter by default.

## Data Model

Rust owns a new SQLite schema. Logical tables include:

- `app_security`: master password verifier metadata, KDF parameters, biometric
  unlock metadata, and security version.
- `wallets`: wallet id, label, creation time, current visibility state, and
  encrypted keystore reference.
- `keystore_items`: encrypted mnemonic payload, nonce, salt, KDF parameters,
  cipher name, keystore version, and timestamps.
- `accounts`: wallet id, chain, network, account index, derivation path, and
  public address.
- `chain_settings`: chain id, enabled flag, public default RPC URL, user RPC
  override, explorer URL, and native asset metadata.
- `indexer_settings`: provider type, endpoint, API key reference or encrypted
  secret, enabled flag, and last sync metadata.
- `tokens`: chain id, contract address, token type, symbol, name, decimals,
  source, visibility, and metadata timestamps.
- `asset_balances`: wallet id, account id, asset id, balance, block height,
  source, and refresh timestamp.
- `activities`: wallet id, account id, chain, provider id, transaction hash,
  activity type, status, from address, to address, contract address, asset,
  amount, fee, block height, timestamp, decoded summary, and raw provider
  reference where safe.
- `transactions`: locally initiated transfers, preview data, broadcast result,
  status, transaction hash, and timestamps.
- `ui_preferences`: non-sensitive UI settings.

Plaintext mnemonics, seed bytes, private keys, and master passwords must never
be written to SQLite.

## Security Design

Each wallet has an independently encrypted mnemonic keystore. The application
uses one master password to unlock all local wallets. The master password is
the root local credential; biometrics are optional convenience unlocks backed by
the platform secure store and must be explicitly enabled by the user.

Security requirements:

- Store no plaintext mnemonic, seed, private key, or master password on disk.
- Keep signing inside Rust.
- Do not expose private keys to Flutter, Dart state, logs, or FFI responses.
- Do not expose decrypted keystore payloads over FFI.
- Require explicit user confirmation before signing or broadcasting.
- Show wallet, chain, RPC endpoint, source address, recipient, asset, amount,
  fee estimate, and total spend before signing.
- Require a second master-password verification before revealing a mnemonic.
- Return generic unlock errors for wrong passwords.
- Avoid logging mnemonics, seed bytes, private keys, passwords, signed payloads,
  and authorization material.
- Clear temporary mnemonic, password, QR, address, and amount form state after
  setup, unlock, reveal, or transfer flows complete.
- Test that the SQLite database does not contain plaintext secrets.
- Test that Flutter-visible models and FFI responses exclude private material.

## Chain And Asset Support

BTC support:

- BIP84 native SegWit account derivation.
- Address validation.
- Native BTC balance lookup through configured RPC or service adapter.
- Transfer preview, fee estimate, transaction construction, signing, and
  broadcast.

EVM support:

- Ethereum-style account derivation shared by Ethereum, BSC, Polygon, Arbitrum,
  and Optimism.
- EVM address validation.
- Native coin balance lookup.
- ERC20 metadata lookup by contract address.
- ERC20 balance lookup and transfer.
- Gas estimation, transaction preview, signing, and broadcast.
- Indexed activity ingestion for transfers, approvals, swaps, contract calls,
  and failed transactions where provider support exists.

TRON support:

- BIP44 TRON derivation.
- TRON address validation.
- TRX balance lookup.
- TRC20 metadata lookup by contract address.
- TRC20 balance lookup and transfer.
- Transaction preview, signing, and broadcast.
- Indexed activity ingestion for transfers, approvals, swaps, contract calls,
  and failed transactions where provider support exists.

Automatic token discovery uses configured indexers. Manual token addition uses
RPC reads for token metadata and then persists the token locally. If indexer
configuration is missing or unavailable, the app still supports wallet
creation, import, address display, native coin flows, manually added tokens,
and transfers, but automatic token discovery and full activity history may be
incomplete.

## Network Configuration

Each supported chain ships with a public default RPC URL so the wallet can work
out of the box. Users can override RPC URLs per chain in settings.

Indexer configuration is separate from RPC configuration. Indexers are used for
token discovery and complete address activity. The app supports configurable
indexer endpoints and API keys for EVM chains and TRON. Indexer failures must
degrade gracefully and must not block local wallet access or signing.

Public RPC and public indexer defaults are treated as convenience defaults, not
privacy guarantees. The UI should make configured endpoints visible before
signing and in network settings.

## Flutter UI

Flutter first screen is the app itself, not a landing page. The interface is a
quiet wallet workbench optimized for repeated use.

Core screens:

- Startup and unlock: first-run master password setup, normal password unlock,
  optional biometric unlock, and lock state handling.
- Wallet management: create wallet, import mnemonic, switch wallet, rename
  wallet, delete local wallet, and reveal mnemonic after password verification.
- Assets: wallet and chain selection, native coins, ERC20 and TRC20 balances,
  addresses, refresh status, and visibility controls.
- Tokens: automatic detection results, manual token addition by contract
  address, token metadata confirmation, hide and show asset controls.
- Transfer: wallet, chain, asset, recipient, amount, QR scan, image QR import,
  parsed URI review, and form validation.
- Confirm transfer: chain, RPC endpoint, source address, recipient, asset,
  amount, fee estimate, total spend, and final signing confirmation.
- Activity: complete indexed address activity and local transfer records,
  grouped by wallet, chain, asset, status, and timestamp.
- Settings: chain RPC endpoints, indexer endpoint or API key settings, chain
  enablement, biometrics, lock app, app version, and diagnostics.

Flutter may temporarily hold user-entered or user-visible mnemonics, passwords,
addresses, QR payloads, amounts, and notes in form state. It must not persist
them outside the intended encrypted Rust storage.

## QR Transfer Design

QR transfer support includes:

- Camera scanning on supported platforms.
- Image import and QR decode.
- Bare BTC, EVM, and TRON addresses.
- `bitcoin:` URI parsing.
- `ethereum:` URI parsing where chain and token semantics are unambiguous.
- TRON address parsing.
- Optional amount and note prefill when the payload includes them.

Parsed QR data never bypasses validation or confirmation. The result always
flows through the same transfer preview and confirmation screen as manual input.

## Error Handling

Errors returned to Flutter must be user-safe.

- Wrong master password: generic unlock failure.
- Missing database: initialize local state and show first-run flow.
- Corrupt keystore or database: refuse unlock and instruct restoration from
  mnemonic.
- Invalid mnemonic: block import before keystore creation.
- Invalid recipient: block transfer preview.
- Invalid token contract: block custom token addition.
- RPC unavailable: show endpoint failure and keep wallet state unchanged.
- Indexer unavailable: show sync degradation and keep wallet access available.
- Insufficient funds: block signing and show required amount plus estimated fee.
- Broadcast failure: store safe failure metadata and avoid showing raw signed
  transaction payloads by default.
- FFI failure: surface a safe error and preserve Rust lock state.

## Testing Strategy

Rust tests:

- Mnemonic generation, validation, seed derivation, and deterministic address
  derivation.
- Master password setup, unlock, lock, wrong password, and biometric metadata
  behavior with platform-dependent pieces abstracted.
- Keystore encryption, corrupt keystore handling, and no plaintext persistence.
- SQLite schema creation and persistence behavior against temporary databases.
- Multi-wallet create, import, rename, delete, switch, and reveal mnemonic
  behavior.
- BTC, EVM, and TRON address validation.
- ERC20 and TRC20 metadata reads with mocked RPC clients.
- Native coin and token transfer preview behavior.
- Signing and broadcast through mocked chain clients.
- Indexer sync ingestion for transfers, approvals, swaps, contract calls, and
  failed transactions through mocked providers.
- QR and payment URI parsing fixtures.
- FFI responses proving private material is excluded.

Flutter tests:

- First-run master password setup.
- Unlock and optional biometric flow states.
- Wallet create, import, switch, rename, delete, and reveal mnemonic flow.
- Asset list, refresh state, token visibility, and manual token addition.
- QR scan result parsing through mocked scanner adapters.
- Transfer entry, preview, confirmation, success, and failure states.
- Activity list filtering and provider sync failure states.
- Settings for RPC, indexer, chain enablement, biometrics, and locking.

Build verification:

- Rust workspace builds and tests pass.
- Flutter tests pass.
- macOS Flutter build passes.
- Android Flutter build passes.
- iOS Flutter build passes.

Real public RPC and indexer calls may be used for smoke checks, but stable tests
must rely on deterministic fixtures and mocked clients.

## Delivery Strategy

Although the rewrite is a hard cutover, implementation still proceeds in
verifiable slices:

1. Create the Rust workspace, Flutter app, FFI boundary, SQLite schema, and
   minimal app shell.
2. Implement master password, multi-wallet keystore, and basic wallet
   management.
3. Implement account derivation and address display for BTC, EVM chains, and
   TRON.
4. Implement chain settings, public RPC defaults, and user RPC overrides.
5. Implement asset models, native balances, ERC20 and TRC20 metadata, and manual
   token addition.
6. Implement transfer preview, signing, broadcast, and transaction records.
7. Implement indexer configuration, token discovery, and complete address
   activity sync.
8. Implement QR transfer support.
9. Implement optional biometric unlock.
10. Verify macOS, Android, and iOS builds.
11. Remove or archive Python and Electron runtime files after Rust and Flutter
    tests and builds cover the replacement behavior.

Old Python and Electron runtime files must not be deleted before the new Rust
and Flutter skeleton exists and the implementation plan defines the exact
removal checkpoint.

## Non-Goals

- Old database migration.
- Private key export.
- WalletConnect.
- DApp connection.
- Built-in DApp browser.
- Hardware wallets.
- Cloud backup.
- Custodial accounts.
- Exchange integration.
