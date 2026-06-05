import 'package:flutter/material.dart';

import '../../../core/ui/busy_icon.dart';
import 'wallet_dialog_error.dart';

Future<void> showPrivateKeyDialog({
  required BuildContext context,
  required Future<void> Function({
    required String label,
    required String privateKey,
  })
  onSubmit,
}) async {
  final labelController = TextEditingController();
  final privateKeyController = TextEditingController();
  var busy = false;

  await showDialog<void>(
    context: context,
    builder: (context) {
      return StatefulBuilder(
        builder: (context, setState) {
          Future<void> submit() async {
            setState(() {
              busy = true;
            });
            try {
              await onSubmit(
                label: labelController.text.trim().isEmpty
                    ? 'Imported key'
                    : labelController.text.trim(),
                privateKey: privateKeyController.text.trim(),
              );
              if (context.mounted) {
                Navigator.of(context).pop();
              }
            } catch (error) {
              if (context.mounted) {
                showWalletDialogError(context, error);
              }
            } finally {
              if (context.mounted) {
                setState(() {
                  busy = false;
                });
              }
            }
          }

          return AlertDialog(
            title: const Text('Import private key'),
            content: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 520),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  TextField(
                    controller: labelController,
                    enabled: !busy,
                    decoration: const InputDecoration(
                      labelText: 'Wallet label',
                    ),
                  ),
                  const SizedBox(height: 12),
                  TextField(
                    controller: privateKeyController,
                    enabled: !busy,
                    obscureText: true,
                    decoration: const InputDecoration(
                      labelText: 'Private key',
                      prefixIcon: Icon(Icons.key_outlined),
                    ),
                  ),
                ],
              ),
            ),
            actions: [
              TextButton(
                onPressed: busy ? null : () => Navigator.of(context).pop(),
                child: const Text('Cancel'),
              ),
              FilledButton.icon(
                onPressed: busy ? null : submit,
                icon: busy ? const BusyIcon() : const Icon(Icons.key_outlined),
                label: const Text('Import private key'),
              ),
            ],
          );
        },
      );
    },
  );
  await Future<void>.delayed(kThemeAnimationDuration);
  labelController.dispose();
  privateKeyController.dispose();
}
