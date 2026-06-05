import '../../../models.dart';
import '../../../wallet_api.dart';

class WalletsClient {
  const WalletsClient(this._api);

  final WalletApi _api;

  Future<String> generateMnemonic() => _api.generateMnemonic();
  Future<List<WalletSummary>> list() => _api.listWallets();
  Future<List<AccountSummary>> listAccounts(String walletId) =>
      _api.listAccounts(walletId);
  Future<WalletSummary> create({
    required String label,
    required String mnemonic,
    required String password,
  }) => _api.createWallet(label: label, mnemonic: mnemonic, password: password);
  Future<WalletSummary> importMnemonic({
    required String label,
    required String mnemonic,
    required String password,
  }) => _api.importWallet(label: label, mnemonic: mnemonic, password: password);
  Future<WalletSummary> importPrivateKey({
    required String label,
    required String privateKey,
    required String password,
  }) => _api.importPrivateKey(
    label: label,
    privateKey: privateKey,
    password: password,
  );
  Future<WalletSummary> importKeystore({
    required String label,
    required String keystoreJson,
    required String keystorePassword,
    required String password,
  }) => _api.importKeystore(
    label: label,
    keystoreJson: keystoreJson,
    keystorePassword: keystorePassword,
    password: password,
  );
  Future<KeystoreExport> exportKeystore({
    required String walletId,
    required String password,
  }) => _api.exportKeystore(walletId: walletId, password: password);
  Future<void> delete({required String walletId, required String password}) =>
      _api.deleteWallet(walletId: walletId, password: password);
}
