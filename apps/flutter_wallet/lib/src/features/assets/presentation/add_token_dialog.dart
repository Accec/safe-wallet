import 'package:flutter/material.dart';

import '../../../core/ui/busy_icon.dart';
import '../../../wallet_api.dart';
import 'asset_dialog_callbacks.dart';

Future<void> showAddTokenDialog({
  required BuildContext context,
  required String selectedChain,
  required AddCustomTokenCallback onAddCustomToken,
}) async {
  final tokenNameController = TextEditingController();
  final contractController = TextEditingController();
  var busy = false;
  String? errorText;

  await showDialog<void>(
    context: context,
    builder: (context) {
      return StatefulBuilder(
        builder: (context, setState) {
          Future<void> submit() async {
            final tokenName = tokenNameController.text.trim();
            final contractAddress = contractController.text.trim();
            if (tokenName.isEmpty) {
              setState(() {
                errorText = 'Token name is required';
              });
              return;
            }
            if (contractAddress.isEmpty) {
              setState(() {
                errorText = 'Contract address is required';
              });
              return;
            }
            setState(() {
              busy = true;
              errorText = null;
            });
            try {
              await onAddCustomToken(
                chain: selectedChain,
                tokenName: tokenName,
                contractAddress: contractAddress,
              );
              if (context.mounted) {
                Navigator.of(context).pop();
              }
            } catch (error) {
              if (context.mounted) {
                setState(() {
                  errorText = error is WalletApiException
                      ? error.message
                      : 'Wallet action failed';
                });
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
            title: const Text('Add token'),
            content: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                TextField(
                  controller: tokenNameController,
                  enabled: !busy,
                  decoration: const InputDecoration(labelText: 'Token name'),
                ),
                const SizedBox(height: 12),
                TextField(
                  controller: contractController,
                  enabled: !busy,
                  decoration: const InputDecoration(
                    labelText: 'Contract address',
                  ),
                ),
                if (errorText != null) ...[
                  const SizedBox(height: 12),
                  Text(
                    errorText!,
                    style: TextStyle(
                      color: Theme.of(context).colorScheme.error,
                    ),
                  ),
                ],
              ],
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
                    : const Icon(Icons.add_circle_outline),
                label: const Text('Add token'),
              ),
            ],
          );
        },
      );
    },
  );
  await Future<void>.delayed(kThemeAnimationDuration);
  tokenNameController.dispose();
  contractController.dispose();
}
