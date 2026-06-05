import '../native_wallet_gateway.dart';

class NativeAuthClient {
  const NativeAuthClient(this._gateway);

  final NativeWalletGateway _gateway;

  Future<void> setMasterPassword(String password) {
    return _gateway.invokeDb(
      domain: 'auth',
      action: 'set_master_password',
      payload: <String, Object?>{'password': password},
    );
  }

  Future<void> unlock(String password) {
    return _gateway.invokeDb(
      domain: 'auth',
      action: 'unlock',
      payload: <String, Object?>{'password': password},
    );
  }

  Future<void> setDuressPassword({
    required String masterPassword,
    required String duressPassword,
  }) {
    return _gateway.invokeDb(
      domain: 'auth',
      action: 'set_duress_password',
      payload: <String, Object?>{
        'master_password': masterPassword,
        'duress_password': duressPassword,
      },
    );
  }

  Future<void> updateBiometricUnlock({
    required String password,
    required bool enabled,
  }) {
    return _gateway.invokeDb(
      domain: 'auth',
      action: 'update_biometric_unlock',
      payload: <String, Object?>{'password': password, 'enabled': enabled},
    );
  }
}
