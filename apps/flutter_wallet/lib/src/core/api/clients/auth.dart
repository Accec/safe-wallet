import '../../../models.dart';
import '../../../wallet_api.dart';

class AuthClient {
  const AuthClient(this._api);

  final WalletApi _api;

  Future<AppStatus> status() => _api.appStatus();
  Future<void> setMasterPassword(String password) =>
      _api.setMasterPassword(password);
  Future<void> unlock(String password) => _api.unlockApp(password);
  Future<void> setDuressPassword({
    required String masterPassword,
    required String duressPassword,
  }) => _api.setDuressPassword(
    masterPassword: masterPassword,
    duressPassword: duressPassword,
  );
  Future<void> updateBiometricUnlock({
    required String password,
    required bool enabled,
  }) => _api.updateBiometricUnlock(password: password, enabled: enabled);
}
