import '../../../models.dart';
import 'demo_wallet_store.dart';

class DemoActivityClient {
  const DemoActivityClient(this._store);

  final DemoWalletStore _store;

  Future<List<ActivitySummary>> list(String walletId) async {
    return _store.activity;
  }

  Future<void> sync(String walletId, {String? chain}) async {}
}
