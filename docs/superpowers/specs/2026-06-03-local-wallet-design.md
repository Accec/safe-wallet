# Python Local Wallet With Electron UI Design

## Scope

Build a local, non-custodial desktop wallet. The first release supports BTC, ETH/EVM chains, and TRON. The wallet lets a user create or import a BIP39 mnemonic, encrypt it locally, derive addresses, query balances, prepare transactions, sign transactions locally, and broadcast them.

The product is not a custodial service, exchange account manager, browser wallet extension, cloud-synced wallet, or hosted key service. Private keys and mnemonic material stay on the user's machine.

## First Release Chains

- BTC: native Bitcoin address derivation, UTXO balance lookup, transaction construction, signing, and broadcast.
- ETH/EVM: Ethereum-style address derivation and transaction support. Chain configs cover Ethereum, BSC, Polygon, Arbitrum, and Optimism through RPC settings.
- TRON: TRON address derivation from the same HD wallet seed, balance lookup, TRX transfer construction, signing, and broadcast.

The chain layer is adapter based so later releases can add Solana, Cosmos, TON, XRP, Litecoin, Dogecoin, or other chains without changing the keystore, storage, or UI contract.

## Architecture

The app has two major parts:

- Electron desktop app in `apps/desktop`
- Python wallet backend in `wallet_core` and `wallet_api`

Electron owns windows, navigation, forms, and visual state. Python owns cryptography, mnemonic handling, keystore encryption, async SQLite storage through Tortoise-ORM, address derivation, balance lookup, transaction construction, signing, and broadcasting.

The Python backend is async-first for I/O. FastAPI handlers are async, SQLite access goes through Tortoise-ORM with the SQLite/aiosqlite backend, and network clients use async HTTP/RPC clients wherever the upstream library supports them. Local CPU-bound cryptographic work, such as BIP39 seed derivation, password KDF, AES-GCM encryption, and transaction signing, remains synchronous inside Python because those operations are not I/O.

Electron launches the Python API as a local child process. The API listens only on `127.0.0.1` and exposes a small HTTP interface protected by a per-session token known only to the Electron main process and Python child process. The renderer process can briefly handle mnemonic words during create/import flows because the user must see or enter them, but it must never persist them. Private keys never enter Electron renderer state. Sensitive signing operations happen inside Python after the user unlocks the keystore.

## Python Modules

- `wallet_core.storage`: Tortoise-ORM models, async SQLite lifecycle, public account rows, settings, transaction metadata, and encrypted keystore row persistence.
- `wallet_core.keystore`: encrypted keystore item creation, import, password-based key derivation, unlock state, lock state, and sensitive wallet material handling with async persistence.
- `wallet_core.mnemonic`: BIP39 mnemonic generation, import validation, and seed derivation.
- `wallet_core.accounts`: HD account derivation paths and account metadata.
- `wallet_core.chains.base`: async chain adapter interface for address validation, balance lookup, transaction preview, signing, and broadcasting.
- `wallet_core.chains.evm`: EVM chain adapter using `AsyncWeb3` and `AsyncHTTPProvider` for configured JSON-RPC endpoints.
- `wallet_core.chains.btc`: BTC chain adapter using async HTTP clients for UTXO data and broadcast where endpoint support allows it.
- `wallet_core.chains.tron`: TRON chain adapter using async HTTP clients for configured TRON endpoints where endpoint support allows it.
- `wallet_core.transactions`: shared transaction preview and confirmation models.
- `wallet_api`: local API server used by Electron.

## Electron UI

The UI has these screens:

- Welcome: choose create wallet or import wallet.
- Create wallet: generate mnemonic, require the user to acknowledge backup responsibility, set a local password, create the encrypted keystore item, and store public metadata in SQLite.
- Import wallet: input an existing mnemonic, validate it, set a local password, create the encrypted keystore item, and store public metadata in SQLite.
- Unlock: enter the local password to unlock the keystore.
- Assets: show enabled chains, addresses, balances, and refresh state.
- Transfer: choose chain, recipient address, amount, and optional note.
- Confirm transfer: show chain, recipient, amount, fee estimate, total spend, source address, and network endpoint before signing.
- Activity: show broadcast results and transaction hashes stored locally.
- Settings: configure RPC endpoints, enable or disable supported chains, and lock wallet.

## Data Flow

Create wallet:

1. Electron requests a new wallet session from the Python API.
2. Python generates a BIP39 mnemonic.
3. Electron displays the mnemonic once for backup, then clears it from UI state after keystore creation.
4. User sets a local password.
5. Python encrypts the mnemonic-derived wallet material into a keystore record using password-derived AES-GCM encryption.
6. Python asynchronously stores the encrypted keystore record, public account metadata, settings, and empty transaction metadata tables in SQLite through Tortoise-ORM.
7. Python returns public account metadata and addresses to Electron.

Import wallet:

1. Electron sends the entered mnemonic to Python over the local API.
2. Python validates the mnemonic.
3. User sets a local password.
4. Python encrypts the wallet material into a keystore record.
5. Python asynchronously stores encrypted keystore and public metadata in SQLite through Tortoise-ORM.
6. Python returns public account metadata and addresses.

Unlock wallet:

1. Electron sends the password to Python.
2. Python asynchronously loads the encrypted keystore record from SQLite through Tortoise-ORM.
3. Python attempts to decrypt the keystore.
4. On success, Python keeps sensitive material in process memory only while unlocked.
5. On failure, Python returns a generic unlock error without exposing keystore internals.

Transfer:

1. Electron sends chain, recipient, and amount to Python.
2. Python validates the recipient address for the selected chain.
3. Python asynchronously queries network state needed for the transaction.
4. Python returns a transaction preview with fees and total spend.
5. Electron shows a confirmation screen.
6. After user confirmation, Python signs inside the backend and broadcasts.
7. Python returns transaction hash and broadcast status.

## Security Requirements

- Store no plaintext mnemonic, seed, or private key on disk.
- Do not expose private keys to Electron renderer state.
- Store encrypted mnemonic/seed material in SQLite only as AES-GCM keystore ciphertext with KDF metadata, persisted through Tortoise-ORM models.
- Store public addresses, chain settings, and transaction metadata in SQLite separately from encrypted keystore ciphertext.
- Do not persist plaintext mnemonics in Electron. During create/import, clear mnemonic UI state immediately after keystore creation/import completes.
- Bind the local API to `127.0.0.1` only.
- Protect local API requests with a per-session token and reject requests without it.
- Require an unlocked keystore for private derivation, transaction preview, signing, and broadcast. Public account metadata can be read from SQLite while locked.
- Require explicit user confirmation before signing.
- Show chain, recipient, amount, fee, source address, and endpoint before signing.
- Lock the keystore on request and when the Python API process exits.
- Return generic errors for password failures.
- Avoid logging mnemonic, seed, private keys, passwords, signed transaction payloads, or authorization material.
- Disable renderer-side persistence of sensitive form fields and avoid browser storage for mnemonic/password data.

## Error Handling

- Wrong password: return a generic unlock failure.
- Missing keystore/database: initialize SQLite and show create/import flow.
- Corrupt keystore row or SQLite database: refuse unlock and advise restore from mnemonic.
- Invalid mnemonic: block import before keystore creation.
- Invalid recipient: block transaction preview.
- RPC unavailable: show endpoint failure and keep keystore state unchanged.
- Insufficient funds: block signing and show required amount plus estimated fee.
- Broadcast failure: keep signed transaction result internal by default and show network error details safe for users.

## Configuration

The app stores all local state in SQLite through Tortoise-ORM, with sensitive wallet material encrypted before it is written:

- Encrypted keystore items containing mnemonic-derived wallet material ciphertext and KDF metadata.
- Public accounts and derivation paths.
- Enabled chains.
- RPC endpoints for EVM chains, BTC, and TRON.
- Default account index.
- UI preferences.
- Recent transaction metadata without secrets.

## Testing Strategy

- Unit tests for mnemonic generation, mnemonic import validation, and seed derivation.
- Unit tests for async Tortoise-ORM SQLite initialization, keystore encryption, unlock, lock, wrong password, and corrupt keystore handling.
- Unit tests proving SQLite never stores plaintext mnemonic, seed, private keys, or passwords.
- Unit tests for BTC, EVM, and TRON address derivation using known test vectors where available.
- Unit tests for transaction preview validation and insufficient balance paths.
- API tests for async local endpoint behavior and locked/unlocked access.
- Electron tests for create/import/unlock/navigation forms with backend mocked where appropriate.
- Integration smoke tests for the Python API with a temporary SQLite database.

## Initial Implementation Boundaries

The first implementation includes real local Tortoise-backed SQLite storage, encrypted keystore behavior, BIP39 mnemonic generation/import, BTC/EVM/TRON account derivation, async local API endpoints, and an Electron UI for create/import/unlock/assets/transfer preview/send boundary flows.

The current chain adapters expose async RPC integration points and deterministic offline previews for tests. Real chain-specific network balance queries, transaction assembly, and broadcast clients are the next production-hardening step before moving value on mainnet.

TRC20 tokens, ERC20 tokens, address book, hardware wallet support, cloud sync, QR code scanning, multi-signature wallets, and Solana support are planned extensions, not first-release requirements.
