import '../../../models.dart';
import '../../../wallet_api.dart';
import '../native_response_parsers.dart';
import '../native_wallet_gateway.dart';

class NativeActivityClient {
  const NativeActivityClient(this._gateway);

  final NativeWalletGateway _gateway;

  Future<List<ActivitySummary>> list(String walletId) async {
    final body = await _gateway.invokeDb(
      domain: 'activity',
      action: 'list',
      payload: <String, Object?>{'wallet_id': walletId},
    );
    if (body is! List<dynamic>) {
      throw const WalletApiException('Invalid native response');
    }
    return body.map(activityFromNativeJson).toList();
  }

  Future<void> sync(String walletId, {String? chain}) {
    return _gateway.invokeDb(
      domain: 'activity',
      action: 'sync',
      payload: <String, Object?>{'wallet_id': walletId, 'chain': chain},
    );
  }
}
