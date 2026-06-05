# On-Chain Multisig Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add first-version on-chain multisig account management and proposal signing for EVM Safe and TRON permission multisig.

**Architecture:** Rust core owns validation, SQLite persistence, and chain-specific proposal payload construction. FFI exposes command JSON. Flutter adds models, API methods, and a Multisig tab for import, proposal creation, signing progress, and ready-state display.

**Tech Stack:** Rust, rusqlite, serde, Flutter/Dart, existing wallet FFI command bridge.

---

### Task 1: Rust Models And Storage

**Files:**
- Modify: `crates/wallet_core/src/models.rs`
- Modify: `crates/wallet_core/src/storage.rs`

- [ ] Write failing Rust tests for importing a multisig account, listing it, creating a proposal, adding signatures, and threshold-ready state.
- [ ] Add `MultisigKind`, `MultisigAccount`, `MultisigOwner`, `MultisigProposal`, `MultisigSignature`, and request structs.
- [ ] Add SQLite tables and migration helpers.
- [ ] Implement storage methods and mapping helpers.
- [ ] Run `cargo test -p wallet_core multisig`.

### Task 2: Rust Service

**Files:**
- Modify: `crates/wallet_core/src/service.rs`
- Modify: `crates/wallet_core/src/error.rs`

- [ ] Write failing service tests for valid EVM Safe import, valid TRON permission import, invalid thresholds, duplicate owners, proposal payload construction, and duplicate signatures.
- [ ] Implement service validation and proposal/signature methods.
- [ ] Add safe user-facing errors.
- [ ] Run `cargo test -p wallet_core multisig`.

### Task 3: FFI Commands

**Files:**
- Modify: `crates/wallet_ffi/src/lib.rs`

- [ ] Write failing FFI tests for `import_multisig_account`, `list_multisig_accounts`, `create_multisig_proposal`, `add_multisig_signature`, and `list_multisig_proposals`.
- [ ] Add command variants and response serialization.
- [ ] Run `cargo test -p wallet_ffi multisig`.

### Task 4: Flutter API And Demo

**Files:**
- Modify: `apps/flutter_wallet/lib/src/models.dart`
- Modify: `apps/flutter_wallet/lib/src/wallet_api.dart`
- Modify: `apps/flutter_wallet/lib/src/native_wallet_api.dart`
- Modify: `apps/flutter_wallet/test/native_wallet_api_test.dart`

- [ ] Write failing Dart API tests for multisig command mapping.
- [ ] Add Dart models and API methods.
- [ ] Implement native parsing and demo API state.
- [ ] Run `flutter test test/native_wallet_api_test.dart`.

### Task 5: Flutter UI

**Files:**
- Create: `apps/flutter_wallet/lib/src/screens/multisig_screen.dart`
- Modify: `apps/flutter_wallet/lib/main.dart`
- Modify: `apps/flutter_wallet/test/wallet_workflow_test.dart`

- [ ] Write failing widget test for importing a multisig account and creating a proposal.
- [ ] Add Multisig tab and dialogs.
- [ ] Wire proposal signing progress and ready state.
- [ ] Run `flutter test test/wallet_workflow_test.dart`.

### Task 6: Verification

- [ ] Run `cargo test --workspace`.
- [ ] Run `flutter analyze`.
- [ ] Run `flutter test`.
- [ ] Run `flutter build macos --debug`.
