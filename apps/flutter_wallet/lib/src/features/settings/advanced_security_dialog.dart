import 'package:flutter/material.dart';

Future<void> showAdvancedSecurityDialog({
  required BuildContext context,
  required VoidCallback onAlternateUnlock,
}) async {
  await showDialog<void>(
    context: context,
    builder: (context) {
      return AlertDialog(
        title: const Text('Advanced security'),
        content: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 420),
          child: ListTile(
            contentPadding: EdgeInsets.zero,
            leading: const Icon(Icons.password_outlined),
            title: const Text('Alternate unlock'),
            subtitle: const Text(
              'Use a separate password to open an isolated wallet space.',
            ),
            trailing: const Icon(Icons.chevron_right),
            onTap: () {
              Navigator.of(context).pop();
              onAlternateUnlock();
            },
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Close'),
          ),
        ],
      );
    },
  );
}
