import '../../../models.dart';
import 'demo_wallet_store.dart';

class DemoWalletsClient {
  const DemoWalletsClient(this._store);

  final DemoWalletStore _store;

  Future<String> generateMnemonic() async {
    return 'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about';
  }

  Future<WalletSummary> create({
    required String label,
    required String mnemonic,
    required String password,
  }) async {
    final wallet = WalletSummary(
      id: 'wallet-${_store.wallets.length + 1}',
      label: label,
    );
    _store.wallets.add(wallet);
    return wallet;
  }

  Future<WalletSummary> importMnemonic({
    required String label,
    required String mnemonic,
    required String password,
  }) {
    return create(label: label, mnemonic: mnemonic, password: password);
  }

  Future<WalletSummary> importPrivateKey({
    required String label,
    required String privateKey,
    required String password,
  }) {
    return create(
      label: label,
      mnemonic: 'private-key-import',
      password: password,
    );
  }

  Future<WalletSummary> importKeystore({
    required String label,
    required String keystoreJson,
    required String keystorePassword,
    required String password,
  }) {
    if (keystoreJson.isEmpty || keystorePassword.isEmpty || password.isEmpty) {
      throw StateError('Import failed');
    }
    return create(
      label: label,
      mnemonic: 'keystore-import',
      password: password,
    );
  }

  Future<KeystoreExport> exportKeystore({
    required String walletId,
    required String password,
  }) async {
    final wallet = _store.wallets.firstWhere(
      (wallet) => wallet.id == walletId,
      orElse: () => WalletSummary(id: walletId, label: 'Wallet'),
    );
    return KeystoreExport(
      walletId: wallet.id,
      label: wallet.label,
      secretKind: 'private_key',
      ciphertextB64: 'encrypted-demo-ciphertext',
      nonceB64: 'encrypted-demo-nonce',
      saltB64: 'encrypted-demo-salt',
      kdfName: 'scrypt',
      kdfParamsJson: '{}',
      cipherName: 'AES-256-GCM',
      version: 1,
    );
  }

  Future<List<WalletSummary>> list() async {
    return _store.wallets;
  }

  Future<void> delete({
    required String walletId,
    required String password,
  }) async {
    if (password.isEmpty) {
      throw StateError('Delete failed');
    }
    _store.wallets.removeWhere((wallet) => wallet.id == walletId);
  }

  Future<List<AccountSummary>> listAccounts(String walletId) async {
    return [
      AccountSummary(
        walletId: walletId,
        chain: 'ethereum',
        address: '0x9858effd232b4033e47d90003d41ec34ecaeda94',
        derivationPath: "m/44'/60'/0'/0/0",
      ),
    ];
  }
}
