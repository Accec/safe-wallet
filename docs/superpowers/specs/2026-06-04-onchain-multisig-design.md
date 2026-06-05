# On-Chain Multisig Design

Safe Wallet will add first-version on-chain multisig support for EVM Safe accounts and TRON native permission accounts.

## Scope

- EVM chains use an existing Safe-compatible account address, owner addresses, and threshold.
- TRON uses an existing account with on-chain permission configuration, owner addresses, weights, threshold, and permission id.
- Safe Wallet stores multisig accounts, proposals, and owner signatures in SQLite.
- Transfer proposals produce chain-specific execution payload metadata:
  - EVM Safe: Safe account address, target address, value, call data, operation, nonce placeholder, and signature slots.
  - TRON: owner address, permission id, target address, amount, and signature slots.
- A proposal becomes ready only after collected owner signatures meet the configured threshold.

## Non-Goals For This First Cut

- Deploying Safe contracts.
- Mutating TRON account permissions.
- Broadcasting signed raw EVM or TRON transactions as a successful on-chain execution. The current wallet transaction layer does not yet contain a raw transaction signer/broadcaster, so this first cut must not claim an on-chain transaction hash unless that layer is later added.
- BTC PSBT multisig.

## UX

- Add a Multisig tab.
- Users can import an existing EVM Safe or TRON multisig account.
- Users can create a proposal from a multisig account.
- Users can add local owner confirmations from wallets already in Safe Wallet.
- The proposal list shows threshold progress and whether the proposal is ready for execution.

## Data

- `multisig_accounts`: account metadata, chain, address, threshold, permission id.
- `multisig_owners`: owner address and weight.
- `multisig_proposals`: target, asset, amount, chain-specific payload, status.
- `multisig_signatures`: owner address and signature payload.

## Testing

- Rust storage migration and CRUD tests.
- Rust service tests for threshold validation, proposal creation, duplicate owner rejection, and ready status.
- FFI tests for command serialization.
- Flutter API tests for command mapping.
- Widget tests for import account and proposal creation flow.
