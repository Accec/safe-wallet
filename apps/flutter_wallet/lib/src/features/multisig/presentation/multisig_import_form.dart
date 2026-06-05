import 'package:flutter/material.dart';

import '../../../models.dart';
import 'multisig_import_account_fields.dart';
import 'multisig_import_type_fields.dart';
import 'multisig_owner_parser.dart';

class MultisigImportForm extends StatefulWidget {
  const MultisigImportForm({super.key});

  @override
  State<MultisigImportForm> createState() => _MultisigImportFormState();
}

class _MultisigImportFormState extends State<MultisigImportForm> {
  final TextEditingController _labelController = TextEditingController(
    text: 'Treasury Safe',
  );
  final TextEditingController _addressController = TextEditingController();
  final TextEditingController _thresholdController = TextEditingController(
    text: '2',
  );
  final TextEditingController _permissionController = TextEditingController(
    text: '2',
  );
  final TextEditingController _ownersController = TextEditingController();
  var _chain = 'ethereum';
  var _kind = 'evm_safe';
  String? _errorText;

  @override
  void dispose() {
    _labelController.dispose();
    _addressController.dispose();
    _thresholdController.dispose();
    _permissionController.dispose();
    _ownersController.dispose();
    super.dispose();
  }

  void _submit() {
    final threshold = int.tryParse(_thresholdController.text.trim());
    final permissionId = int.tryParse(_permissionController.text.trim());
    final owners = parseMultisigOwners(_ownersController.text);
    if (threshold == null || threshold <= 0 || owners.isEmpty) {
      setState(() {
        _errorText = 'Invalid multisig settings';
      });
      return;
    }
    if (_kind == 'tron_permission' && permissionId == null) {
      setState(() {
        _errorText = 'Permission ID is required';
      });
      return;
    }

    Navigator.of(context).pop(
      ImportMultisigAccountDraft(
        label: _labelController.text.trim().isEmpty
            ? 'Multisig'
            : _labelController.text.trim(),
        chain: _chain,
        kind: _kind,
        address: _addressController.text.trim(),
        threshold: threshold,
        permissionId: _kind == 'tron_permission' ? permissionId : null,
        owners: owners,
      ),
    );
  }

  void _selectKind(String value) {
    setState(() {
      _kind = value;
      _chain = value == 'tron_permission' ? 'tron' : 'ethereum';
    });
  }

  void _selectChain(String value) {
    setState(() {
      _chain = value;
    });
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Import multisig'),
      content: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 560),
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              MultisigImportTypeFields(
                kind: _kind,
                chain: _chain,
                onKindChanged: _selectKind,
                onChainChanged: _kind == 'tron_permission'
                    ? null
                    : _selectChain,
              ),
              MultisigImportAccountFields(
                labelController: _labelController,
                addressController: _addressController,
                thresholdController: _thresholdController,
                permissionController: _permissionController,
                ownersController: _ownersController,
                showPermissionId: _kind == 'tron_permission',
                errorText: _errorText,
              ),
            ],
          ),
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        FilledButton.icon(
          onPressed: _submit,
          icon: const Icon(Icons.add),
          label: const Text('Import'),
        ),
      ],
    );
  }
}
