import '../../../models.dart';
import '../../../wallet_api.dart';
import '../native_wallet_gateway.dart';

class NativeAppClient {
  const NativeAppClient(this._gateway);

  final NativeWalletGateway _gateway;

  Future<AppStatus> status() async {
    final body = await _gateway.invokeDb(domain: 'app', action: 'status');
    if (body is! Map<String, dynamic>) {
      throw const WalletApiException('Invalid native response');
    }
    final initialized = body['initialized'];
    final locked = body['locked'];
    final biometricEnabled = body['biometric_enabled'] ?? false;
    if (initialized is! bool || locked is! bool || biometricEnabled is! bool) {
      throw const WalletApiException('Invalid native response');
    }
    return AppStatus(
      initialized: initialized,
      locked: locked,
      biometricEnabled: biometricEnabled,
    );
  }
}
