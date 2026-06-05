import 'package:flutter/material.dart';

import '../../../core/ui/busy_icon.dart';

class MultisigScreenActions extends StatelessWidget {
  const MultisigScreenActions({
    super.key,
    required this.loading,
    required this.busy,
    required this.onReload,
    required this.onImportAccount,
  });

  final bool loading;
  final bool busy;
  final VoidCallback onReload;
  final VoidCallback onImportAccount;

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        IconButton(
          tooltip: 'Refresh multisig',
          onPressed: busy ? null : onReload,
          icon: loading ? const BusyIcon() : const Icon(Icons.refresh),
        ),
        IconButton(
          tooltip: 'Import multisig',
          onPressed: busy ? null : onImportAccount,
          icon: const Icon(Icons.add_circle_outline),
        ),
      ],
    );
  }
}
