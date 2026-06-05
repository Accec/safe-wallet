import 'package:flutter/material.dart';

import '../../core/ui/busy_icon.dart';

class AlternateUnlockDialogActions extends StatelessWidget {
  const AlternateUnlockDialogActions({
    super.key,
    required this.busy,
    required this.onCancel,
    required this.onSubmit,
  });

  final bool busy;
  final VoidCallback onCancel;
  final VoidCallback onSubmit;

  @override
  Widget build(BuildContext context) {
    return OverflowBar(
      children: [
        TextButton(
          onPressed: busy ? null : onCancel,
          child: const Text('Cancel'),
        ),
        FilledButton.icon(
          onPressed: busy ? null : onSubmit,
          icon: busy ? const BusyIcon() : const Icon(Icons.save_outlined),
          label: const Text('Save'),
        ),
      ],
    );
  }
}
