part of 'native_wallets_client.dart';

mixin NativeWalletsExports {
  NativeWalletGateway get _gateway;

  Future<KeystoreExport> exportKeystore({
    required String walletId,
    required String password,
  }) async {
    final body = await _gateway.invokeDb(
      domain: 'wallets',
      action: 'export_keystore',
      payload: <String, Object?>{'wallet_id': walletId, 'password': password},
    );
    return keystoreExportFromNativeJson(body);
  }

  Future<void> delete({required String walletId, required String password}) {
    return _gateway.invokeDb(
      domain: 'wallets',
      action: 'delete',
      payload: <String, Object?>{'wallet_id': walletId, 'password': password},
    );
  }
}
