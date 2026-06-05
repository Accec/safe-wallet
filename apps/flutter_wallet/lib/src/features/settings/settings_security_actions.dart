import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/state/wallet_providers.dart';
import '../session/session_controller.dart';
import '../wallets/wallet_workspace_controller.dart';
import 'security_dialogs.dart';

class SettingsSecurityActions {
  const SettingsSecurityActions({
    required this.context,
    required this.ref,
    required this.isMounted,
    required this.setBiometricEnabled,
    required this.setSecurityBusy,
  });

  final BuildContext context;
  final WidgetRef ref;
  final bool Function() isMounted;
  final ValueChanged<bool> setBiometricEnabled;
  final ValueChanged<bool> setSecurityBusy;

  Future<void> toggleBiometrics(bool enabled) async {
    final password = await promptMasterPassword(
      context: context,
      title: enabled ? 'Enable biometrics' : 'Disable biometrics',
      actionLabel: enabled ? 'Enable' : 'Disable',
    );
    if (password == null || !context.mounted || !isMounted()) {
      return;
    }

    setSecurityBusy(true);
    try {
      await ref
          .read(walletApiProvider)
          .updateBiometricUnlock(password: password, enabled: enabled);
      if (!context.mounted || !isMounted()) {
        return;
      }
      setBiometricEnabled(enabled);
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            enabled ? 'Biometrics enabled.' : 'Biometrics disabled.',
          ),
        ),
      );
    } catch (error) {
      if (context.mounted && isMounted()) {
        showSafeWalletError(context, error);
      }
    } finally {
      if (context.mounted && isMounted()) {
        setSecurityBusy(false);
      }
    }
  }

  Future<void> openAdvancedSecurity() async {
    final masterPassword = await promptMasterPassword(
      context: context,
      title: 'Advanced security',
      actionLabel: 'Continue',
    );
    if (masterPassword == null || !context.mounted || !isMounted()) {
      return;
    }

    await showAdvancedSecurityDialog(
      context: context,
      onAlternateUnlock: () => setAlternateUnlock(masterPassword),
    );
  }

  Future<void> setAlternateUnlock(String masterPassword) async {
    final passwords = await showAlternateUnlockDialog(context, masterPassword);
    if (passwords == null || !context.mounted || !isMounted()) {
      return;
    }

    setSecurityBusy(true);
    try {
      await ref
          .read(walletApiProvider)
          .setDuressPassword(
            masterPassword: passwords.masterPassword,
            duressPassword: passwords.alternatePassword,
          );
      if (context.mounted && isMounted()) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Security settings saved.')),
        );
      }
    } catch (error) {
      if (context.mounted && isMounted()) {
        showSafeWalletError(context, error);
      }
    } finally {
      if (context.mounted && isMounted()) {
        setSecurityBusy(false);
      }
    }
  }

  void lock() {
    ref.read(sessionControllerProvider.notifier).lock();
    ref.read(walletWorkspaceControllerProvider.notifier).reset();
  }
}
