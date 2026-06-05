import '../../../models.dart';
import '../../../wallet_api.dart';
import 'fields.dart';

AssetSummary assetFromNativeJson(Object? value) {
  if (value is! Map<String, dynamic>) {
    throw const WalletApiException('Invalid native response');
  }
  final contractAddress = value['contract_address'];
  if (contractAddress != null && contractAddress is! String) {
    throw const WalletApiException('Invalid native response');
  }
  return AssetSummary(
    id: nativeStringField(value, 'id'),
    chain: nativeStringField(value, 'chain'),
    symbol: nativeStringField(value, 'symbol'),
    name: nativeStringField(value, 'name'),
    decimals: nativeIntField(value, 'decimals'),
    kind: nativeStringField(value, 'kind'),
    balance: optionalNativeString(value, 'balance') ?? '0',
    contractAddress: contractAddress,
  );
}
