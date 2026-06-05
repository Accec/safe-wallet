import 'package:flutter/material.dart';

import '../../../core/ui/screen_motion.dart';

class UnlockPasswordForm extends StatelessWidget {
  const UnlockPasswordForm({
    super.key,
    required this.controller,
    required this.initialized,
    required this.busy,
    required this.biometricBusy,
    required this.error,
    required this.onSubmit,
  });

  final TextEditingController controller;
  final bool initialized;
  final bool busy;
  final bool biometricBusy;
  final String? error;
  final VoidCallback onSubmit;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        TextField(
          controller: controller,
          obscureText: true,
          decoration: const InputDecoration(
            labelText: 'Master password',
            prefixIcon: Icon(Icons.lock_outline),
          ),
          onSubmitted: (_) => busy ? null : onSubmit(),
        ),
        MotionSwitcher(
          child: error == null
              ? const SizedBox.shrink(key: ValueKey('no-error'))
              : Padding(
                  key: const ValueKey('unlock-error'),
                  padding: const EdgeInsets.only(top: 12),
                  child: Text(
                    error!,
                    style: TextStyle(
                      color: Theme.of(context).colorScheme.error,
                    ),
                  ),
                ),
        ),
        const SizedBox(height: 16),
        FilledButton.icon(
          onPressed: busy || biometricBusy ? null : onSubmit,
          icon: busy
              ? const SizedBox.square(
                  dimension: 18,
                  child: CircularProgressIndicator(strokeWidth: 2),
                )
              : const Icon(Icons.lock_open),
          label: Text(initialized ? 'Unlock' : 'Create password'),
        ),
      ],
    );
  }
}
