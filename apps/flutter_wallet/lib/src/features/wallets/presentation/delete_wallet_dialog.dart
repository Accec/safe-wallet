import 'package:flutter/material.dart';

import '../../../core/ui/busy_icon.dart';
import '../../../models.dart';
import 'wallet_dialog_error.dart';

Future<void> showDeleteWalletDialog({
  required BuildContext context,
  required WalletSummary wallet,
  required Future<void> Function({
    required String walletId,
    required String password,
  })
  onDeleteWallet,
}) async {
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
              await onDeleteWallet(
                walletId: wallet.id,
                password: passwordController.text,
              );
              if (!context.mounted) {
                return;
              }
              Navigator.of(context).pop();
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(content: Text('${wallet.label} deleted.')),
              );
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
            title: Text('Delete ${wallet.label}'),
            content: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 420),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  const Text(
                    'This removes the wallet and its local accounts, balances, and activity from this device.',
                  ),
                  const SizedBox(height: 16),
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
            actions: [
              TextButton(
                onPressed: busy ? null : () => Navigator.of(context).pop(),
                child: const Text('Cancel'),
              ),
              FilledButton.icon(
                onPressed: busy ? null : submit,
                icon: busy
                    ? const BusyIcon()
                    : const Icon(Icons.delete_outline),
                label: const Text('Delete wallet'),
              ),
            ],
          );
        },
      );
    },
  );
  await Future<void>.delayed(kThemeAnimationDuration);
  passwordController.dispose();
}
