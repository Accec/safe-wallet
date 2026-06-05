import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import '../../../core/ui/busy_icon.dart';
import '../../../models.dart';
import 'wallet_dialog_error.dart';

Future<void> showExportKeystoreDialog({
  required BuildContext context,
  required WalletSummary wallet,
  required Future<KeystoreExport> Function({
    required String walletId,
    required String password,
  })
  onExportKeystore,
}) async {
  final passwordController = TextEditingController();
  var busy = false;
  String? exportedJson;

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
              final export = await onExportKeystore(
                walletId: wallet.id,
                password: passwordController.text,
              );
              if (!context.mounted) {
                return;
              }
              setState(() {
                exportedJson = const JsonEncoder.withIndent(
                  '  ',
                ).convert(export.toJson());
              });
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

          Future<void> copyExport() async {
            final json = exportedJson;
            if (json == null) {
              return;
            }
            await Clipboard.setData(ClipboardData(text: json));
            if (context.mounted) {
              ScaffoldMessenger.of(
                context,
              ).showSnackBar(const SnackBar(content: Text('Keystore copied.')));
            }
          }

          return AlertDialog(
            title: Text('Export ${wallet.label}'),
            content: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 560),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  TextField(
                    controller: passwordController,
                    enabled: !busy,
                    obscureText: true,
                    decoration: const InputDecoration(
                      labelText: 'Master password',
                      prefixIcon: Icon(Icons.lock_outline),
                    ),
                  ),
                  if (exportedJson != null) ...[
                    const SizedBox(height: 16),
                    ConstrainedBox(
                      constraints: const BoxConstraints(maxHeight: 280),
                      child: SingleChildScrollView(
                        child: SelectableText(
                          exportedJson!,
                          style: Theme.of(context).textTheme.bodySmall,
                        ),
                      ),
                    ),
                  ],
                ],
              ),
            ),
            actions: [
              TextButton(
                onPressed: busy ? null : () => Navigator.of(context).pop(),
                child: Text(exportedJson == null ? 'Cancel' : 'Close'),
              ),
              if (exportedJson != null)
                TextButton.icon(
                  onPressed: busy ? null : copyExport,
                  icon: const Icon(Icons.copy_outlined),
                  label: const Text('Copy'),
                ),
              FilledButton.icon(
                onPressed: busy ? null : submit,
                icon: busy
                    ? const BusyIcon()
                    : const Icon(Icons.ios_share_outlined),
                label: Text(exportedJson == null ? 'Export' : 'Export again'),
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
