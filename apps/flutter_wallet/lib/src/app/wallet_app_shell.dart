import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../biometric_auth.dart';
import '../core/state/wallet_providers.dart';
import '../core/ui/screen_motion.dart';
import '../features/session/session_controller.dart';
import '../features/session/presentation/unlock_screen.dart';
import '../features/wallets/wallet_workspace_controller.dart';
import 'status_loading_screen.dart';
import 'wallet_home.dart';

class WalletAppShell extends ConsumerStatefulWidget {
  const WalletAppShell({super.key});

  @override
  ConsumerState<WalletAppShell> createState() => _WalletAppState();
}

class _WalletAppState extends ConsumerState<WalletAppShell> {
  late final BiometricAuth _biometricAuth;

  @override
  void initState() {
    super.initState();
    _biometricAuth = ref.read(biometricAuthProvider);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _loadStatus();
    });
  }

  Future<void> _loadStatus() async {
    final status = await ref
        .read(sessionControllerProvider.notifier)
        .loadStatus();
    if (status.initialized && !status.locked) {
      await ref.read(walletWorkspaceControllerProvider.notifier).loadWallets();
    }
  }

  Future<void> _setup(String password) async {
    await ref.read(sessionControllerProvider.notifier).setup(password);
    await ref.read(walletWorkspaceControllerProvider.notifier).loadWallets();
  }

  Future<void> _unlock(String password) async {
    await ref.read(sessionControllerProvider.notifier).unlock(password);
    await ref.read(walletWorkspaceControllerProvider.notifier).loadWallets();
  }

  Future<void> _unlockWithBiometrics() async {
    if (!await _biometricAuth.canAuthenticate()) {
      throw StateError('Unlock failed');
    }
    final authenticated = await _biometricAuth.authenticate();
    if (!authenticated) {
      throw StateError('Unlock failed');
    }
    await ref.read(walletWorkspaceControllerProvider.notifier).loadWallets();
    ref.read(sessionControllerProvider.notifier).markBiometricUnlocked();
  }

  @override
  Widget build(BuildContext context) {
    final session = ref.watch(sessionControllerProvider);

    final home = !session.statusLoaded
        ? const StatusLoadingScreen()
        : !session.initialized
        ? UnlockScreen(initialized: false, onSetup: _setup, onUnlock: _unlock)
        : session.locked
        ? UnlockScreen(
            initialized: true,
            biometricEnabled: session.biometricEnabled,
            onBiometricUnlock: _unlockWithBiometrics,
            onUnlock: _unlock,
          )
        : const WalletHome();
    final homeKey = !session.statusLoaded
        ? 'loading'
        : !session.initialized
        ? 'setup'
        : session.locked
        ? 'locked'
        : 'home';

    return MotionSwitcher(
      duration: safeWalletLongMotionDuration,
      child: KeyedSubtree(key: ValueKey(homeKey), child: home),
    );
  }
}
