import 'package:flutter/material.dart';

import '../../core/ui/busy_icon.dart';

class NetworkPrivacyDialogActions extends StatelessWidget {
  const NetworkPrivacyDialogActions({
    super.key,
    required this.busy,
    required this.proxyEnabled,
    required this.onCancel,
    required this.onTestConnection,
    required this.onSave,
  });

  final bool busy;
  final bool proxyEnabled;
  final VoidCallback onCancel;
  final VoidCallback onTestConnection;
  final VoidCallback onSave;

  @override
  Widget build(BuildContext context) {
    return OverflowBar(
      children: [
        TextButton(
          onPressed: busy ? null : onCancel,
          child: const Text('Cancel'),
        ),
        OutlinedButton.icon(
          onPressed: busy || !proxyEnabled ? null : onTestConnection,
          icon: busy ? const BusyIcon() : const Icon(Icons.network_check),
          label: const Text('Test connection'),
        ),
        FilledButton.icon(
          onPressed: busy ? null : onSave,
          icon: busy ? const BusyIcon() : const Icon(Icons.save_outlined),
          label: const Text('Save'),
        ),
      ],
    );
  }
}
