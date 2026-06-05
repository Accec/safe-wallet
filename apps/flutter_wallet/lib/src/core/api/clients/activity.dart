import '../../../models.dart';
import '../../../wallet_api.dart';

class ActivityClient {
  const ActivityClient(this._api);

  final WalletApi _api;

  Future<List<ActivitySummary>> list(String walletId) =>
      _api.listActivity(walletId);
  Future<void> sync(String walletId, {String? chain}) =>
      _api.syncActivity(walletId, chain: chain);
}
