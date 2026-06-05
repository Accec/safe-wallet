import '../../../../models.dart';
import '../demo_activity_client.dart';

mixin DemoActivityApi {
  DemoActivityClient get activityClient;

  Future<List<ActivitySummary>> listActivity(String walletId) {
    return activityClient.list(walletId);
  }

  Future<void> syncActivity(String walletId, {String? chain}) {
    return activityClient.sync(walletId, chain: chain);
  }
}
