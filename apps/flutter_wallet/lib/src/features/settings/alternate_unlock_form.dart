import 'package:flutter/material.dart';

class AlternateUnlockForm extends StatelessWidget {
  const AlternateUnlockForm({
    super.key,
    required this.alternateController,
    required this.confirmController,
    required this.busy,
    required this.errorText,
    required this.onSubmit,
  });

  final TextEditingController alternateController;
  final TextEditingController confirmController;
  final bool busy;
  final String? errorText;
  final VoidCallback onSubmit;

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 420),
      child: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Text(
              'Using an alternate password opens an isolated wallet space.',
            ),
            const SizedBox(height: 8),
            const Text(
              'Keep an external backup of important wallets before enabling this option. After an alternate sign-in, the previous wallet space will no longer be available in this app.',
            ),
            const SizedBox(height: 12),
            TextField(
              controller: alternateController,
              enabled: !busy,
              obscureText: true,
              decoration: const InputDecoration(
                labelText: 'Alternate unlock password',
                prefixIcon: Icon(Icons.password_outlined),
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: confirmController,
              enabled: !busy,
              obscureText: true,
              decoration: const InputDecoration(
                labelText: 'Confirm alternate unlock password',
                prefixIcon: Icon(Icons.verified_user_outlined),
              ),
              onSubmitted: (_) {
                if (!busy) {
                  onSubmit();
                }
              },
            ),
            if (errorText != null) ...[
              const SizedBox(height: 12),
              Text(
                errorText!,
                style: TextStyle(color: Theme.of(context).colorScheme.error),
              ),
            ],
          ],
        ),
      ),
    );
  }
}
