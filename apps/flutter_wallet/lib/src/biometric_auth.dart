import 'package:local_auth/local_auth.dart';

abstract class BiometricAuth {
  Future<bool> canAuthenticate();
  Future<bool> authenticate();
}

class LocalBiometricAuth implements BiometricAuth {
  LocalBiometricAuth({LocalAuthentication? localAuthentication})
    : _localAuthentication = localAuthentication ?? LocalAuthentication();

  final LocalAuthentication _localAuthentication;

  @override
  Future<bool> canAuthenticate() async {
    return await _localAuthentication.canCheckBiometrics ||
        await _localAuthentication.isDeviceSupported();
  }

  @override
  Future<bool> authenticate() {
    return _localAuthentication.authenticate(
      localizedReason: 'Unlock Safe Wallet',
      biometricOnly: false,
      sensitiveTransaction: true,
      persistAcrossBackgrounding: true,
    );
  }
}
