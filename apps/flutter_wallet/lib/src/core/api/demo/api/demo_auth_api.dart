import '../demo_auth_client.dart';

mixin DemoAuthApi {
  DemoAuthClient get authClient;

  Future<void> setMasterPassword(String password) {
    return authClient.setMasterPassword(password);
  }

  Future<void> unlockApp(String password) => authClient.unlock(password);

  Future<void> setDuressPassword({
    required String masterPassword,
    required String duressPassword,
  }) {
    return authClient.setDuressPassword(
      masterPassword: masterPassword,
      duressPassword: duressPassword,
    );
  }

  Future<void> updateBiometricUnlock({
    required String password,
    required bool enabled,
  }) {
    return authClient.updateBiometricUnlock(
      password: password,
      enabled: enabled,
    );
  }
}
