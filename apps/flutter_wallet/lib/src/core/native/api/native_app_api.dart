import '../../../models.dart';
import '../clients/native_app_client.dart';

mixin NativeAppApi {
  NativeAppClient get appClient;

  Future<AppStatus> appStatus() => appClient.status();
}
