part of 'native_wallets_client.dart';

mixin NativeWalletsImports {
  NativeWalletGateway get _gateway;

  Future<WalletSummary> create({
    required String label,
    required String mnemonic,
    required String password,
  }) async {
    final body = await _gateway.invokeDb(
      domain: 'wallets',
      action: 'create',
      payload: <String, Object?>{
        'label': label,
        'mnemonic': mnemonic,
        'password': password,
      },
    );
    return walletFromNativeJson(body);
  }

  Future<WalletSummary> importMnemonic({
    required String label,
    required String mnemonic,
    required String password,
  }) async {
    final body = await _gateway.invokeDb(
      domain: 'wallets',
      action: 'import_mnemonic',
      payload: <String, Object?>{
        'label': label,
        'mnemonic': mnemonic,
        'password': password,
      },
    );
    return walletFromNativeJson(body);
  }

  Future<WalletSummary> importPrivateKey({
    required String label,
    required String privateKey,
    required String password,
  }) async {
    final body = await _gateway.invokeDb(
      domain: 'wallets',
      action: 'import_private_key',
      payload: <String, Object?>{
        'label': label,
        'private_key': privateKey,
        'password': password,
      },
    );
    return walletFromNativeJson(body);
  }

  Future<WalletSummary> importKeystore({
    required String label,
    required String keystoreJson,
    required String keystorePassword,
    required String password,
  }) async {
    final body = await _gateway.invokeDb(
      domain: 'wallets',
      action: 'import_keystore',
      payload: <String, Object?>{
        'label': label,
        'keystore_json': keystoreJson,
        'keystore_password': keystorePassword,
        'password': password,
      },
    );
    return walletFromNativeJson(body);
  }
}
