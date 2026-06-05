import 'package:flutter/material.dart';

Future<MultisigSignatureDraft?> showSignatureDialog(
  BuildContext context,
) async {
  final ownerController = TextEditingController();
  final signatureController = TextEditingController();
  String? errorText;
  final result = await showDialog<MultisigSignatureDraft>(
    context: context,
    builder: (context) {
      return StatefulBuilder(
        builder: (context, setState) {
          void submit() {
            if (ownerController.text.trim().isEmpty ||
                signatureController.text.trim().isEmpty) {
              setState(() {
                errorText = 'Signature fields are required';
              });
              return;
            }
            Navigator.of(context).pop(
              MultisigSignatureDraft(
                ownerAddress: ownerController.text.trim(),
                signature: signatureController.text.trim(),
              ),
            );
          }

          return AlertDialog(
            title: const Text('Add multisig signature'),
            content: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 520),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  TextField(
                    controller: ownerController,
                    decoration: const InputDecoration(
                      labelText: 'Owner address',
                    ),
                  ),
                  const SizedBox(height: 12),
                  TextField(
                    controller: signatureController,
                    decoration: const InputDecoration(labelText: 'Signature'),
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
                icon: const Icon(Icons.edit_outlined),
                label: const Text('Add signature'),
              ),
            ],
          );
        },
      );
    },
  );
  await Future<void>.delayed(kThemeAnimationDuration);
  ownerController.dispose();
  signatureController.dispose();
  return result;
}

class MultisigSignatureDraft {
  const MultisigSignatureDraft({
    required this.ownerAddress,
    required this.signature,
  });

  final String ownerAddress;
  final String signature;
}
