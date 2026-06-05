import '../../../models.dart';
import '../../../wallet_api.dart';
import 'fields.dart';

NetworkSettings networkSettingsFromNativeJson(Object? value) {
  if (value is! Map<String, dynamic>) {
    throw const WalletApiException('Invalid native response');
  }
  return NetworkSettings(
    chain: nativeStringField(value, 'chain'),
    networkName: nativeStringField(value, 'network_name'),
    chainId: optionalNativeString(value, 'chain_id'),
    enabled: nativeBoolField(value, 'enabled'),
    defaultRpcUrl: nativeStringField(value, 'default_rpc_url'),
    userRpcUrl: optionalNativeString(value, 'user_rpc_url'),
    indexerEndpoint: optionalNativeString(value, 'indexer_endpoint'),
    explorerUrl: optionalNativeString(value, 'explorer_url'),
    nativeSymbol: nativeStringField(value, 'native_symbol'),
    nativeDecimals: nativeIntField(value, 'native_decimals'),
  );
}

NetworkPrivacySettings networkPrivacySettingsFromNativeJson(Object? value) {
  if (value is! Map<String, dynamic>) {
    throw const WalletApiException('Invalid native response');
  }
  return NetworkPrivacySettings(
    proxyEnabled: nativeBoolField(value, 'proxy_enabled'),
    proxyMode: nativeStringField(value, 'proxy_mode'),
    proxyUrl: optionalNativeString(value, 'proxy_url'),
  );
}
