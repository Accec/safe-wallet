import 'package:flutter/material.dart';

import '../../../core/ui/screen_motion.dart';

class UnlockBiometricButton extends StatelessWidget {
  const UnlockBiometricButton({
    super.key,
    required this.initialized,
    required this.biometricEnabled,
    required this.hasBiometricUnlock,
    required this.busy,
    required this.biometricBusy,
    required this.onBiometricUnlock,
  });

  final bool initialized;
  final bool biometricEnabled;
  final bool hasBiometricUnlock;
  final bool busy;
  final bool biometricBusy;
  final VoidCallback onBiometricUnlock;

  @override
  Widget build(BuildContext context) {
    return MotionSwitcher(
      child: initialized && biometricEnabled && hasBiometricUnlock
          ? Padding(
              key: const ValueKey('biometric-unlock'),
              padding: const EdgeInsets.only(top: 12),
              child: OutlinedButton.icon(
                onPressed: busy || biometricBusy ? null : onBiometricUnlock,
                icon: biometricBusy
                    ? const SizedBox.square(
                        dimension: 18,
                        child: CircularProgressIndicator(strokeWidth: 2),
                      )
                    : const Icon(Icons.fingerprint),
                label: const Text('Use biometrics'),
              ),
            )
          : const SizedBox.shrink(key: ValueKey('no-biometric-unlock')),
    );
  }
}
