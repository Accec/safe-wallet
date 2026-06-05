import 'package:flutter/material.dart';

import '../../../core/ui/screen_motion.dart';
import 'unlock_biometric_button.dart';
import 'unlock_brand_header.dart';
import 'unlock_password_form.dart';

class UnlockScreenBody extends StatelessWidget {
  const UnlockScreenBody({
    super.key,
    required this.initialized,
    required this.biometricEnabled,
    required this.hasBiometricUnlock,
    required this.passwordController,
    required this.busy,
    required this.biometricBusy,
    required this.error,
    required this.onSubmit,
    required this.onBiometricUnlock,
  });

  final bool initialized;
  final bool biometricEnabled;
  final bool hasBiometricUnlock;
  final TextEditingController passwordController;
  final bool busy;
  final bool biometricBusy;
  final String? error;
  final VoidCallback onSubmit;
  final VoidCallback onBiometricUnlock;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SafeArea(
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 360),
            child: Padding(
              padding: const EdgeInsets.all(24),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  MotionEntrance(
                    child: UnlockBrandHeader(initialized: initialized),
                  ),
                  const SizedBox(height: 24),
                  MotionEntrance(
                    delay: safeWalletLayerDelay(1),
                    child: UnlockPasswordForm(
                      controller: passwordController,
                      initialized: initialized,
                      busy: busy,
                      biometricBusy: biometricBusy,
                      error: error,
                      onSubmit: onSubmit,
                    ),
                  ),
                  MotionEntrance(
                    delay: safeWalletLayerDelay(2),
                    child: UnlockBiometricButton(
                      initialized: initialized,
                      biometricEnabled: biometricEnabled,
                      hasBiometricUnlock: hasBiometricUnlock,
                      busy: busy,
                      biometricBusy: biometricBusy,
                      onBiometricUnlock: onBiometricUnlock,
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
