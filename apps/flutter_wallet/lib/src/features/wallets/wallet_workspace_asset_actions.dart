part of 'wallet_workspace_controller.dart';

mixin WalletWorkspaceAssetActions
    on Notifier<WalletWorkspaceState>, WalletWorkspaceWalletSelection {
  @override
  WalletClients get walletClients;

  Future<void> addCustomToken({
    required String chain,
    required String contractAddress,
    required String tokenName,
  }) async {
    await walletClients.assets.addCustomToken(
      chain: chain,
      contractAddress: contractAddress,
      tokenName: tokenName,
    );
    await loadWalletDetails();
  }

  Future<void> refreshAssets() async {
    final wallet = state.selectedWallet;
    if (wallet == null) {
      return;
    }
    await walletClients.assets.refresh(wallet.id, chain: state.selectedChain);
    await loadWalletDetails();
  }

  Future<void> discoverAssets() async {
    final wallet = state.selectedWallet;
    if (wallet == null) {
      return;
    }
    await walletClients.assets.discover(wallet.id, chain: state.selectedChain);
    await loadWalletDetails();
  }

  Future<void> syncActivity() async {
    final wallet = state.selectedWallet;
    if (wallet == null) {
      return;
    }
    await walletClients.activity.sync(wallet.id, chain: state.selectedChain);
    await loadWalletDetails();
  }

  Future<void> removeCustomToken(String assetId) async {
    await walletClients.assets.removeCustomToken(assetId);
    await loadWalletDetails();
  }
}
