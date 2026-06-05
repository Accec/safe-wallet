import 'package:flutter/material.dart';

import '../../../core/ui/busy_icon.dart';
import '../../../models.dart';
import '../../../wallet_api.dart';
import 'asset_dialog_callbacks.dart';

Future<void> showRemoveTokenDialog({
  required BuildContext context,
  required AssetSummary asset,
  required RemoveCustomTokenCallback onRemoveCustomToken,
}) async {
  var busy = false;
  String? errorText;

  await showDialog<void>(
    context: context,
    builder: (context) {
      return StatefulBuilder(
        builder: (context, setState) {
          Future<void> submit() async {
            setState(() {
              busy = true;
              errorText = null;
            });
            try {
              await onRemoveCustomToken(asset.id);
              if (!context.mounted) {
                return;
              }
              Navigator.of(context).pop();
              ScaffoldMessenger.of(
                context,
              ).showSnackBar(const SnackBar(content: Text('Token removed.')));
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
            title: Text('Remove ${asset.symbol}?'),
            content: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                if (asset.contractAddress != null)
                  SelectableText(asset.contractAddress!),
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
                icon: busy ? const BusyIcon() : const Icon(Icons.delete),
                label: const Text('Remove'),
              ),
            ],
          );
        },
      );
    },
  );
}
