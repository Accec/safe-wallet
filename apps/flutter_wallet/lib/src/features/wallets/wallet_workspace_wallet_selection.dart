part of 'wallet_workspace_controller.dart';

mixin WalletWorkspaceWalletSelection on Notifier<WalletWorkspaceState> {
  WalletClients get walletClients;

  Future<void> loadWallets() async {
    final wallets = await walletClients.wallets.list();
    final selectedWallet =
        wallets.any((wallet) => wallet.id == state.selectedWallet?.id)
        ? wallets.firstWhere((wallet) => wallet.id == state.selectedWallet?.id)
        : wallets.firstOrNull;
    state = state.copyWith(
      wallets: wallets,
      selectedWallet: selectedWallet,
      clearSelectedWallet: selectedWallet == null,
    );
    await loadWalletDetails();
  }

  Future<void> loadWalletDetails() async {
    final wallet = state.selectedWallet;
    if (wallet == null) {
      state = state.copyWith(
        accounts: const [],
        assets: const [],
        activity: const [],
      );
      return;
    }

    final accounts = await walletClients.wallets.listAccounts(wallet.id);
    final assets = await walletClients.assets.list(wallet.id);
    final activity = await walletClients.activity.list(wallet.id);
    final selectedChain = effectiveChain(
      safeWalletSupportedChains,
      state.selectedChain,
    );
    state = state.copyWith(
      accounts: accounts,
      assets: assets,
      activity: activity,
      selectedChain: selectedChain,
      clearSelectedChain: selectedChain == null,
    );
  }

  Future<void> selectWallet(WalletSummary wallet) async {
    if (state.selectedWallet?.id == wallet.id) {
      return;
    }
    state = state.copyWith(selectedWallet: wallet);
    await loadWalletDetails();
  }

  void selectChain(String chain) {
    state = state.copyWith(selectedChain: chain);
  }
}
