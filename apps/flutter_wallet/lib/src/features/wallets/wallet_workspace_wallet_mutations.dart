part of 'wallet_workspace_controller.dart';

mixin WalletWorkspaceWalletMutations
    on Notifier<WalletWorkspaceState>, WalletWorkspaceWalletSelection {
  @override
  WalletClients get walletClients;

  Future<String> generateMnemonic() => walletClients.wallets.generateMnemonic();

  Future<void> createWallet({
    required String label,
    required String mnemonic,
    required String password,
  }) async {
    final wallet = await walletClients.wallets.create(
      label: label,
      mnemonic: mnemonic,
      password: password,
    );
    state = state.copyWith(selectedWallet: wallet);
    await loadWallets();
  }

  Future<void> importWallet({
    required String label,
    required String mnemonic,
    required String password,
  }) async {
    final wallet = await walletClients.wallets.importMnemonic(
      label: label,
      mnemonic: mnemonic,
      password: password,
    );
    state = state.copyWith(selectedWallet: wallet);
    await loadWallets();
  }

  Future<void> importPrivateKey({
    required String label,
    required String privateKey,
    required String password,
  }) async {
    final wallet = await walletClients.wallets.importPrivateKey(
      label: label,
      privateKey: privateKey,
      password: password,
    );
    state = state.copyWith(selectedWallet: wallet);
    await loadWallets();
  }

  Future<void> importKeystore({
    required String label,
    required String keystoreJson,
    required String keystorePassword,
    required String password,
  }) async {
    final wallet = await walletClients.wallets.importKeystore(
      label: label,
      keystoreJson: keystoreJson,
      keystorePassword: keystorePassword,
      password: password,
    );
    state = state.copyWith(selectedWallet: wallet);
    await loadWallets();
  }

  Future<KeystoreExport> exportKeystore({
    required String walletId,
    required String password,
  }) {
    return walletClients.wallets.exportKeystore(
      walletId: walletId,
      password: password,
    );
  }

  Future<void> deleteWallet({
    required String walletId,
    required String password,
  }) async {
    await walletClients.wallets.delete(walletId: walletId, password: password);
    await loadWallets();
  }
}
