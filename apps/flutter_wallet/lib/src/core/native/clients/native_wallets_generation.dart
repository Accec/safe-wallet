part of 'native_wallets_client.dart';

mixin NativeWalletsGeneration {
  NativeWalletGateway get _gateway;

  Future<String> generateMnemonic() async {
    final body = await _gateway.invoke(
      domain: 'wallets',
      action: 'generate_mnemonic',
    );
    if (body is! Map<String, dynamic>) {
      throw const WalletApiException('Invalid native response');
    }
    return stringFieldFromNativeJson(body, 'mnemonic');
  }
}
