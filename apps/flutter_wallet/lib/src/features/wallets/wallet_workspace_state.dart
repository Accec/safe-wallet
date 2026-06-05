part of 'wallet_workspace_controller.dart';

class WalletWorkspaceState {
  const WalletWorkspaceState({
    this.wallets = const [],
    this.accounts = const [],
    this.assets = const [],
    this.activity = const [],
    this.selectedWallet,
    this.selectedChain = 'ethereum',
  });

  final List<WalletSummary> wallets;
  final List<AccountSummary> accounts;
  final List<AssetSummary> assets;
  final List<ActivitySummary> activity;
  final WalletSummary? selectedWallet;
  final String? selectedChain;

  WalletWorkspaceState copyWith({
    List<WalletSummary>? wallets,
    List<AccountSummary>? accounts,
    List<AssetSummary>? assets,
    List<ActivitySummary>? activity,
    WalletSummary? selectedWallet,
    bool clearSelectedWallet = false,
    String? selectedChain,
    bool clearSelectedChain = false,
  }) {
    return WalletWorkspaceState(
      wallets: wallets ?? this.wallets,
      accounts: accounts ?? this.accounts,
      assets: assets ?? this.assets,
      activity: activity ?? this.activity,
      selectedWallet: clearSelectedWallet
          ? null
          : selectedWallet ?? this.selectedWallet,
      selectedChain: clearSelectedChain
          ? null
          : selectedChain ?? this.selectedChain,
    );
  }
}
