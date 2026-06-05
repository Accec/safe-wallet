import 'package:flutter/material.dart';

import '../../core/ui/busy_icon.dart';

Future<String?> promptMasterPassword({
  required BuildContext context,
  required String title,
  required String actionLabel,
}) {
  return showDialog<String>(
    context: context,
    builder: (context) =>
        _MasterPasswordDialog(title: title, actionLabel: actionLabel),
  );
}

class _MasterPasswordDialog extends StatefulWidget {
  const _MasterPasswordDialog({required this.title, required this.actionLabel});

  final String title;
  final String actionLabel;

  @override
  State<_MasterPasswordDialog> createState() => _MasterPasswordDialogState();
}

class _MasterPasswordDialogState extends State<_MasterPasswordDialog> {
  final TextEditingController _controller = TextEditingController();
  bool _busy = false;
  String? _errorText;

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _submit() {
    if (_controller.text.isEmpty) {
      setState(() {
        _errorText = 'Master password is required';
      });
      return;
    }
    setState(() {
      _busy = true;
    });
    Navigator.of(context).pop(_controller.text);
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(widget.title),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          TextField(
            controller: _controller,
            enabled: !_busy,
            obscureText: true,
            decoration: const InputDecoration(
              labelText: 'Master password',
              prefixIcon: Icon(Icons.lock_outline),
            ),
            onSubmitted: (_) => _busy ? null : _submit(),
          ),
          if (_errorText != null) ...[
            const SizedBox(height: 12),
            Text(
              _errorText!,
              style: TextStyle(color: Theme.of(context).colorScheme.error),
            ),
          ],
        ],
      ),
      actions: [
        TextButton(
          onPressed: _busy ? null : () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        FilledButton.icon(
          onPressed: _busy ? null : _submit,
          icon: _busy ? const BusyIcon() : const Icon(Icons.check),
          label: Text(widget.actionLabel),
        ),
      ],
    );
  }
}
