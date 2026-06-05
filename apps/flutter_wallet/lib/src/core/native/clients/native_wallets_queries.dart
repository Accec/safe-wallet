part of 'native_wallets_client.dart';

mixin NativeWalletsQueries {
  NativeWalletGateway get _gateway;

  Future<List<WalletSummary>> list() async {
    final body = await _gateway.invokeDb(domain: 'wallets', action: 'list');
    if (body is! List<dynamic>) {
      throw const WalletApiException('Invalid native response');
    }
    return body.map(walletFromNativeJson).toList();
  }

  Future<List<AccountSummary>> listAccounts(String walletId) async {
    final body = await _gateway.invokeDb(
      domain: 'wallets',
      action: 'list_accounts',
      payload: <String, Object?>{'wallet_id': walletId},
    );
    if (body is! List<dynamic>) {
      throw const WalletApiException('Invalid native response');
    }
    return body.map(accountFromNativeJson).toList();
  }
}
