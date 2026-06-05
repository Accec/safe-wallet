import 'package:flutter/material.dart';

import '../../../models.dart';

Future<CreateMultisigProposalDraft?> showCreateProposalDialog(
  BuildContext context,
  MultisigAccountSummary account,
) async {
  final toController = TextEditingController();
  final assetController = TextEditingController(
    text: account.chain == 'tron' ? 'TRX' : 'ETH',
  );
  final amountController = TextEditingController();
  String? errorText;

  final result = await showDialog<CreateMultisigProposalDraft>(
    context: context,
    builder: (context) {
      return StatefulBuilder(
        builder: (context, setState) {
          void submit() {
            if (toController.text.trim().isEmpty ||
                assetController.text.trim().isEmpty ||
                amountController.text.trim().isEmpty) {
              setState(() {
                errorText = 'Proposal fields are required';
              });
              return;
            }
            Navigator.of(context).pop(
              CreateMultisigProposalDraft(
                multisigAccountId: account.id,
                toAddress: toController.text.trim(),
                assetSymbol: assetController.text.trim(),
                amount: amountController.text.trim(),
              ),
            );
          }

          return AlertDialog(
            title: const Text('Create multisig proposal'),
            content: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 520),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  TextField(
                    controller: toController,
                    decoration: const InputDecoration(labelText: 'Recipient'),
                  ),
                  const SizedBox(height: 12),
                  TextField(
                    controller: assetController,
                    decoration: const InputDecoration(
                      labelText: 'Asset symbol',
                    ),
                  ),
                  const SizedBox(height: 12),
                  TextField(
                    controller: amountController,
                    keyboardType: const TextInputType.numberWithOptions(
                      decimal: true,
                    ),
                    decoration: const InputDecoration(labelText: 'Amount'),
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
            ),
            actions: [
              TextButton(
                onPressed: () => Navigator.of(context).pop(),
                child: const Text('Cancel'),
              ),
              FilledButton.icon(
                onPressed: submit,
                icon: const Icon(Icons.note_add_outlined),
                label: const Text('Create proposal'),
              ),
            ],
          );
        },
      );
    },
  );
  await Future<void>.delayed(kThemeAnimationDuration);
  toController.dispose();
  assetController.dispose();
  amountController.dispose();
  return result;
}
