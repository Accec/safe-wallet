import '../../wallet_api.dart';
import 'native_library.dart';
import 'wallet_protocol.dart';

class NativeWalletGateway {
  NativeWalletGateway({required this.dbPath, required this.commandRunner});

  final String dbPath;
  final NativeCommandRunner commandRunner;
  final WalletProtocol _protocol = WalletProtocol();

  Future<dynamic> invoke({
    required String domain,
    required String action,
    Map<String, Object?> payload = const {},
  }) async {
    try {
      final command = _protocol.wrapRequest(
        domain: domain,
        action: action,
        payload: payload,
      );
      final response = await commandRunner(command);
      return _protocol.unwrapNativeResponse(response);
    } on WalletApiException {
      rethrow;
    } catch (_) {
      throw const WalletApiException('Native command failed');
    }
  }

  Future<dynamic> invokeDb({
    required String domain,
    required String action,
    Map<String, Object?> payload = const {},
  }) {
    return invoke(
      domain: domain,
      action: action,
      payload: <String, Object?>{'db_path': dbPath, ...payload},
    );
  }
}
