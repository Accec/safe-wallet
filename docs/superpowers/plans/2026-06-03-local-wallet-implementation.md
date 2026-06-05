# Local Wallet Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a local non-custodial desktop wallet with a Python wallet core, local Python API, and Electron UI supporting BTC, ETH/EVM, and TRON.

**Architecture:** The Python backend owns all wallet, keystore, async Tortoise-backed SQLite storage, derivation, chain, and signing behavior. Electron owns windows and UI state, launches the Python API as a local child process, and communicates through a session-token-protected loopback API. The first working version includes real encrypted keystore records stored in SQLite through Tortoise-ORM and deterministic address derivation, with async chain network clients behind adapters so RPC integration and tests stay separate.

**Tech Stack:** Python 3.11-3.13, SQLite, Tortoise-ORM, aiosqlite, FastAPI, Uvicorn, Pydantic, cryptography, bip-utils, AsyncWeb3/Web3.py, aiohttp, pytest, pytest-asyncio, Electron, Vite, React, TypeScript.

---

## Scope Check

The approved spec includes three connected subsystems: Python wallet core, local API, and Electron UI. They are implemented in one ordered plan because each subsystem depends on the previous public contract and the final deliverable is a single desktop wallet. Each task below produces a working, testable slice and ends with a commit.

## Target File Structure

- `pyproject.toml`: Python package metadata, runtime dependencies, test configuration.
- `package.json`: workspace scripts for Electron development and tests.
- `wallet_core/__init__.py`: wallet core package marker and version export.
- `wallet_core/errors.py`: typed domain exceptions with safe API messages.
- `wallet_core/models.py`: Pydantic/domain dataclasses shared by core and API.
- `wallet_core/mnemonic.py`: BIP39 generation, validation, and seed derivation.
- `wallet_core/storage.py`: Tortoise-ORM models, async SQLite lifecycle, encrypted keystore persistence, public account rows, settings, and transaction metadata.
- `wallet_core/keystore.py`: encrypted keystore create, import, unlock, lock, and status behavior using async SQLite-backed storage.
- `wallet_core/accounts.py`: BTC, EVM, and TRON HD account derivation.
- `wallet_core/chains/base.py`: async chain adapter protocol and shared transaction preview types.
- `wallet_core/chains/evm.py`: EVM address validation, async Web3.py RPC, transfer preview, signing boundary.
- `wallet_core/chains/btc.py`: BTC address validation, async HTTP UTXO client boundary, transfer preview, signing boundary.
- `wallet_core/chains/tron.py`: TRON address validation, async HTTP client boundary, transfer preview, signing boundary.
- `wallet_core/settings.py`: non-sensitive app settings and chain endpoint defaults.
- `wallet_api/main.py`: async FastAPI app factory, token middleware, wallet endpoints.
- `wallet_api/state.py`: process-local app state, unlocked keystore state, SQLite storage handle, adapter registry.
- `wallet_api/__main__.py`: CLI entrypoint used by Electron to start the API.
- `tests/`: Python tests for keystore, SQLite storage, mnemonic, derivation, adapters, and API behavior.
- `apps/desktop/package.json`: Electron app package metadata.
- `apps/desktop/index.html`: Vite entry HTML.
- `apps/desktop/src/main.ts`: Electron main process, Python child process launcher, window creation.
- `apps/desktop/src/preload.ts`: safe IPC bridge for renderer API calls.
- `apps/desktop/src/renderer/App.tsx`: wallet UI state machine and screens.
- `apps/desktop/src/renderer/api.ts`: typed client for preload bridge.
- `apps/desktop/src/renderer/styles.css`: desktop UI layout and responsive styling.
- `apps/desktop/src/renderer/types.ts`: TypeScript models mirroring the Python API response shapes.

---

### Task 1: Repository Scaffold

**Files:**
- Create: `pyproject.toml`
- Create: `package.json`
- Create: `wallet_core/__init__.py`
- Create: `wallet_api/__init__.py`
- Create: `wallet_core/chains/__init__.py`
- Create: `tests/test_scaffold.py`

- [ ] **Step 1: Write the scaffold smoke test**

Create `tests/test_scaffold.py`:

```python
from wallet_core import __version__


def test_wallet_core_version_is_exposed() -> None:
    assert __version__ == "0.1.0"
```

- [ ] **Step 2: Run the smoke test to verify it fails**

Run:

```bash
pytest tests/test_scaffold.py -q
```

Expected: FAIL with `ModuleNotFoundError: No module named 'wallet_core'`.

- [ ] **Step 3: Add Python packaging and package markers**

Create `pyproject.toml`:

```toml
[build-system]
requires = ["setuptools>=69", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "local-wallet"
version = "0.1.0"
description = "Local non-custodial wallet core and API"
requires-python = ">=3.11"
dependencies = [
  "base58>=2.1.1",
  "bip-utils>=2.9.3",
  "cryptography>=42.0.0",
  "fastapi>=0.111.0",
  "httpx>=0.27.0",
  "pydantic>=2.7.0",
  "tortoise-orm>=0.25.0",
  "aiosqlite>=0.20.0",
  "web3>=6.20.0",
  "aiohttp>=3.9.0",
  "tronpy>=0.5.0",
  "bitcoinlib>=0.7.0",
  "uvicorn>=0.30.0",
]

[project.optional-dependencies]
dev = [
  "pytest>=8.2.0",
  "pytest-asyncio>=0.23.0",
  "ruff>=0.5.0",
]

[tool.pytest.ini_options]
testpaths = ["tests"]
asyncio_mode = "auto"

[tool.ruff]
line-length = 100
target-version = "py311"
```

Create `package.json`:

```json
{
  "name": "local-wallet-workspace",
  "private": true,
  "scripts": {
    "test:py": "pytest -q",
    "lint:py": "ruff check wallet_core wallet_api tests",
    "desktop:dev": "npm --prefix apps/desktop run dev",
    "desktop:test": "npm --prefix apps/desktop run test"
  }
}
```

Create `wallet_core/__init__.py`:

```python
__version__ = "0.1.0"
```

Create `wallet_api/__init__.py`:

```python
"""Local API for the desktop wallet."""
```

Create `wallet_core/chains/__init__.py`:

```python
"""Chain adapters for supported wallet networks."""
```

- [ ] **Step 4: Install Python package in editable mode**

Run:

```bash
python -m pip install -e ".[dev]"
```

Expected: dependencies install and the package is available to pytest.

- [ ] **Step 5: Run the smoke test to verify it passes**

Run:

```bash
pytest tests/test_scaffold.py -q
```

Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```bash
git add pyproject.toml package.json wallet_core wallet_api tests/test_scaffold.py
git commit -m "chore: scaffold wallet project"
```

---

### Task 2: Domain Models and Safe Errors

**Files:**
- Create: `wallet_core/errors.py`
- Create: `wallet_core/models.py`
- Create: `tests/test_models.py`

- [ ] **Step 1: Write failing model and error tests**

Create `tests/test_models.py`:

```python
from dataclasses import asdict
from decimal import Decimal

import pytest

from wallet_core.errors import InvalidAddressError, WalletError
from wallet_core.models import (
    Chain,
    ChainAccount,
    SigningAccount,
    TransactionPreview,
    chain_account_to_api,
    transaction_preview_to_api,
)


def test_wallet_error_exposes_safe_message() -> None:
    err = InvalidAddressError("eth", "bad-address")
    assert isinstance(err, WalletError)
    assert err.safe_message == "Invalid eth address"


def test_transaction_preview_total_spend() -> None:
    preview = TransactionPreview(
        chain=Chain.ETH,
        from_address="0x1111111111111111111111111111111111111111",
        to_address="0x2222222222222222222222222222222222222222",
        amount=Decimal("1.25"),
        fee=Decimal("0.01"),
        asset="ETH",
        network="Ethereum",
    )
    assert preview.total == Decimal("1.26")
    assert asdict(preview)["total"] == Decimal("1.26")


def test_chain_account_is_public_metadata() -> None:
    account = ChainAccount(
        chain=Chain.TRON,
        network="TRON",
        address="TQ5p2nQ6QgX3L6Lw9zSX6WZQbVqjQxKQ3L",
        derivation_path="m/44'/195'/0'/0/0",
    )
    assert account.private_key is None


def test_chain_account_serialization_excludes_private_key() -> None:
    account = ChainAccount(
        chain=Chain.TRON,
        network="TRON",
        address="TQ5p2nQ6QgX3L6Lw9zSX6WZQbVqjQxKQ3L",
        derivation_path="m/44'/195'/0'/0/0",
    )

    assert "private_key" not in asdict(account)
    assert chain_account_to_api(account) == {
        "chain": "tron",
        "network": "TRON",
        "address": "TQ5p2nQ6QgX3L6Lw9zSX6WZQbVqjQxKQ3L",
        "derivation_path": "m/44'/195'/0'/0/0",
    }


def test_transaction_preview_api_includes_total() -> None:
    preview = TransactionPreview(
        chain=Chain.ETH,
        from_address="0x1111111111111111111111111111111111111111",
        to_address="0x2222222222222222222222222222222222222222",
        amount=Decimal("1.25"),
        fee=Decimal("0.01"),
        asset="ETH",
        network="Ethereum",
    )

    assert transaction_preview_to_api(preview) == {
        "chain": "eth",
        "network": "Ethereum",
        "from_address": "0x1111111111111111111111111111111111111111",
        "to_address": "0x2222222222222222222222222222222222222222",
        "amount": "1.25",
        "fee": "0.01",
        "total": "1.26",
        "asset": "ETH",
    }


def test_signing_account_can_hold_private_key_as_internal_model() -> None:
    account = SigningAccount(
        chain=Chain.ETH,
        network="Ethereum",
        address="0x1111111111111111111111111111111111111111",
        derivation_path="m/44'/60'/0'/0/0",
        private_key="secret",
    )

    assert account.private_key == "secret"
    assert not isinstance(account, ChainAccount)


def test_signing_account_redacts_private_key_from_repr() -> None:
    account = SigningAccount(
        chain=Chain.ETH,
        network="Ethereum",
        address="0x1111111111111111111111111111111111111111",
        derivation_path="m/44'/60'/0'/0/0",
        private_key="secret",
    )

    assert "secret" not in repr(account)
    assert "<redacted>" in repr(account)


def test_signing_account_is_not_dataclass_serializable() -> None:
    account = SigningAccount(
        chain=Chain.ETH,
        network="Ethereum",
        address="0x1111111111111111111111111111111111111111",
        derivation_path="m/44'/60'/0'/0/0",
        private_key="secret",
    )

    with pytest.raises(TypeError):
        asdict(account)
```

- [ ] **Step 2: Run tests to verify they fail**

Run:

```bash
pytest tests/test_models.py -q
```

Expected: FAIL with missing `wallet_core.errors` and `wallet_core.models`.

- [ ] **Step 3: Implement safe errors and shared models**

Create `wallet_core/errors.py`:

```python
class WalletError(Exception):
    """Base wallet error carrying a safe message for UI/API callers."""

    safe_message = "Wallet operation failed"

    def __init__(self, detail: str | None = None) -> None:
        super().__init__(detail or self.safe_message)
        self.detail = detail


class InvalidAddressError(WalletError):
    def __init__(self, chain: str, address: str) -> None:
        self.chain = chain
        self.address = address
        self.safe_message = f"Invalid {chain} address"
        super().__init__(self.safe_message)


class InvalidMnemonicError(WalletError):
    safe_message = "Invalid mnemonic"


class KeystoreExistsError(WalletError):
    safe_message = "Keystore already exists"


class KeystoreMissingError(WalletError):
    safe_message = "Keystore does not exist"


class KeystoreLockedError(WalletError):
    safe_message = "Wallet is locked"


class KeystoreCorruptError(WalletError):
    safe_message = "Keystore is corrupt"


class UnlockFailedError(WalletError):
    safe_message = "Unable to unlock wallet"


class InsufficientFundsError(WalletError):
    safe_message = "Insufficient funds"


class RpcError(WalletError):
    safe_message = "Network endpoint failed"
```

Create `wallet_core/models.py`:

```python
from __future__ import annotations

from dataclasses import dataclass, field
from decimal import Decimal
from enum import StrEnum


class Chain(StrEnum):
    BTC = "btc"
    ETH = "eth"
    BSC = "bsc"
    POLYGON = "polygon"
    ARBITRUM = "arbitrum"
    OPTIMISM = "optimism"
    TRON = "tron"


@dataclass(frozen=True)
class ChainAccount:
    chain: Chain
    network: str
    address: str
    derivation_path: str

    @property
    def private_key(self) -> None:
        return None


class SigningAccount:
    __slots__ = ("chain", "network", "address", "derivation_path", "_private_key")

    chain: Chain
    network: str
    address: str
    derivation_path: str
    _private_key: str

    def __init__(
        self,
        chain: Chain,
        network: str,
        address: str,
        derivation_path: str,
        private_key: str,
    ) -> None:
        self.chain = chain
        self.network = network
        self.address = address
        self.derivation_path = derivation_path
        self._private_key = private_key

    @property
    def private_key(self) -> str:
        return self._private_key

    def __repr__(self) -> str:
        return (
            "SigningAccount("
            f"chain={self.chain!r}, "
            f"network={self.network!r}, "
            f"address={self.address!r}, "
            f"derivation_path={self.derivation_path!r}, "
            "private_key='<redacted>'"
            ")"
        )


@dataclass(frozen=True)
class Balance:
    chain: Chain
    network: str
    address: str
    asset: str
    amount: Decimal


@dataclass(frozen=True)
class TransactionPreview:
    chain: Chain
    from_address: str
    to_address: str
    amount: Decimal
    fee: Decimal
    asset: str
    network: str
    total: Decimal = field(init=False)

    def __post_init__(self) -> None:
        object.__setattr__(self, "total", self.amount + self.fee)


def chain_account_to_api(account: ChainAccount) -> dict[str, str]:
    return {
        "chain": account.chain.value,
        "network": account.network,
        "address": account.address,
        "derivation_path": account.derivation_path,
    }


def transaction_preview_to_api(preview: TransactionPreview) -> dict[str, str]:
    return {
        "chain": preview.chain.value,
        "network": preview.network,
        "from_address": preview.from_address,
        "to_address": preview.to_address,
        "amount": str(preview.amount),
        "fee": str(preview.fee),
        "total": str(preview.total),
        "asset": preview.asset,
    }


@dataclass(frozen=True)
class BroadcastResult:
    chain: Chain
    tx_hash: str
    status: str
```

- [ ] **Step 4: Run tests to verify they pass**

Run:

```bash
pytest tests/test_models.py -q
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```bash
git add wallet_core/errors.py wallet_core/models.py tests/test_models.py
git commit -m "feat: add wallet domain models"
```

---

### Task 3: Mnemonic Generation and Validation

**Files:**
- Create: `wallet_core/mnemonic.py`
- Create: `tests/test_mnemonic.py`

- [ ] **Step 1: Write failing mnemonic tests**

Create `tests/test_mnemonic.py`:

```python
import pytest

from wallet_core.errors import InvalidMnemonicError
from wallet_core.mnemonic import generate_mnemonic, mnemonic_to_seed, validate_mnemonic

VALID_MNEMONIC = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"


def test_generate_mnemonic_returns_12_words() -> None:
    words = generate_mnemonic().split()
    assert len(words) == 12


def test_validate_mnemonic_accepts_valid_bip39_phrase() -> None:
    assert validate_mnemonic(VALID_MNEMONIC) == VALID_MNEMONIC


def test_validate_mnemonic_rejects_invalid_phrase() -> None:
    with pytest.raises(InvalidMnemonicError):
        validate_mnemonic("not a valid seed phrase")


def test_mnemonic_to_seed_is_deterministic() -> None:
    seed_one = mnemonic_to_seed(VALID_MNEMONIC)
    seed_two = mnemonic_to_seed(VALID_MNEMONIC)
    assert seed_one == seed_two
    assert len(seed_one) == 64
```

- [ ] **Step 2: Run tests to verify they fail**

Run:

```bash
pytest tests/test_mnemonic.py -q
```

Expected: FAIL with missing `wallet_core.mnemonic`.

- [ ] **Step 3: Implement BIP39 helpers**

Create `wallet_core/mnemonic.py`:

```python
from bip_utils import Bip39Languages, Bip39MnemonicGenerator, Bip39MnemonicValidator, Bip39SeedGenerator, Bip39WordsNum

from wallet_core.errors import InvalidMnemonicError


def generate_mnemonic() -> str:
    return str(Bip39MnemonicGenerator(Bip39Languages.ENGLISH).FromWordsNumber(Bip39WordsNum.WORDS_NUM_12))


def validate_mnemonic(mnemonic: str) -> str:
    normalized = " ".join(mnemonic.strip().lower().split())
    try:
        Bip39MnemonicValidator(Bip39Languages.ENGLISH).Validate(normalized)
    except Exception as exc:
        raise InvalidMnemonicError() from exc
    return normalized


def mnemonic_to_seed(mnemonic: str, passphrase: str = "") -> bytes:
    normalized = validate_mnemonic(mnemonic)
    return Bip39SeedGenerator(normalized).Generate(passphrase)
```

- [ ] **Step 4: Run mnemonic tests**

Run:

```bash
pytest tests/test_mnemonic.py -q
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```bash
git add wallet_core/mnemonic.py tests/test_mnemonic.py
git commit -m "feat: add bip39 mnemonic support"
```

---

### Task 4: SQLite-Backed Keystore

**Files:**
- Create: `wallet_core/storage.py`
- Create: `wallet_core/keystore.py`
- Create: `tests/test_keystore.py`

- [ ] **Step 1: Write failing keystore tests**

Create `tests/test_keystore.py`:

```python
import sqlite3

import pytest
from tortoise import Tortoise

from wallet_core.errors import KeystoreExistsError, KeystoreMissingError, UnlockFailedError
from wallet_core.keystore import Keystore
from wallet_core.models import Chain, ChainAccount
from wallet_core.storage import WalletDatabase

MNEMONIC = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
pytestmark = pytest.mark.asyncio


@pytest.fixture(autouse=True)
async def close_tortoise_connections():
    try:
        yield
    finally:
        await Tortoise.close_connections()


async def test_database_initializes_required_tables(tmp_path) -> None:
    db = WalletDatabase(tmp_path / "wallet.sqlite")
    await db.initialize()

    with sqlite3.connect(db.path) as conn:
        tables = {row[0] for row in conn.execute("select name from sqlite_master where type='table'")}

    assert {"keystore_items", "accounts", "settings", "transactions"} <= tables


async def test_create_encrypts_mnemonic_without_plaintext_in_sqlite(tmp_path) -> None:
    db = WalletDatabase(tmp_path / "wallet.sqlite")
    keystore = Keystore(db)

    await keystore.create(MNEMONIC, "correct horse battery staple")

    raw = db.path.read_bytes()
    assert MNEMONIC.encode() not in raw
    assert b"correct horse battery staple" not in raw
    row = await db.get_keystore_item()
    assert row is not None
    assert row["version"] == 1
    assert row["kdf_name"] == "scrypt"
    assert keystore.is_locked is True


async def test_create_refuses_to_overwrite_existing_keystore(tmp_path) -> None:
    db = WalletDatabase(tmp_path / "wallet.sqlite")
    keystore = Keystore(db)
    await keystore.create(MNEMONIC, "password one")

    with pytest.raises(KeystoreExistsError):
        await keystore.create(MNEMONIC, "password two")


async def test_unlock_returns_mnemonic_and_keeps_process_unlocked(tmp_path) -> None:
    db = WalletDatabase(tmp_path / "wallet.sqlite")
    keystore = Keystore(db)
    await keystore.create(MNEMONIC, "correct horse battery staple")

    material = await keystore.unlock("correct horse battery staple")

    assert material.mnemonic == MNEMONIC
    assert keystore.is_locked is False
    assert keystore.material.mnemonic == MNEMONIC


async def test_wrong_password_returns_generic_unlock_failure(tmp_path) -> None:
    db = WalletDatabase(tmp_path / "wallet.sqlite")
    keystore = Keystore(db)
    await keystore.create(MNEMONIC, "correct horse battery staple")

    with pytest.raises(UnlockFailedError):
        await keystore.unlock("wrong password")


async def test_lock_clears_process_material(tmp_path) -> None:
    db = WalletDatabase(tmp_path / "wallet.sqlite")
    keystore = Keystore(db)
    await keystore.create(MNEMONIC, "correct horse battery staple")
    await keystore.unlock("correct horse battery staple")

    keystore.lock()

    assert keystore.is_locked is True
    assert keystore.material is None


async def test_unlock_missing_keystore_fails(tmp_path) -> None:
    db = WalletDatabase(tmp_path / "wallet.sqlite")
    keystore = Keystore(db)
    with pytest.raises(KeystoreMissingError):
        await keystore.unlock("password")


async def test_database_saves_and_loads_public_accounts(tmp_path) -> None:
    db = WalletDatabase(tmp_path / "wallet.sqlite")
    accounts = [
        ChainAccount(
            chain=Chain.TRON,
            network="TRON",
            address="TQ5p2nQ6QgX3L6Lw9zSX6WZQbVqjQxKQ3L",
            derivation_path="m/44'/195'/0'/0/0",
        )
    ]

    await db.save_accounts(accounts)

    assert await db.list_accounts() == accounts
```

- [ ] **Step 2: Run tests to verify they fail**

Run:

```bash
pytest tests/test_keystore.py -q
```

Expected: FAIL with missing `wallet_core.keystore` and `wallet_core.storage`.

- [ ] **Step 3: Implement SQLite storage**

Create `wallet_core/storage.py`:

```python
from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from tortoise import Tortoise, fields
from tortoise.models import Model

from wallet_core.models import Chain, ChainAccount


class KeystoreItem(Model):
    id = fields.IntField(primary_key=True)
    version = fields.IntField()
    kdf_name = fields.CharField(max_length=32)
    kdf_params = fields.TextField()
    cipher_name = fields.CharField(max_length=64)
    nonce = fields.BinaryField()
    ciphertext = fields.BinaryField()
    created_at = fields.DatetimeField(auto_now_add=True)
    updated_at = fields.DatetimeField(auto_now=True)

    class Meta:
        table = "keystore_items"


class AccountRow(Model):
    id = fields.IntField(primary_key=True)
    chain = fields.CharField(max_length=32)
    network = fields.CharField(max_length=64)
    address = fields.CharField(max_length=128)
    derivation_path = fields.CharField(max_length=64)
    created_at = fields.DatetimeField(auto_now_add=True)

    class Meta:
        table = "accounts"
        unique_together = (("chain", "derivation_path"),)


class SettingRow(Model):
    key = fields.CharField(max_length=128, primary_key=True)
    value = fields.TextField()

    class Meta:
        table = "settings"


class TransactionRow(Model):
    id = fields.IntField(primary_key=True)
    chain = fields.CharField(max_length=32)
    tx_hash = fields.CharField(max_length=128)
    status = fields.CharField(max_length=32)
    created_at = fields.DatetimeField(auto_now_add=True)

    class Meta:
        table = "transactions"


class WalletDatabase:
    def __init__(self, path: Path) -> None:
        self.path = path
        self._context: Any | None = None
        self._initialized = False

    async def initialize(self) -> None:
        if self._initialized:
            return
        self.path.parent.mkdir(parents=True, exist_ok=True)
        self._context = await Tortoise.init(
            db_url=f"sqlite://{self.path}",
            modules={"models": ["wallet_core.storage"]},
            _enable_global_fallback=True,
        )
        await Tortoise.generate_schemas(safe=True)
        self._initialized = True

    async def close(self) -> None:
        if self._context is not None:
            await self._context.close_connections()
            self._context = None
        else:
            await Tortoise.close_connections()
        self._initialized = False

    async def has_keystore(self) -> bool:
        await self.initialize()
        return await KeystoreItem.filter(id=1).exists()

    async def save_keystore_item(
        self,
        *,
        version: int,
        kdf_name: str,
        kdf_params: dict[str, Any],
        cipher_name: str,
        nonce: bytes,
        ciphertext: bytes,
    ) -> None:
        await self.initialize()
        await KeystoreItem.create(
            id=1,
            version=version,
            kdf_name=kdf_name,
            kdf_params=json.dumps(kdf_params, separators=(",", ":")),
            cipher_name=cipher_name,
            nonce=nonce,
            ciphertext=ciphertext,
        )

    async def get_keystore_item(self) -> dict[str, Any] | None:
        await self.initialize()
        row = await KeystoreItem.filter(id=1).first()
        if row is None:
            return None
        return {
            "version": row.version,
            "kdf_name": row.kdf_name,
            "kdf_params": json.loads(row.kdf_params),
            "cipher_name": row.cipher_name,
            "nonce": bytes(row.nonce),
            "ciphertext": bytes(row.ciphertext),
        }

    async def save_accounts(self, accounts: list[ChainAccount]) -> None:
        await self.initialize()
        await AccountRow.all().delete()
        await AccountRow.bulk_create(
            [
                AccountRow(
                    chain=account.chain.value,
                    network=account.network,
                    address=account.address,
                    derivation_path=account.derivation_path,
                )
                for account in accounts
            ]
        )

    async def list_accounts(self) -> list[ChainAccount]:
        await self.initialize()
        rows = await AccountRow.all().order_by("id")
        return [
            ChainAccount(
                chain=Chain(row.chain),
                network=row.network,
                address=row.address,
                derivation_path=row.derivation_path,
            )
            for row in rows
        ]
```

- [ ] **Step 4: Implement AES-GCM keystore encryption**

Create `wallet_core/keystore.py`:

```python
from __future__ import annotations

import json
import os
from dataclasses import dataclass

from cryptography.exceptions import InvalidTag
from cryptography.hazmat.primitives.ciphers.aead import AESGCM
from cryptography.hazmat.primitives.kdf.scrypt import Scrypt

from wallet_core.errors import KeystoreExistsError, KeystoreMissingError, UnlockFailedError
from wallet_core.mnemonic import validate_mnemonic
from wallet_core.storage import WalletDatabase


SCRYPT_N = 2**14
SCRYPT_R = 8
SCRYPT_P = 1
KEY_LEN = 32


@dataclass(frozen=True)
class KeystoreMaterial:
    mnemonic: str


class Keystore:
    def __init__(self, database: WalletDatabase) -> None:
        self.database = database
        self.material: KeystoreMaterial | None = None

    async def exists(self) -> bool:
        return await self.database.has_keystore()

    @property
    def is_locked(self) -> bool:
        return self.material is None

    async def create(self, mnemonic: str, password: str) -> None:
        if await self.database.has_keystore():
            raise KeystoreExistsError()
        normalized = validate_mnemonic(mnemonic)
        salt = os.urandom(16)
        nonce = os.urandom(12)
        key = self._derive_key(password, salt)
        plaintext = json.dumps({"mnemonic": normalized}, separators=(",", ":")).encode()
        ciphertext = AESGCM(key).encrypt(nonce, plaintext, b"local-wallet-keystore-v1")
        await self.database.save_keystore_item(
            version=1,
            kdf_name="scrypt",
            kdf_params={
                "salt": salt.hex(),
                "n": SCRYPT_N,
                "r": SCRYPT_R,
                "p": SCRYPT_P,
            },
            cipher_name="aes-256-gcm",
            nonce=nonce,
            ciphertext=ciphertext,
        )
        self.lock()

    async def unlock(self, password: str) -> KeystoreMaterial:
        payload = await self.database.get_keystore_item()
        if payload is None:
            raise KeystoreMissingError()
        try:
            salt = bytes.fromhex(payload["kdf_params"]["salt"])
            nonce = payload["nonce"]
            ciphertext = payload["ciphertext"]
            key = self._derive_key(password, salt)
            plaintext = AESGCM(key).decrypt(nonce, ciphertext, b"local-wallet-keystore-v1")
            data = json.loads(plaintext.decode("utf-8"))
            material = KeystoreMaterial(mnemonic=validate_mnemonic(data["mnemonic"]))
        except (KeyError, json.JSONDecodeError, InvalidTag, ValueError) as exc:
            raise UnlockFailedError() from exc
        self.material = material
        return material

    def lock(self) -> None:
        self.material = None

    @staticmethod
    def _derive_key(password: str, salt: bytes) -> bytes:
        kdf = Scrypt(salt=salt, length=KEY_LEN, n=SCRYPT_N, r=SCRYPT_R, p=SCRYPT_P)
        return kdf.derive(password.encode("utf-8"))
```

- [ ] **Step 5: Run keystore tests**

Run:

```bash
pytest tests/test_keystore.py -q
```

Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```bash
git add wallet_core/storage.py wallet_core/keystore.py tests/test_keystore.py
git commit -m "feat: add sqlite backed keystore"
```

---

### Task 5: BTC, EVM, and TRON Account Derivation

**Files:**
- Create: `wallet_core/accounts.py`
- Create: `tests/test_accounts.py`

- [ ] **Step 1: Write failing derivation tests**

Create `tests/test_accounts.py`:

```python
from dataclasses import asdict

from wallet_core.accounts import derive_default_accounts, derive_signing_accounts
from wallet_core.models import Chain, SigningAccount

MNEMONIC = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"


def test_derive_default_accounts_returns_first_release_chains() -> None:
    accounts = derive_default_accounts(MNEMONIC)
    chains = {account.chain for account in accounts}

    assert Chain.BTC in chains
    assert Chain.ETH in chains
    assert Chain.BSC in chains
    assert Chain.POLYGON in chains
    assert Chain.ARBITRUM in chains
    assert Chain.OPTIMISM in chains
    assert Chain.TRON in chains


def test_evm_chains_share_the_same_address() -> None:
    accounts = derive_default_accounts(MNEMONIC)
    by_chain = {account.chain: account for account in accounts}

    assert by_chain[Chain.ETH].address.startswith("0x")
    assert by_chain[Chain.ETH].address == by_chain[Chain.BSC].address
    assert by_chain[Chain.ETH].derivation_path == "m/44'/60'/0'/0/0"


def test_btc_and_tron_addresses_use_expected_prefixes() -> None:
    accounts = derive_default_accounts(MNEMONIC)
    by_chain = {account.chain: account for account in accounts}

    assert by_chain[Chain.BTC].address.startswith("bc1")
    assert by_chain[Chain.BTC].derivation_path == "m/84'/0'/0'/0/0"
    assert by_chain[Chain.TRON].address.startswith("T")
    assert by_chain[Chain.TRON].derivation_path == "m/44'/195'/0'/0/0"


def test_public_accounts_do_not_include_private_keys_by_default() -> None:
    accounts = derive_default_accounts(MNEMONIC)
    assert all(account.private_key is None for account in accounts)
    assert all("private_key" not in asdict(account) for account in accounts)


def test_signing_accounts_use_internal_signing_model() -> None:
    accounts = derive_signing_accounts(MNEMONIC)
    assert all(isinstance(account, SigningAccount) for account in accounts)
    assert all(account.private_key for account in accounts)
    assert all("private_key='<redacted>'" in repr(account) for account in accounts)
```

- [ ] **Step 2: Run tests to verify they fail**

Run:

```bash
pytest tests/test_accounts.py -q
```

Expected: FAIL with missing `wallet_core.accounts`.

- [ ] **Step 3: Implement HD account derivation**

Create `wallet_core/accounts.py`:

```python
from bip_utils import Bip44, Bip44Changes, Bip44Coins, Bip84, Bip84Coins

from wallet_core.mnemonic import mnemonic_to_seed
from wallet_core.models import Chain, ChainAccount, SigningAccount


EVM_NETWORKS = {
    Chain.ETH: "Ethereum",
    Chain.BSC: "BNB Smart Chain",
    Chain.POLYGON: "Polygon",
    Chain.ARBITRUM: "Arbitrum",
    Chain.OPTIMISM: "Optimism",
}


def derive_default_accounts(mnemonic: str, account_index: int = 0) -> list[ChainAccount]:
    seed = mnemonic_to_seed(mnemonic)
    btc = _derive_btc(seed, account_index)
    evm_accounts = _derive_evm_accounts(seed, account_index)
    tron = _derive_tron(seed, account_index)
    return [btc, *evm_accounts, tron]


def derive_signing_accounts(mnemonic: str, account_index: int = 0) -> list[SigningAccount]:
    seed = mnemonic_to_seed(mnemonic)
    return [
        _derive_btc_signing(seed, account_index),
        *_derive_evm_signing_accounts(seed, account_index),
        _derive_tron_signing(seed, account_index),
    ]


def _derive_btc(seed: bytes, account_index: int) -> ChainAccount:
    node = (
        Bip84.FromSeed(seed, Bip84Coins.BITCOIN)
        .Purpose()
        .Coin()
        .Account(account_index)
        .Change(Bip44Changes.CHAIN_EXT)
        .AddressIndex(0)
    )
    return ChainAccount(
        chain=Chain.BTC,
        network="Bitcoin",
        address=node.PublicKey().ToAddress(),
        derivation_path=f"m/84'/0'/{account_index}'/0/0",
    )


def _derive_evm_accounts(seed: bytes, account_index: int) -> list[ChainAccount]:
    node = (
        Bip44.FromSeed(seed, Bip44Coins.ETHEREUM)
        .Purpose()
        .Coin()
        .Account(account_index)
        .Change(Bip44Changes.CHAIN_EXT)
        .AddressIndex(0)
    )
    address = node.PublicKey().ToAddress()
    path = f"m/44'/60'/{account_index}'/0/0"
    return [
        ChainAccount(chain=chain, network=network, address=address, derivation_path=path)
        for chain, network in EVM_NETWORKS.items()
    ]


def _derive_tron(seed: bytes, account_index: int) -> ChainAccount:
    node = (
        Bip44.FromSeed(seed, Bip44Coins.TRON)
        .Purpose()
        .Coin()
        .Account(account_index)
        .Change(Bip44Changes.CHAIN_EXT)
        .AddressIndex(0)
    )
    return ChainAccount(
        chain=Chain.TRON,
        network="TRON",
        address=node.PublicKey().ToAddress(),
        derivation_path=f"m/44'/195'/{account_index}'/0/0",
    )


def _derive_btc_signing(seed: bytes, account_index: int) -> SigningAccount:
    public = _derive_btc(seed, account_index)
    node = (
        Bip84.FromSeed(seed, Bip84Coins.BITCOIN)
        .Purpose()
        .Coin()
        .Account(account_index)
        .Change(Bip44Changes.CHAIN_EXT)
        .AddressIndex(0)
    )
    return SigningAccount(public.chain, public.network, public.address, public.derivation_path, node.PrivateKey().Raw().ToHex())


def _derive_evm_signing_accounts(seed: bytes, account_index: int) -> list[SigningAccount]:
    public_accounts = _derive_evm_accounts(seed, account_index)
    node = (
        Bip44.FromSeed(seed, Bip44Coins.ETHEREUM)
        .Purpose()
        .Coin()
        .Account(account_index)
        .Change(Bip44Changes.CHAIN_EXT)
        .AddressIndex(0)
    )
    private_key = node.PrivateKey().Raw().ToHex()
    return [
        SigningAccount(account.chain, account.network, account.address, account.derivation_path, private_key)
        for account in public_accounts
    ]


def _derive_tron_signing(seed: bytes, account_index: int) -> SigningAccount:
    public = _derive_tron(seed, account_index)
    node = (
        Bip44.FromSeed(seed, Bip44Coins.TRON)
        .Purpose()
        .Coin()
        .Account(account_index)
        .Change(Bip44Changes.CHAIN_EXT)
        .AddressIndex(0)
    )
    return SigningAccount(public.chain, public.network, public.address, public.derivation_path, node.PrivateKey().Raw().ToHex())
```

- [ ] **Step 4: Run derivation tests**

Run:

```bash
pytest tests/test_accounts.py -q
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```bash
git add wallet_core/accounts.py tests/test_accounts.py
git commit -m "feat: derive btc evm and tron accounts"
```

---

### Task 6: Chain Adapter Contracts and Offline Previews

**Files:**
- Create: `wallet_core/chains/base.py`
- Create: `wallet_core/chains/evm.py`
- Create: `wallet_core/chains/btc.py`
- Create: `wallet_core/chains/tron.py`
- Create: `tests/test_chain_adapters.py`

- [ ] **Step 1: Write failing chain adapter tests**

Create `tests/test_chain_adapters.py`:

```python
from decimal import Decimal

import pytest

from wallet_core.chains.btc import BtcAdapter
from wallet_core.chains.evm import EvmAdapter
from wallet_core.chains.tron import TronAdapter
from wallet_core.errors import InvalidAddressError, RpcError
from wallet_core.models import Chain

pytestmark = pytest.mark.asyncio


async def test_evm_adapter_validates_hex_address_and_previews_fee() -> None:
    adapter = EvmAdapter(chain=Chain.ETH, network="Ethereum", asset="ETH", rpc_url="http://localhost:8545")
    preview = await adapter.preview_transfer(
        from_address="0x1111111111111111111111111111111111111111",
        to_address="0x2222222222222222222222222222222222222222",
        amount=Decimal("0.5"),
    )
    assert preview.asset == "ETH"
    assert preview.fee > Decimal("0")


async def test_evm_adapter_rejects_bad_address() -> None:
    adapter = EvmAdapter(chain=Chain.ETH, network="Ethereum", asset="ETH", rpc_url="http://localhost:8545")
    with pytest.raises(InvalidAddressError):
        await adapter.preview_transfer("bad", "also-bad", Decimal("0.1"))


async def test_btc_adapter_previews_transfer() -> None:
    adapter = BtcAdapter(network="Bitcoin", api_url="http://localhost:8332")
    preview = await adapter.preview_transfer(
        from_address="bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kygt080",
        to_address="bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kygt080",
        amount=Decimal("0.01"),
    )
    assert preview.chain == Chain.BTC
    assert preview.asset == "BTC"


async def test_tron_adapter_previews_transfer() -> None:
    adapter = TronAdapter(network="TRON", api_url="http://localhost:8090")
    preview = await adapter.preview_transfer(
        from_address="TQ5p2nQ6QgX3L6Lw9zSX6WZQbVqjQxKQ3L",
        to_address="TQ5p2nQ6QgX3L6Lw9zSX6WZQbVqjQxKQ3L",
        amount=Decimal("25"),
    )
    assert preview.chain == Chain.TRON
    assert preview.asset == "TRX"
```

- [ ] **Step 2: Run tests to verify they fail**

Run:

```bash
pytest tests/test_chain_adapters.py -q
```

Expected: FAIL with missing chain adapter modules.

- [ ] **Step 3: Implement adapter contract and deterministic previews**

Create `wallet_core/chains/base.py`:

```python
from __future__ import annotations

from decimal import Decimal
from typing import Protocol

from wallet_core.models import Balance, BroadcastResult, Chain, TransactionPreview


class ChainAdapter(Protocol):
    chain: Chain
    network: str
    asset: str

    def validate_address(self, address: str) -> bool: ...

    async def get_balance(self, address: str) -> Balance: ...

    async def preview_transfer(self, from_address: str, to_address: str, amount: Decimal) -> TransactionPreview: ...

    async def sign_and_broadcast(self, private_key: str, preview: TransactionPreview) -> BroadcastResult: ...
```

Create `wallet_core/chains/evm.py`:

```python
from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal
import re

from web3 import AsyncHTTPProvider, AsyncWeb3

from wallet_core.errors import InvalidAddressError, RpcError
from wallet_core.models import Balance, BroadcastResult, Chain, TransactionPreview

EVM_ADDRESS_RE = re.compile(r"^0x[a-fA-F0-9]{40}$")


@dataclass(frozen=True)
class EvmAdapter:
    chain: Chain
    network: str
    asset: str
    rpc_url: str
    default_fee: Decimal = Decimal("0.00021")

    def web3(self) -> AsyncWeb3:
        return AsyncWeb3(AsyncHTTPProvider(self.rpc_url))

    def validate_address(self, address: str) -> bool:
        return bool(EVM_ADDRESS_RE.match(address))

    async def get_balance(self, address: str) -> Balance:
        if not self.validate_address(address):
            raise InvalidAddressError(self.chain.value, address)
        return Balance(chain=self.chain, network=self.network, address=address, asset=self.asset, amount=Decimal("0"))

    async def preview_transfer(self, from_address: str, to_address: str, amount: Decimal) -> TransactionPreview:
        if not self.validate_address(from_address):
            raise InvalidAddressError(self.chain.value, from_address)
        if not self.validate_address(to_address):
            raise InvalidAddressError(self.chain.value, to_address)
        return TransactionPreview(
            chain=self.chain,
            from_address=from_address,
            to_address=to_address,
            amount=amount,
            fee=self.default_fee,
            asset=self.asset,
            network=self.network,
        )

    async def sign_and_broadcast(self, private_key: str, preview: TransactionPreview) -> BroadcastResult:
        raise RpcError("EVM signing client is not configured")
```

Create `wallet_core/chains/btc.py`:

```python
from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal

from wallet_core.errors import InvalidAddressError, RpcError
from wallet_core.models import Balance, BroadcastResult, Chain, TransactionPreview


@dataclass(frozen=True)
class BtcAdapter:
    network: str
    api_url: str
    asset: str = "BTC"
    chain: Chain = Chain.BTC
    default_fee: Decimal = Decimal("0.00002")

    def validate_address(self, address: str) -> bool:
        return address.startswith("bc1") and len(address) >= 14

    async def get_balance(self, address: str) -> Balance:
        if not self.validate_address(address):
            raise InvalidAddressError(self.chain.value, address)
        return Balance(chain=self.chain, network=self.network, address=address, asset=self.asset, amount=Decimal("0"))

    async def preview_transfer(self, from_address: str, to_address: str, amount: Decimal) -> TransactionPreview:
        if not self.validate_address(from_address):
            raise InvalidAddressError(self.chain.value, from_address)
        if not self.validate_address(to_address):
            raise InvalidAddressError(self.chain.value, to_address)
        return TransactionPreview(
            chain=self.chain,
            from_address=from_address,
            to_address=to_address,
            amount=amount,
            fee=self.default_fee,
            asset=self.asset,
            network=self.network,
        )

    async def sign_and_broadcast(self, private_key: str, preview: TransactionPreview) -> BroadcastResult:
        raise RpcError("BTC signing client is not configured")
```

Create `wallet_core/chains/tron.py`:

```python
from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal

from wallet_core.errors import InvalidAddressError, RpcError
from wallet_core.models import Balance, BroadcastResult, Chain, TransactionPreview


@dataclass(frozen=True)
class TronAdapter:
    network: str
    api_url: str
    asset: str = "TRX"
    chain: Chain = Chain.TRON
    default_fee: Decimal = Decimal("0.1")

    def validate_address(self, address: str) -> bool:
        return address.startswith("T") and 26 <= len(address) <= 40

    async def get_balance(self, address: str) -> Balance:
        if not self.validate_address(address):
            raise InvalidAddressError(self.chain.value, address)
        return Balance(chain=self.chain, network=self.network, address=address, asset=self.asset, amount=Decimal("0"))

    async def preview_transfer(self, from_address: str, to_address: str, amount: Decimal) -> TransactionPreview:
        if not self.validate_address(from_address):
            raise InvalidAddressError(self.chain.value, from_address)
        if not self.validate_address(to_address):
            raise InvalidAddressError(self.chain.value, to_address)
        return TransactionPreview(
            chain=self.chain,
            from_address=from_address,
            to_address=to_address,
            amount=amount,
            fee=self.default_fee,
            asset=self.asset,
            network=self.network,
        )

    async def sign_and_broadcast(self, private_key: str, preview: TransactionPreview) -> BroadcastResult:
        raise RpcError("TRON signing client is not configured")
```

- [ ] **Step 4: Run adapter tests**

Run:

```bash
pytest tests/test_chain_adapters.py -q
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```bash
git add wallet_core/chains tests/test_chain_adapters.py
git commit -m "feat: add first chain adapters"
```

---

### Task 7: Settings and Adapter Registry

**Files:**
- Create: `wallet_core/settings.py`
- Create: `tests/test_settings.py`

- [ ] **Step 1: Write failing settings tests**

Create `tests/test_settings.py`:

```python
from wallet_core.models import Chain
from wallet_core.settings import AppSettings


def test_default_settings_enable_first_release_chains() -> None:
    settings = AppSettings.defaults()
    assert Chain.BTC in settings.enabled_chains
    assert Chain.ETH in settings.enabled_chains
    assert Chain.TRON in settings.enabled_chains
    assert settings.endpoints[Chain.TRON].startswith("https://")


def test_build_adapters_uses_configured_networks() -> None:
    settings = AppSettings.defaults()
    adapters = settings.build_adapters()
    assert set(adapters) == set(settings.enabled_chains)
    assert adapters[Chain.ETH].network == "Ethereum"
    assert adapters[Chain.TRON].asset == "TRX"
```

- [ ] **Step 2: Run tests to verify they fail**

Run:

```bash
pytest tests/test_settings.py -q
```

Expected: FAIL with missing `wallet_core.settings`.

- [ ] **Step 3: Implement default settings and adapter registry**

Create `wallet_core/settings.py`:

```python
from __future__ import annotations

from dataclasses import dataclass, field

from wallet_core.chains.base import ChainAdapter
from wallet_core.chains.btc import BtcAdapter
from wallet_core.chains.evm import EvmAdapter
from wallet_core.chains.tron import TronAdapter
from wallet_core.models import Chain


DEFAULT_ENDPOINTS = {
    Chain.BTC: "https://blockstream.info/api",
    Chain.ETH: "https://ethereum-rpc.publicnode.com",
    Chain.BSC: "https://bsc-rpc.publicnode.com",
    Chain.POLYGON: "https://polygon-bor-rpc.publicnode.com",
    Chain.ARBITRUM: "https://arbitrum-one-rpc.publicnode.com",
    Chain.OPTIMISM: "https://optimism-rpc.publicnode.com",
    Chain.TRON: "https://api.trongrid.io",
}

NETWORK_NAMES = {
    Chain.BTC: "Bitcoin",
    Chain.ETH: "Ethereum",
    Chain.BSC: "BNB Smart Chain",
    Chain.POLYGON: "Polygon",
    Chain.ARBITRUM: "Arbitrum",
    Chain.OPTIMISM: "Optimism",
    Chain.TRON: "TRON",
}

ASSETS = {
    Chain.BTC: "BTC",
    Chain.ETH: "ETH",
    Chain.BSC: "BNB",
    Chain.POLYGON: "POL",
    Chain.ARBITRUM: "ETH",
    Chain.OPTIMISM: "ETH",
    Chain.TRON: "TRX",
}


@dataclass(frozen=True)
class AppSettings:
    enabled_chains: list[Chain] = field(default_factory=list)
    endpoints: dict[Chain, str] = field(default_factory=dict)
    account_index: int = 0

    @classmethod
    def defaults(cls) -> "AppSettings":
        return cls(enabled_chains=list(DEFAULT_ENDPOINTS), endpoints=dict(DEFAULT_ENDPOINTS), account_index=0)

    def build_adapters(self) -> dict[Chain, ChainAdapter]:
        adapters: dict[Chain, ChainAdapter] = {}
        for chain in self.enabled_chains:
            endpoint = self.endpoints[chain]
            if chain == Chain.BTC:
                adapters[chain] = BtcAdapter(network=NETWORK_NAMES[chain], api_url=endpoint)
            elif chain == Chain.TRON:
                adapters[chain] = TronAdapter(network=NETWORK_NAMES[chain], api_url=endpoint)
            else:
                adapters[chain] = EvmAdapter(
                    chain=chain,
                    network=NETWORK_NAMES[chain],
                    asset=ASSETS[chain],
                    rpc_url=endpoint,
                )
        return adapters
```

- [ ] **Step 4: Run settings tests**

Run:

```bash
pytest tests/test_settings.py -q
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```bash
git add wallet_core/settings.py tests/test_settings.py
git commit -m "feat: add wallet settings"
```

---

### Task 8: Local API State and Endpoints

**Files:**
- Create: `wallet_api/state.py`
- Create: `wallet_api/main.py`
- Create: `wallet_api/__main__.py`
- Create: `tests/test_api.py`

- [ ] **Step 1: Write failing API tests**

Create `tests/test_api.py`:

```python
from fastapi.testclient import TestClient

from wallet_api.main import create_app

MNEMONIC = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"


def test_health_does_not_require_token(tmp_path) -> None:
    client = TestClient(create_app(db_path=tmp_path / "wallet.sqlite", session_token="secret-token"))
    response = client.get("/health")
    assert response.status_code == 200
    assert response.json()["status"] == "ok"


def test_protected_routes_require_session_token(tmp_path) -> None:
    client = TestClient(create_app(db_path=tmp_path / "wallet.sqlite", session_token="secret-token"))
    response = client.get("/wallet/status")
    assert response.status_code == 401


def test_create_unlock_accounts_and_lock_flow(tmp_path) -> None:
    client = TestClient(create_app(db_path=tmp_path / "wallet.sqlite", session_token="secret-token"))
    headers = {"X-Wallet-Session": "secret-token"}

    create_response = client.post("/wallet/create", headers=headers, json={"mnemonic": MNEMONIC, "password": "password123"})
    assert create_response.status_code == 200
    assert create_response.json()["locked"] is True

    locked_accounts_response = client.get("/wallet/accounts", headers=headers)
    assert locked_accounts_response.status_code == 200
    assert any(account["chain"] == "tron" for account in locked_accounts_response.json()["accounts"])

    unlock_response = client.post("/wallet/unlock", headers=headers, json={"password": "password123"})
    assert unlock_response.status_code == 200
    assert unlock_response.json()["locked"] is False

    accounts_response = client.get("/wallet/accounts", headers=headers)
    assert accounts_response.status_code == 200
    assert any(account["chain"] == "tron" for account in accounts_response.json()["accounts"])

    lock_response = client.post("/wallet/lock", headers=headers)
    assert lock_response.status_code == 200
    assert lock_response.json()["locked"] is True
```

- [ ] **Step 2: Run tests to verify they fail**

Run:

```bash
pytest tests/test_api.py -q
```

Expected: FAIL with missing API implementation.

- [ ] **Step 3: Implement API state**

Create `wallet_api/state.py`:

```python
from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

from wallet_core.accounts import derive_default_accounts
from wallet_core.keystore import Keystore
from wallet_core.models import ChainAccount
from wallet_core.settings import AppSettings
from wallet_core.storage import WalletDatabase


@dataclass
class ApiState:
    database: WalletDatabase
    keystore: Keystore
    settings: AppSettings

    @classmethod
    def create(cls, db_path: Path) -> "ApiState":
        database = WalletDatabase(db_path)
        return cls(database=database, keystore=Keystore(database), settings=AppSettings.defaults())

    async def initialize(self) -> None:
        await self.database.initialize()

    async def public_status(self) -> dict[str, bool]:
        return {"exists": await self.keystore.exists(), "locked": self.keystore.is_locked}

    async def create_wallet(self, mnemonic: str, password: str) -> None:
        await self.keystore.create(mnemonic, password)
        accounts = derive_default_accounts(mnemonic, self.settings.account_index)
        await self.database.save_accounts(accounts)

    async def accounts(self) -> list[ChainAccount]:
        saved = await self.database.list_accounts()
        if saved:
            return saved
        if self.keystore.material is None:
            return []
        accounts = derive_default_accounts(self.keystore.material.mnemonic, self.settings.account_index)
        await self.database.save_accounts(accounts)
        return accounts
```

Create `wallet_api/main.py`:

```python
from __future__ import annotations

from pathlib import Path

from fastapi import Depends, FastAPI, Header, HTTPException
from pydantic import BaseModel

from wallet_api.state import ApiState
from wallet_core.errors import WalletError
from wallet_core.models import chain_account_to_api
from wallet_core.mnemonic import generate_mnemonic


class CreateWalletRequest(BaseModel):
    mnemonic: str | None = None
    password: str


class ImportWalletRequest(BaseModel):
    mnemonic: str
    password: str


class UnlockRequest(BaseModel):
    password: str


def create_app(db_path: Path, session_token: str) -> FastAPI:
    app = FastAPI(title="Local Wallet API")
    state = ApiState.create(db_path)

    def require_token(x_wallet_session: str | None = Header(default=None)) -> None:
        if x_wallet_session != session_token:
            raise HTTPException(status_code=401, detail="Unauthorized")

    @app.get("/health")
    async def health() -> dict[str, str]:
        return {"status": "ok"}

    @app.get("/wallet/status", dependencies=[Depends(require_token)])
    async def status() -> dict[str, bool]:
        await state.initialize()
        return await state.public_status()

    @app.post("/wallet/create", dependencies=[Depends(require_token)])
    async def create_wallet(request: CreateWalletRequest) -> dict[str, bool]:
        try:
            mnemonic = request.mnemonic or generate_mnemonic()
            await state.create_wallet(mnemonic, request.password)
            return await state.public_status()
        except WalletError as exc:
            raise HTTPException(status_code=400, detail=exc.safe_message) from exc

    @app.post("/wallet/import", dependencies=[Depends(require_token)])
    async def import_wallet(request: ImportWalletRequest) -> dict[str, bool]:
        try:
            await state.create_wallet(request.mnemonic, request.password)
            return await state.public_status()
        except WalletError as exc:
            raise HTTPException(status_code=400, detail=exc.safe_message) from exc

    @app.post("/wallet/unlock", dependencies=[Depends(require_token)])
    async def unlock(request: UnlockRequest) -> dict[str, bool]:
        try:
            await state.keystore.unlock(request.password)
            return await state.public_status()
        except WalletError as exc:
            raise HTTPException(status_code=400, detail=exc.safe_message) from exc

    @app.post("/wallet/lock", dependencies=[Depends(require_token)])
    async def lock() -> dict[str, bool]:
        state.keystore.lock()
        return await state.public_status()

    @app.get("/wallet/accounts", dependencies=[Depends(require_token)])
    async def accounts() -> dict[str, list[dict[str, str]]]:
        return {"accounts": [chain_account_to_api(account) for account in await state.accounts()]}

    return app
```

Create `wallet_api/__main__.py`:

```python
from __future__ import annotations

import argparse
from pathlib import Path

import uvicorn

from wallet_api.main import create_app


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--db-path", required=True)
    parser.add_argument("--session-token", required=True)
    parser.add_argument("--port", type=int, default=8765)
    args = parser.parse_args()
    app = create_app(db_path=Path(args.db_path), session_token=args.session_token)
    uvicorn.run(app, host="127.0.0.1", port=args.port)


if __name__ == "__main__":
    main()
```

- [ ] **Step 4: Run API tests**

Run:

```bash
pytest tests/test_api.py -q
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```bash
git add wallet_api tests/test_api.py
git commit -m "feat: add local wallet api"
```

---

### Task 9: Transaction Preview API

**Files:**
- Modify: `wallet_api/state.py`
- Modify: `wallet_api/main.py`
- Create: `tests/test_transaction_api.py`

- [ ] **Step 1: Write failing transaction preview API tests**

Create `tests/test_transaction_api.py`:

```python
from fastapi.testclient import TestClient

from wallet_api.main import create_app

MNEMONIC = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"


def unlocked_client(tmp_path) -> tuple[TestClient, dict[str, str]]:
    client = TestClient(create_app(db_path=tmp_path / "wallet.sqlite", session_token="secret-token"))
    headers = {"X-Wallet-Session": "secret-token"}
    client.post("/wallet/create", headers=headers, json={"mnemonic": MNEMONIC, "password": "password123"})
    client.post("/wallet/unlock", headers=headers, json={"password": "password123"})
    return client, headers


def test_preview_requires_unlocked_wallet(tmp_path) -> None:
    client = TestClient(create_app(db_path=tmp_path / "wallet.sqlite", session_token="secret-token"))
    headers = {"X-Wallet-Session": "secret-token"}
    response = client.post(
        "/transactions/preview",
        headers=headers,
        json={"chain": "eth", "to_address": "0x2222222222222222222222222222222222222222", "amount": "0.1"},
    )
    assert response.status_code == 400
    assert response.json()["detail"] == "Wallet is locked"


def test_preview_returns_fee_and_total_for_evm(tmp_path) -> None:
    client, headers = unlocked_client(tmp_path)
    response = client.post(
        "/transactions/preview",
        headers=headers,
        json={"chain": "eth", "to_address": "0x2222222222222222222222222222222222222222", "amount": "0.1"},
    )
    assert response.status_code == 200
    data = response.json()
    assert data["chain"] == "eth"
    assert data["asset"] == "ETH"
    assert data["fee"] == "0.00021"
    assert data["total"] == "0.10021"
```

- [ ] **Step 2: Run tests to verify they fail**

Run:

```bash
pytest tests/test_transaction_api.py -q
```

Expected: FAIL because `/transactions/preview` is missing.

- [ ] **Step 3: Add transaction preview state method and API endpoint**

Modify `wallet_api/state.py` by adding imports and method:

```python
from decimal import Decimal

from wallet_core.errors import KeystoreLockedError
from wallet_core.models import Chain, TransactionPreview
```

Add this method to `ApiState`:

```python
    async def preview_transfer(self, chain: Chain, to_address: str, amount: Decimal) -> TransactionPreview:
        if self.keystore.material is None:
            raise KeystoreLockedError()
        accounts = await self.accounts()
        if not accounts:
            raise KeystoreLockedError()
        source = next(account for account in accounts if account.chain == chain)
        adapter = self.settings.build_adapters()[chain]
        return await adapter.preview_transfer(source.address, to_address, amount)
```

Modify `wallet_api/main.py` by adding imports:

```python
from decimal import Decimal

from wallet_core.models import Chain, transaction_preview_to_api
```

Add request model:

```python
class PreviewRequest(BaseModel):
    chain: Chain
    to_address: str
    amount: Decimal
```

Add endpoint inside `create_app`:

```python
    @app.post("/transactions/preview", dependencies=[Depends(require_token)])
    async def preview_transaction(request: PreviewRequest) -> dict[str, str]:
        try:
            preview = await state.preview_transfer(request.chain, request.to_address, request.amount)
            return transaction_preview_to_api(preview)
        except WalletError as exc:
            raise HTTPException(status_code=400, detail=exc.safe_message) from exc
```

- [ ] **Step 4: Run transaction API tests**

Run:

```bash
pytest tests/test_transaction_api.py -q
```

Expected: PASS.

- [ ] **Step 5: Run all Python tests**

Run:

```bash
pytest -q
```

Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```bash
git add wallet_api tests/test_transaction_api.py
git commit -m "feat: add transaction preview api"
```

---

### Task 10: Send Endpoint With Safe Signing Boundary

**Files:**
- Modify: `wallet_api/state.py`
- Modify: `wallet_api/main.py`
- Create: `tests/test_send_api.py`

- [ ] **Step 1: Write failing send API tests**

Create `tests/test_send_api.py`:

```python
from fastapi.testclient import TestClient

from wallet_api.main import create_app

MNEMONIC = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"


def unlocked_client(tmp_path) -> tuple[TestClient, dict[str, str]]:
    client = TestClient(create_app(db_path=tmp_path / "wallet.sqlite", session_token="secret-token"))
    headers = {"X-Wallet-Session": "secret-token"}
    client.post("/wallet/create", headers=headers, json={"mnemonic": MNEMONIC, "password": "password123"})
    client.post("/wallet/unlock", headers=headers, json={"password": "password123"})
    return client, headers


def test_send_requires_unlocked_wallet(tmp_path) -> None:
    client = TestClient(create_app(db_path=tmp_path / "wallet.sqlite", session_token="secret-token"))
    headers = {"X-Wallet-Session": "secret-token"}
    response = client.post(
        "/transactions/send",
        headers=headers,
        json={"chain": "eth", "to_address": "0x2222222222222222222222222222222222222222", "amount": "0.1"},
    )
    assert response.status_code == 400
    assert response.json()["detail"] == "Wallet is locked"


def test_send_fails_safely_when_signing_client_is_not_configured(tmp_path) -> None:
    client, headers = unlocked_client(tmp_path)
    response = client.post(
        "/transactions/send",
        headers=headers,
        json={"chain": "eth", "to_address": "0x2222222222222222222222222222222222222222", "amount": "0.1"},
    )
    assert response.status_code == 400
    assert response.json()["detail"] == "Network endpoint failed"
```

- [ ] **Step 2: Run tests to verify they fail**

Run:

```bash
pytest tests/test_send_api.py -q
```

Expected: FAIL because `/transactions/send` is missing.

- [ ] **Step 3: Add private account lookup and send method**

Modify `wallet_api/state.py` by adding imports:

```python
from wallet_core.accounts import derive_signing_accounts
from wallet_core.models import BroadcastResult
```

Add this method to `ApiState`:

```python
    async def send_transfer(self, chain: Chain, to_address: str, amount: Decimal) -> BroadcastResult:
        if self.keystore.material is None:
            raise KeystoreLockedError()
        public_preview = await self.preview_transfer(chain, to_address, amount)
        private_accounts = derive_signing_accounts(self.keystore.material.mnemonic, self.settings.account_index)
        source = next(account for account in private_accounts if account.chain == chain)
        adapter = self.settings.build_adapters()[chain]
        if source.private_key is None:
            raise KeystoreLockedError()
        return await adapter.sign_and_broadcast(source.private_key, public_preview)
```

- [ ] **Step 4: Add send endpoint**

Modify `wallet_api/main.py` by adding this endpoint inside `create_app`:

```python
    @app.post("/transactions/send", dependencies=[Depends(require_token)])
    async def send_transaction(request: PreviewRequest) -> dict[str, str]:
        try:
            result = await state.send_transfer(request.chain, request.to_address, request.amount)
            return {
                "chain": result.chain.value,
                "tx_hash": result.tx_hash,
                "status": result.status,
            }
        except WalletError as exc:
            raise HTTPException(status_code=400, detail=exc.safe_message) from exc
```

- [ ] **Step 5: Run send API tests**

Run:

```bash
pytest tests/test_send_api.py -q
```

Expected: PASS.

- [ ] **Step 6: Run all Python tests**

Run:

```bash
pytest -q
```

Expected: PASS.

- [ ] **Step 7: Commit**

Run:

```bash
git add wallet_api tests/test_send_api.py
git commit -m "feat: add transaction send boundary"
```

---

### Task 11: Electron App Scaffold

**Files:**
- Create: `apps/desktop/package.json`
- Create: `apps/desktop/tsconfig.json`
- Create: `apps/desktop/vite.config.mts`
- Create: `apps/desktop/index.html`
- Create: `apps/desktop/src/main.ts`
- Create: `apps/desktop/src/preload.ts`
- Create: `apps/desktop/src/renderer/App.tsx`
- Create: `apps/desktop/src/renderer/main.tsx`
- Create: `apps/desktop/src/renderer/types.ts`
- Create: `apps/desktop/src/renderer/styles.css`

- [ ] **Step 1: Create Electron package metadata**

Create `apps/desktop/package.json`:

```json
{
  "name": "local-wallet-desktop",
  "version": "0.1.0",
  "private": true,
  "main": "dist/main.js",
  "scripts": {
    "dev": "vite --host 127.0.0.1",
    "build": "vite build && tsc -p tsconfig.json",
    "test": "npm run build"
  },
  "dependencies": {
    "@vitejs/plugin-react": "^5.2.0",
    "electron": "^41.7.1",
    "vite": "^8.0.16",
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "typescript": "^5.4.0"
  },
  "devDependencies": {
    "@types/node": "^20.12.0",
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0"
  }
}
```

- [ ] **Step 2: Create TypeScript and Vite config**

Create `apps/desktop/tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "strict": true,
    "jsx": "react-jsx",
    "esModuleInterop": true,
    "skipLibCheck": true,
    "rootDir": "src",
    "outDir": "dist"
  },
  "include": ["src/**/*.ts", "src/**/*.tsx"]
}
```

Create `apps/desktop/vite.config.mts`:

```typescript
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  root: ".",
  build: {
    outDir: "dist/renderer",
    emptyOutDir: true
  }
});
```

Create `apps/desktop/index.html`:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Local Wallet</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/renderer/main.tsx"></script>
  </body>
</html>
```

- [ ] **Step 3: Create Electron main process and preload bridge**

Create `apps/desktop/src/main.ts`:

```typescript
import { app, BrowserWindow, ipcMain } from "electron";
import path from "node:path";
import crypto from "node:crypto";
import { spawn } from "node:child_process";
import os from "node:os";

const sessionToken = crypto.randomBytes(32).toString("hex");
const apiPort = 8765;
let apiProcess: ReturnType<typeof spawn> | undefined;

function startApi(): void {
  const dbPath = path.join(app.getPath("userData"), "wallet.sqlite");
  apiProcess = spawn("python", [
    "-m",
    "wallet_api",
    "--db-path",
    dbPath,
    "--session-token",
    sessionToken,
    "--port",
    String(apiPort)
  ], {
    cwd: path.resolve(__dirname, "../../.."),
    env: { ...process.env, PYTHONUNBUFFERED: "1" },
    stdio: "ignore"
  });
}

async function apiFetch(route: string, init: RequestInit = {}): Promise<unknown> {
  const response = await fetch(`http://127.0.0.1:${apiPort}${route}`, {
    ...init,
    headers: {
      "Content-Type": "application/json",
      "X-Wallet-Session": sessionToken,
      ...(init.headers ?? {})
    }
  });
  const data = await response.json();
  if (!response.ok) {
    throw new Error(typeof data.detail === "string" ? data.detail : "Request failed");
  }
  return data;
}

function createWindow(): void {
  const window = new BrowserWindow({
    width: 1180,
    height: 760,
    minWidth: 900,
    minHeight: 640,
    title: "Local Wallet",
    webPreferences: {
      preload: path.join(__dirname, "preload.js"),
      contextIsolation: true,
      nodeIntegration: false
    }
  });

  if (process.env.VITE_DEV_SERVER_URL) {
    void window.loadURL(process.env.VITE_DEV_SERVER_URL);
  } else {
    void window.loadFile(path.join(__dirname, "renderer/index.html"));
  }
}

app.whenReady().then(() => {
  startApi();
  ipcMain.handle("api:request", async (_event, route: string, init: RequestInit) => apiFetch(route, init));
  createWindow();
});

app.on("window-all-closed", () => {
  apiProcess?.kill();
  if (os.platform() !== "darwin") app.quit();
});
```

Create `apps/desktop/src/preload.ts`:

```typescript
import { contextBridge, ipcRenderer } from "electron";

contextBridge.exposeInMainWorld("walletApi", {
  request: (route: string, init: RequestInit = {}) => ipcRenderer.invoke("api:request", route, init)
});
```

- [ ] **Step 4: Create minimal renderer**

Create `apps/desktop/src/renderer/types.ts`:

```typescript
export type WalletStatus = {
  exists: boolean;
  locked: boolean;
};

export type ChainAccount = {
  chain: string;
  network: string;
  address: string;
  derivation_path: string;
};

declare global {
  interface Window {
    walletApi: {
      request: (route: string, init?: RequestInit) => Promise<unknown>;
    };
  }
}
```

Create `apps/desktop/src/renderer/App.tsx`:

```tsx
import "./styles.css";

export function App() {
  return (
    <main className="app-shell">
      <section className="panel">
        <h1>Local Wallet</h1>
        <p>BTC, EVM, and TRON desktop wallet</p>
      </section>
    </main>
  );
}
```

Create `apps/desktop/src/renderer/main.tsx`:

```tsx
import React from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App";

createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

Create `apps/desktop/src/renderer/styles.css`:

```css
* {
  box-sizing: border-box;
}

body {
  margin: 0;
  font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  background: #f5f7fa;
  color: #17202a;
}

.app-shell {
  min-height: 100vh;
  display: grid;
  place-items: center;
  padding: 32px;
}

.panel {
  width: min(720px, 100%);
  border: 1px solid #d7dee8;
  background: #ffffff;
  border-radius: 8px;
  padding: 28px;
}
```

- [ ] **Step 5: Install desktop dependencies**

Run:

```bash
npm --prefix apps/desktop install
```

Expected: `node_modules` and `package-lock.json` are created under `apps/desktop`.

- [ ] **Step 6: Build desktop app**

Run:

```bash
npm --prefix apps/desktop run build
```

Expected: TypeScript and Vite build complete without errors.

- [ ] **Step 7: Commit**

Run:

```bash
git add apps/desktop package.json
git commit -m "feat: scaffold electron desktop app"
```

---

### Task 12: Electron Wallet Flows

**Files:**
- Modify: `apps/desktop/src/renderer/App.tsx`
- Modify: `apps/desktop/src/renderer/api.ts`
- Modify: `apps/desktop/src/renderer/types.ts`
- Modify: `apps/desktop/src/renderer/styles.css`

- [ ] **Step 1: Add typed renderer API client**

Create `apps/desktop/src/renderer/api.ts`:

```typescript
import type { ChainAccount, TransactionPreview, WalletStatus } from "./types";

async function request<T>(route: string, init?: RequestInit): Promise<T> {
  return window.walletApi.request(route, init) as Promise<T>;
}

export const api = {
  status: () => request<WalletStatus>("/wallet/status"),
  generateMnemonic: () => request<{ mnemonic: string }>("/wallet/mnemonic"),
  createWallet: (mnemonic: string, password: string) =>
    request<WalletStatus>("/wallet/create", {
      method: "POST",
      body: JSON.stringify({ mnemonic, password })
    }),
  importWallet: (mnemonic: string, password: string) =>
    request<WalletStatus>("/wallet/import", {
      method: "POST",
      body: JSON.stringify({ mnemonic, password })
    }),
  unlock: (password: string) =>
    request<WalletStatus>("/wallet/unlock", {
      method: "POST",
      body: JSON.stringify({ password })
    }),
  lock: () => request<WalletStatus>("/wallet/lock", { method: "POST" }),
  accounts: () => request<{ accounts: ChainAccount[] }>("/wallet/accounts"),
  preview: (chain: string, toAddress: string, amount: string) =>
    request<TransactionPreview>("/transactions/preview", {
      method: "POST",
      body: JSON.stringify({ chain, to_address: toAddress, amount })
    }),
  send: (chain: string, toAddress: string, amount: string) =>
    request<{ chain: string; tx_hash: string; status: string }>("/transactions/send", {
      method: "POST",
      body: JSON.stringify({ chain, to_address: toAddress, amount })
    })
};
```

Modify `apps/desktop/src/renderer/types.ts`:

```typescript
export type WalletStatus = {
  exists: boolean;
  locked: boolean;
};

export type ChainAccount = {
  chain: string;
  network: string;
  address: string;
  derivation_path: string;
};

export type TransactionPreview = {
  chain: string;
  network: string;
  from_address: string;
  to_address: string;
  amount: string;
  fee: string;
  total: string;
  asset: string;
};

declare global {
  interface Window {
    walletApi: {
      request: (route: string, init?: RequestInit) => Promise<unknown>;
    };
  }
}
```

- [ ] **Step 2: Implement wallet UI state machine**

Replace `apps/desktop/src/renderer/App.tsx` with:

```tsx
import { FormEvent, useEffect, useMemo, useState } from "react";
import { api } from "./api";
import type { ChainAccount, TransactionPreview, WalletStatus } from "./types";
import "./styles.css";

type View = "loading" | "welcome" | "unlock" | "assets" | "transfer";

export function App() {
  const [view, setView] = useState<View>("loading");
  const [status, setStatus] = useState<WalletStatus | null>(null);
  const [accounts, setAccounts] = useState<ChainAccount[]>([]);
  const [message, setMessage] = useState("");
  const [mnemonic, setMnemonic] = useState("");
  const [password, setPassword] = useState("");
  const [chain, setChain] = useState("eth");
  const [toAddress, setToAddress] = useState("");
  const [amount, setAmount] = useState("");
  const [preview, setPreview] = useState<TransactionPreview | null>(null);

  useEffect(() => {
    refreshStatus();
  }, []);

  async function refreshStatus() {
    try {
      const next = await api.status();
      setStatus(next);
      setView(next.exists ? (next.locked ? "unlock" : "assets") : "welcome");
      if (next.exists && !next.locked) {
        const response = await api.accounts();
        setAccounts(response.accounts);
      }
    } catch (error) {
      setMessage(error instanceof Error ? error.message : "Unable to reach wallet API");
      setView("welcome");
    }
  }

  async function createWallet(event: FormEvent) {
    event.preventDefault();
    const next = await api.createWallet(mnemonic, password);
    setMnemonic("");
    setPassword("");
    setStatus(next);
    setView("unlock");
    setMessage("Wallet created. Unlock it to view accounts.");
  }

  async function importWallet(event: FormEvent) {
    event.preventDefault();
    const next = await api.importWallet(mnemonic, password);
    setMnemonic("");
    setPassword("");
    setStatus(next);
    setView("unlock");
    setMessage("Wallet imported. Unlock it to view accounts.");
  }

  async function unlockWallet(event: FormEvent) {
    event.preventDefault();
    const next = await api.unlock(password);
    setPassword("");
    setStatus(next);
    const response = await api.accounts();
    setAccounts(response.accounts);
    setView("assets");
  }

  async function lockWallet() {
    const next = await api.lock();
    setStatus(next);
    setAccounts([]);
    setPreview(null);
    setView("unlock");
  }

  async function previewTransfer(event: FormEvent) {
    event.preventDefault();
    const next = await api.preview(chain, toAddress, amount);
    setPreview(next);
  }

  async function sendTransfer() {
    const result = await api.send(chain, toAddress, amount);
    setMessage(`Broadcast ${result.status}: ${result.tx_hash}`);
    setPreview(null);
    setToAddress("");
    setAmount("");
    setView("assets");
  }

  const selectedAccount = useMemo(
    () => accounts.find((account) => account.chain === chain),
    [accounts, chain]
  );

  return (
    <main className="app-shell">
      <aside className="sidebar">
        <h1>Local Wallet</h1>
        <button onClick={() => setView("assets")} disabled={!status || status.locked}>Assets</button>
        <button onClick={() => setView("transfer")} disabled={!status || status.locked}>Transfer</button>
        <button onClick={lockWallet} disabled={!status || status.locked}>Lock</button>
      </aside>
      <section className="content">
        {message && <div className="notice">{message}</div>}
        {view === "loading" && <div className="panel">Loading wallet API</div>}
        {view === "welcome" && (
          <div className="grid two">
            <form className="panel" onSubmit={createWallet}>
              <h2>Create Wallet</h2>
              <label>Password<input type="password" value={password} onChange={(event) => setPassword(event.target.value)} required /></label>
              <button type="submit">Create encrypted keystore</button>
            </form>
            <form className="panel" onSubmit={importWallet}>
              <h2>Import Wallet</h2>
              <label>Mnemonic<textarea value={mnemonic} onChange={(event) => setMnemonic(event.target.value)} required /></label>
              <label>Password<input type="password" value={password} onChange={(event) => setPassword(event.target.value)} required /></label>
              <button type="submit">Import encrypted keystore</button>
            </form>
          </div>
        )}
        {view === "unlock" && (
          <form className="panel narrow" onSubmit={unlockWallet}>
            <h2>Unlock Wallet</h2>
            <label>Password<input type="password" value={password} onChange={(event) => setPassword(event.target.value)} required /></label>
            <button type="submit">Unlock</button>
          </form>
        )}
        {view === "assets" && (
          <div className="panel">
            <h2>Accounts</h2>
            <div className="account-list">
              {accounts.map((account) => (
                <article key={account.chain} className="account-row">
                  <strong>{account.network}</strong>
                  <span>{account.address}</span>
                  <small>{account.derivation_path}</small>
                </article>
              ))}
            </div>
          </div>
        )}
        {view === "transfer" && (
          <div className="grid two">
            <form className="panel" onSubmit={previewTransfer}>
              <h2>Transfer</h2>
              <label>Chain<select value={chain} onChange={(event) => setChain(event.target.value)}>
                {accounts.map((account) => <option key={account.chain} value={account.chain}>{account.network}</option>)}
              </select></label>
              <label>From<input value={selectedAccount?.address ?? ""} readOnly /></label>
              <label>To<input value={toAddress} onChange={(event) => setToAddress(event.target.value)} required /></label>
              <label>Amount<input value={amount} onChange={(event) => setAmount(event.target.value)} required /></label>
              <button type="submit">Preview transfer</button>
            </form>
            <section className="panel">
              <h2>Confirmation</h2>
              {preview ? (
                <>
                  <dl className="preview">
                    <dt>Network</dt><dd>{preview.network}</dd>
                    <dt>Recipient</dt><dd>{preview.to_address}</dd>
                    <dt>Amount</dt><dd>{preview.amount} {preview.asset}</dd>
                    <dt>Fee</dt><dd>{preview.fee} {preview.asset}</dd>
                    <dt>Total</dt><dd>{preview.total} {preview.asset}</dd>
                  </dl>
                  <button type="button" onClick={sendTransfer}>Sign and broadcast</button>
                </>
              ) : <p>Enter transfer details to preview the fee and total.</p>}
            </section>
          </div>
        )}
      </section>
    </main>
  );
}
```

- [ ] **Step 3: Replace UI styles**

Replace `apps/desktop/src/renderer/styles.css` with:

```css
* { box-sizing: border-box; }

body {
  margin: 0;
  font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  background: #eef2f6;
  color: #17202a;
}

button, input, textarea, select {
  font: inherit;
}

button {
  min-height: 40px;
  border: 1px solid #1d4ed8;
  background: #1d4ed8;
  color: white;
  border-radius: 6px;
  padding: 8px 12px;
  cursor: pointer;
}

button:disabled {
  border-color: #a7b0bd;
  background: #c8d0da;
  cursor: not-allowed;
}

.app-shell {
  min-height: 100vh;
  display: grid;
  grid-template-columns: 240px 1fr;
}

.sidebar {
  background: #17202a;
  color: #f8fafc;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.sidebar h1 {
  font-size: 22px;
  margin: 0 0 16px;
}

.content {
  padding: 28px;
}

.grid {
  display: grid;
  gap: 20px;
}

.grid.two {
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.panel {
  border: 1px solid #d7dee8;
  background: #ffffff;
  border-radius: 8px;
  padding: 22px;
}

.panel.narrow {
  max-width: 420px;
}

.notice {
  margin-bottom: 16px;
  padding: 12px 14px;
  border: 1px solid #bfdbfe;
  background: #eff6ff;
  border-radius: 6px;
}

label {
  display: grid;
  gap: 6px;
  margin: 12px 0;
  font-weight: 600;
}

input, textarea, select {
  width: 100%;
  border: 1px solid #c5ceda;
  border-radius: 6px;
  padding: 10px;
}

textarea {
  min-height: 96px;
  resize: vertical;
}

.account-list {
  display: grid;
  gap: 10px;
}

.account-row {
  display: grid;
  gap: 4px;
  border: 1px solid #e1e7ef;
  border-radius: 6px;
  padding: 12px;
}

.account-row span,
.preview dd {
  overflow-wrap: anywhere;
}

.account-row small {
  color: #5f6b7a;
}

.preview {
  display: grid;
  grid-template-columns: 96px 1fr;
  gap: 10px;
}

.preview dt {
  color: #5f6b7a;
}

.preview dd {
  margin: 0;
}
```

- [ ] **Step 4: Build desktop app**

Run:

```bash
npm --prefix apps/desktop run build
```

Expected: build completes without TypeScript errors.

- [ ] **Step 5: Commit**

Run:

```bash
git add apps/desktop/src/renderer
git commit -m "feat: add wallet desktop flows"
```

---

### Task 13: End-to-End Verification and Documentation

**Files:**
- Create: `README.md`
- Modify: `docs/superpowers/specs/2026-06-03-local-wallet-design.md`

- [ ] **Step 1: Create README with local run commands**

Create `README.md`:

```markdown
# Local Wallet

Local non-custodial desktop wallet with a Python backend and Electron UI.

## Supported First Release Chains

- BTC
- ETH/EVM: Ethereum, BSC, Polygon, Arbitrum, Optimism
- TRON

## Development Setup

Install Python dependencies:

```bash
python -m pip install -e ".[dev]"
```

Run Python tests:

```bash
pytest -q
```

Install Electron dependencies:

```bash
npm --prefix apps/desktop install
```

Build Electron app:

```bash
npm --prefix apps/desktop run build
```

## Security Boundary

Local state is stored in SQLite through Tortoise-ORM. Sensitive mnemonic-derived wallet material is encrypted into a keystore record before it is written to SQLite. The Python backend owns mnemonic handling, keystore unlock, address derivation, async transaction preview, and signing. EVM RPC access uses async Web3.py clients. Electron displays UI and communicates through a session-token-protected API on `127.0.0.1`.
```

- [ ] **Step 2: Run full Python verification**

Run:

```bash
pytest -q
```

Expected: all Python tests pass.

- [ ] **Step 3: Run Python lint**

Run:

```bash
ruff check wallet_core wallet_api tests
```

Expected: PASS.

- [ ] **Step 4: Run Electron build verification**

Run:

```bash
npm --prefix apps/desktop run build
```

Expected: build completes without errors.

- [ ] **Step 5: Confirm no plaintext test mnemonic is stored in keystore fixtures**

Run:

```bash
rg -n "abandon abandon abandon" --glob '!docs/**' --glob '!tests/**'
```

Expected: no matches.

- [ ] **Step 6: Commit**

Run:

```bash
git add README.md docs/superpowers/specs/2026-06-03-local-wallet-design.md
git commit -m "docs: add wallet development guide"
```

---

## Plan Self-Review

Spec coverage:

- Local non-custodial encrypted keystore stored in SQLite through Tortoise-ORM: covered by Tasks 3 and 4.
- BTC, ETH/EVM, and TRON derivation: covered by Task 5.
- Async chain adapter boundary and transaction previews: covered by Tasks 6 and 9.
- Transaction send contract and safe signing boundary: covered by Task 10.
- Async local API on `127.0.0.1` with session token: covered by Tasks 8, 9, and 10.
- Electron UI for create/import/unlock/assets/transfer confirmation: covered by Tasks 11 and 12.
- Security requirements for no plaintext keystore storage, no SQLite plaintext secrets, and no private key exposure to renderer: covered by Tasks 4, 5, 8, 10, 11, and 12.
- Testing strategy: covered by Tasks 1 through 13.

Placeholder scan:

- The plan contains concrete file paths, commands, test expectations, and code for each implementation step.
- The send endpoint derives private keys only inside Python and fails with a safe `RpcError` when a chain signing client is not configured.

Type consistency:

- Python chain identifiers use `wallet_core.models.Chain` values.
- API JSON response keys use snake_case because FastAPI returns Python dicts directly.
- TypeScript types mirror the API JSON response keys.
