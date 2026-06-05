import '../../../models.dart';
import 'demo_wallet_store.dart';

class DemoAuthClient {
  const DemoAuthClient(this._store);

  final DemoWalletStore _store;

  Future<AppStatus> status() async {
    return AppStatus(
      initialized: _store.initialized,
      locked: _store.locked,
      biometricEnabled: _store.biometricEnabled,
    );
  }

  Future<void> setMasterPassword(String password) async {
    if (password.isEmpty) {
      throw StateError('Setup failed');
    }
    _store.initialized = true;
    _store.locked = false;
  }

  Future<void> unlock(String password) async {
    if (password.isEmpty) {
      throw StateError('Unlock failed');
    }
    _store.locked = false;
  }

  Future<void> setDuressPassword({
    required String masterPassword,
    required String duressPassword,
  }) async {
    if (masterPassword.isEmpty ||
        duressPassword.isEmpty ||
        masterPassword == duressPassword) {
      throw StateError('Security update failed');
    }
  }

  Future<void> updateBiometricUnlock({
    required String password,
    required bool enabled,
  }) async {
    if (password.isEmpty) {
      throw StateError('Security update failed');
    }
    _store.biometricEnabled = enabled;
  }
}
