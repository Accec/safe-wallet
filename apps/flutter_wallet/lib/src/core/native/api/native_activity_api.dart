import '../../../models.dart';
import '../clients/native_activity_client.dart';

mixin NativeActivityApi {
  NativeActivityClient get activityClient;

  Future<List<ActivitySummary>> listActivity(String walletId) {
    return activityClient.list(walletId);
  }

  Future<void> syncActivity(String walletId, {String? chain}) {
    return activityClient.sync(walletId, chain: chain);
  }
}
