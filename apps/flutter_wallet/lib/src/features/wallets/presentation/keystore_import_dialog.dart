import 'package:flutter/material.dart';

import '../../../core/ui/busy_icon.dart';
import 'wallet_dialog_error.dart';

Future<void> showKeystoreImportDialog({
  required BuildContext context,
  required Future<void> Function({
    required String label,
    required String keystoreJson,
    required String keystorePassword,
    required String password,
  })
  onSubmit,
}) async {
  final labelController = TextEditingController();
  final keystoreController = TextEditingController();
  final keystorePasswordController = TextEditingController();
  final passwordController = TextEditingController();
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
                    ? 'Imported keystore'
                    : labelController.text.trim(),
                keystoreJson: keystoreController.text.trim(),
                keystorePassword: keystorePasswordController.text,
                password: passwordController.text,
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
            title: const Text('Import keystore'),
            content: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 560),
              child: SingleChildScrollView(
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
                      controller: keystoreController,
                      enabled: !busy,
                      minLines: 5,
                      maxLines: 10,
                      decoration: const InputDecoration(
                        labelText: 'Keystore JSON',
                        prefixIcon: Icon(Icons.data_object),
                      ),
                    ),
                    const SizedBox(height: 12),
                    TextField(
                      controller: keystorePasswordController,
                      enabled: !busy,
                      obscureText: true,
                      decoration: const InputDecoration(
                        labelText: 'Keystore password',
                        prefixIcon: Icon(Icons.password_outlined),
                      ),
                    ),
                    const SizedBox(height: 12),
                    TextField(
                      controller: passwordController,
                      enabled: !busy,
                      obscureText: true,
                      decoration: const InputDecoration(
                        labelText: 'Master password',
                        prefixIcon: Icon(Icons.lock_outline),
                      ),
                    ),
                  ],
                ),
              ),
            ),
            actions: [
              TextButton(
                onPressed: busy ? null : () => Navigator.of(context).pop(),
                child: const Text('Cancel'),
              ),
              FilledButton.icon(
                onPressed: busy ? null : submit,
                icon: busy
                    ? const BusyIcon()
                    : const Icon(Icons.inventory_2_outlined),
                label: const Text('Import keystore'),
              ),
            ],
          );
        },
      );
    },
  );
  await Future<void>.delayed(kThemeAnimationDuration);
  labelController.dispose();
  keystoreController.dispose();
  keystorePasswordController.dispose();
  passwordController.dispose();
}
