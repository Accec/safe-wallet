import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../biometric_auth.dart';
import '../core/routing/app_router.dart';
import '../core/state/wallet_providers.dart';
import '../core/theme/safe_wallet_theme.dart';
import '../wallet_api.dart';
import 'wallet_app_shell.dart';

class WalletApp extends StatelessWidget {
  const WalletApp({super.key, this.api, this.biometricAuth});

  final WalletApi? api;
  final BiometricAuth? biometricAuth;

  @override
  Widget build(BuildContext context) {
    return ProviderScope(
      overrides: [
        if (api != null) walletApiProvider.overrideWithValue(api!),
        if (biometricAuth != null)
          biometricAuthProvider.overrideWithValue(biometricAuth!),
      ],
      child: const _WalletMaterialApp(),
    );
  }
}

class _WalletMaterialApp extends StatelessWidget {
  const _WalletMaterialApp();

  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      title: 'Safe Wallet',
      theme: safeWalletTheme(),
      routerConfig: createSafeWalletRouter(child: const WalletAppShell()),
    );
  }
}
