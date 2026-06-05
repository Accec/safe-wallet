import 'package:flutter/material.dart';

import 'alternate_unlock_dialog_actions.dart';
import 'alternate_unlock_draft.dart';
import 'alternate_unlock_form.dart';

class AlternateUnlockDialogPanel extends StatefulWidget {
  const AlternateUnlockDialogPanel({super.key, required this.masterPassword});

  final String masterPassword;

  @override
  State<AlternateUnlockDialogPanel> createState() =>
      _AlternateUnlockDialogPanelState();
}

class _AlternateUnlockDialogPanelState
    extends State<AlternateUnlockDialogPanel> {
  final TextEditingController _alternateController = TextEditingController();
  final TextEditingController _confirmController = TextEditingController();
  bool _busy = false;
  String? _errorText;

  @override
  void dispose() {
    _alternateController.dispose();
    _confirmController.dispose();
    super.dispose();
  }

  void _submit() {
    final alternatePassword = _alternateController.text;
    final confirmPassword = _confirmController.text;
    final validationError = _validationError(
      alternatePassword: alternatePassword,
      confirmPassword: confirmPassword,
    );
    if (validationError != null) {
      setState(() {
        _errorText = validationError;
      });
      return;
    }
    setState(() {
      _busy = true;
    });
    Navigator.of(context).pop(
      AlternateUnlockDraft(
        masterPassword: widget.masterPassword,
        alternatePassword: alternatePassword,
      ),
    );
  }

  String? _validationError({
    required String alternatePassword,
    required String confirmPassword,
  }) {
    if (alternatePassword.isEmpty) {
      return 'Alternate password is required';
    }
    if (widget.masterPassword == alternatePassword) {
      return 'Alternate password must be different';
    }
    if (alternatePassword != confirmPassword) {
      return 'Alternate passwords do not match';
    }
    return null;
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Alternate unlock'),
      content: AlternateUnlockForm(
        alternateController: _alternateController,
        confirmController: _confirmController,
        busy: _busy,
        errorText: _errorText,
        onSubmit: _submit,
      ),
      actions: [
        AlternateUnlockDialogActions(
          busy: _busy,
          onCancel: () => Navigator.of(context).pop(),
          onSubmit: _submit,
        ),
      ],
    );
  }
}
