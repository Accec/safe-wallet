import '../../../models.dart';
import '../clients/native_wallets_client.dart';

mixin NativeWalletsApi {
  NativeWalletsClient get walletsClient;

  Future<String> generateMnemonic() => walletsClient.generateMnemonic();

  Future<WalletSummary> createWallet({
    required String label,
    required String mnemonic,
    required String password,
  }) {
    return walletsClient.create(
      label: label,
      mnemonic: mnemonic,
      password: password,
    );
  }

  Future<WalletSummary> importWallet({
    required String label,
    required String mnemonic,
    required String password,
  }) {
    return walletsClient.importMnemonic(
      label: label,
      mnemonic: mnemonic,
      password: password,
    );
  }

  Future<WalletSummary> importPrivateKey({
    required String label,
    required String privateKey,
    required String password,
  }) {
    return walletsClient.importPrivateKey(
      label: label,
      privateKey: privateKey,
      password: password,
    );
  }

  Future<WalletSummary> importKeystore({
    required String label,
    required String keystoreJson,
    required String keystorePassword,
    required String password,
  }) {
    return walletsClient.importKeystore(
      label: label,
      keystoreJson: keystoreJson,
      keystorePassword: keystorePassword,
      password: password,
    );
  }

  Future<KeystoreExport> exportKeystore({
    required String walletId,
    required String password,
  }) {
    return walletsClient.exportKeystore(walletId: walletId, password: password);
  }

  Future<void> deleteWallet({
    required String walletId,
    required String password,
  }) {
    return walletsClient.delete(walletId: walletId, password: password);
  }

  Future<List<WalletSummary>> listWallets() => walletsClient.list();

  Future<List<AccountSummary>> listAccounts(String walletId) {
    return walletsClient.listAccounts(walletId);
  }
}
