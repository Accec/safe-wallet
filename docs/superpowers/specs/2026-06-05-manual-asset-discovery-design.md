# Manual Asset Discovery Design

## Goal

Safe Wallet adds a manual asset discovery flow that finds contract tokens for a selected wallet and chain only when the user explicitly clicks a discovery action.

## Scope

- No automatic refresh on app start, wallet switch, chain switch, unlock, or timer.
- Manual balance refresh continues to update assets already stored locally.
- Manual discovery queries third-party indexers for EVM ERC20 and TRON TRC20 holdings, stores discovered tokens in SQLite, and stores discovered balances.
- BTC remains native-only for this feature because Bitcoin does not expose ERC20/TRC20-style contract token holdings.

## Architecture

Core adds an `AssetDiscoveryProvider` abstraction and a `HttpAssetDiscoveryProvider` implementation. `WalletService::discover_assets` loads the selected wallet account, chooses the configured custom indexer endpoint or a built-in endpoint, asks the provider for discovered tokens, persists token metadata with source `auto_discovered`, and writes balances into `asset_balances`.

Storage keeps using the existing `tokens.source`, `tokens.visible`, and `asset_balances.source` columns. User-hidden tokens stay hidden on later discoveries; new discovered tokens are visible.

Flutter adds a separate "Discover assets" action beside "Refresh assets". The action shows a loading spinner and calls the new native command. It does not run automatically.

## Data Sources

EVM discovery accepts Etherscan V2/BscScan-compatible results from token holding or token transfer style payloads. TRON discovery accepts TronScan account token payloads and TRC20 transfer payloads. The parser is deliberately tolerant because these providers return similar data with different field names.

## Privacy

Discovery sends the selected wallet address to the configured indexer only when the user clicks the discovery action. The HTTP client uses the existing proxy/Tor privacy settings.

## Testing

Rust tests cover parsing EVM and TRON payloads, persistence of discovered tokens and balances, hidden-token behavior, and service orchestration with a fake provider. FFI tests cover the new `discover_assets` command. Flutter tests cover the button, loading state, and command wiring.
