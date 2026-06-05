import '../../../models.dart';
import '../../../wallet_api.dart';
import '../native_response_parsers.dart';
import '../native_wallet_gateway.dart';

class NativeAssetsClient {
  const NativeAssetsClient(this._gateway);

  final NativeWalletGateway _gateway;

  Future<List<AssetSummary>> list(String walletId) async {
    final body = await _gateway.invokeDb(
      domain: 'assets',
      action: 'list',
      payload: <String, Object?>{'wallet_id': walletId},
    );
    if (body is! List<dynamic>) {
      throw const WalletApiException('Invalid native response');
    }
    return body.map(assetFromNativeJson).toList();
  }

  Future<void> refresh(String walletId, {String? chain}) {
    return _gateway.invokeDb(
      domain: 'assets',
      action: 'refresh',
      payload: <String, Object?>{'wallet_id': walletId, 'chain': chain},
    );
  }

  Future<void> discover(String walletId, {String? chain}) {
    return _gateway.invokeDb(
      domain: 'assets',
      action: 'discover',
      payload: <String, Object?>{'wallet_id': walletId, 'chain': chain},
    );
  }

  Future<AssetSummary> addCustomToken({
    required String chain,
    required String contractAddress,
    required String tokenName,
  }) async {
    final body = await _gateway.invokeDb(
      domain: 'assets',
      action: 'add_custom_token',
      payload: <String, Object?>{
        'chain': chain,
        'contract_address': contractAddress,
        'token_name': tokenName,
      },
    );
    return assetFromNativeJson(body);
  }

  Future<void> removeCustomToken(String assetId) {
    return _gateway.invokeDb(
      domain: 'assets',
      action: 'remove_custom_token',
      payload: <String, Object?>{'asset_id': assetId},
    );
  }
}
