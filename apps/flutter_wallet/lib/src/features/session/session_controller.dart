import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/api/wallet_clients.dart';
import '../../models.dart';

final sessionControllerProvider =
    NotifierProvider<SessionController, SessionState>(SessionController.new);

class SessionState {
  const SessionState({
    this.statusLoaded = false,
    this.initialized = true,
    this.locked = true,
    this.biometricEnabled = false,
    this.sessionPassword,
  });

  final bool statusLoaded;
  final bool initialized;
  final bool locked;
  final bool biometricEnabled;
  final String? sessionPassword;

  SessionState copyWith({
    bool? statusLoaded,
    bool? initialized,
    bool? locked,
    bool? biometricEnabled,
    String? sessionPassword,
    bool clearSessionPassword = false,
  }) {
    return SessionState(
      statusLoaded: statusLoaded ?? this.statusLoaded,
      initialized: initialized ?? this.initialized,
      locked: locked ?? this.locked,
      biometricEnabled: biometricEnabled ?? this.biometricEnabled,
      sessionPassword: clearSessionPassword
          ? null
          : sessionPassword ?? this.sessionPassword,
    );
  }
}

class SessionController extends Notifier<SessionState> {
  @override
  SessionState build() => const SessionState();

  Future<AppStatus> loadStatus() async {
    final status = await ref.read(walletClientsProvider).auth.status();
    state = state.copyWith(
      statusLoaded: true,
      initialized: status.initialized,
      locked: status.locked,
      biometricEnabled: status.biometricEnabled,
    );
    return status;
  }

  Future<void> setup(String password) async {
    await ref.read(walletClientsProvider).auth.setMasterPassword(password);
    state = state.copyWith(
      statusLoaded: true,
      initialized: true,
      locked: false,
      biometricEnabled: false,
      sessionPassword: password,
    );
  }

  Future<void> unlock(String password) async {
    await ref.read(walletClientsProvider).auth.unlock(password);
    state = state.copyWith(locked: false, sessionPassword: password);
  }

  void markBiometricUnlocked() {
    state = state.copyWith(locked: false, clearSessionPassword: true);
  }

  void rememberPassword(String password) {
    state = state.copyWith(sessionPassword: password);
  }

  void lock() {
    state = state.copyWith(locked: true, clearSessionPassword: true);
  }
}
