import 'package:flutter/material.dart';

import '../../../core/ui/busy_icon.dart';
import 'wallet_dialog_error.dart';

Future<void> showWalletDialog({
  required BuildContext context,
  required String title,
  required String actionLabel,
  Future<String> Function()? loadMnemonic,
  required Future<void> Function({
    required String label,
    required String mnemonic,
  })
  onSubmit,
}) async {
  final labelController = TextEditingController();
  final mnemonicController = TextEditingController();
  var busy = false;
  var loadingMnemonic = loadMnemonic != null;
  var loadStarted = false;

  await showDialog<void>(
    context: context,
    builder: (context) {
      return StatefulBuilder(
        builder: (context, setState) {
          if (loadingMnemonic && !loadStarted) {
            loadStarted = true;
            loadMnemonic!()
                .then((mnemonic) {
                  if (!context.mounted) {
                    return;
                  }
                  setState(() {
                    mnemonicController.text = mnemonic;
                    loadingMnemonic = false;
                  });
                })
                .catchError((Object error) {
                  if (!context.mounted) {
                    return;
                  }
                  setState(() {
                    loadingMnemonic = false;
                  });
                  showWalletDialogError(context, error);
                });
          }

          Future<void> submit() async {
            setState(() {
              busy = true;
            });
            try {
              await onSubmit(
                label: labelController.text.trim().isEmpty
                    ? 'Primary'
                    : labelController.text.trim(),
                mnemonic: mnemonicController.text.trim(),
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
            title: Text(title),
            content: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 520),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  TextField(
                    controller: labelController,
                    enabled: !busy && !loadingMnemonic,
                    decoration: const InputDecoration(
                      labelText: 'Wallet label',
                    ),
                  ),
                  const SizedBox(height: 12),
                  TextField(
                    controller: mnemonicController,
                    enabled: !busy && !loadingMnemonic,
                    minLines: 3,
                    maxLines: 4,
                    decoration: InputDecoration(
                      labelText: 'Recovery phrase',
                      suffixIcon: loadingMnemonic
                          ? const Padding(
                              padding: EdgeInsets.all(12),
                              child: BusyIcon(dimension: 16),
                            )
                          : null,
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
                onPressed: busy || loadingMnemonic ? null : submit,
                icon: busy || loadingMnemonic
                    ? const BusyIcon()
                    : const Icon(Icons.check),
                label: Text(loadingMnemonic ? 'Generating' : actionLabel),
              ),
            ],
          );
        },
      );
    },
  );
  await Future<void>.delayed(kThemeAnimationDuration);
  labelController.dispose();
  mnemonicController.dispose();
}
