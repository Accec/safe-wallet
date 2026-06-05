import '../../../../models.dart';
import '../demo_auth_client.dart';

mixin DemoAppApi {
  DemoAuthClient get authClient;

  Future<AppStatus> appStatus() => authClient.status();
}
