import 'package:flutter/material.dart';

class MultisigImportAccountFields extends StatelessWidget {
  const MultisigImportAccountFields({
    super.key,
    required this.labelController,
    required this.addressController,
    required this.thresholdController,
    required this.permissionController,
    required this.ownersController,
    required this.showPermissionId,
    required this.errorText,
  });

  final TextEditingController labelController;
  final TextEditingController addressController;
  final TextEditingController thresholdController;
  final TextEditingController permissionController;
  final TextEditingController ownersController;
  final bool showPermissionId;
  final String? errorText;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        const SizedBox(height: 12),
        TextField(
          controller: labelController,
          decoration: const InputDecoration(labelText: 'Label'),
        ),
        const SizedBox(height: 12),
        TextField(
          controller: addressController,
          decoration: const InputDecoration(labelText: 'Multisig address'),
        ),
        const SizedBox(height: 12),
        TextField(
          controller: thresholdController,
          keyboardType: TextInputType.number,
          decoration: const InputDecoration(labelText: 'Threshold'),
        ),
        if (showPermissionId) ...[
          const SizedBox(height: 12),
          TextField(
            controller: permissionController,
            keyboardType: TextInputType.number,
            decoration: const InputDecoration(labelText: 'Permission ID'),
          ),
        ],
        const SizedBox(height: 12),
        TextField(
          controller: ownersController,
          minLines: 3,
          maxLines: 5,
          decoration: const InputDecoration(
            labelText: 'Owners',
            hintText: 'address,weight',
          ),
        ),
        if (errorText != null) ...[
          const SizedBox(height: 12),
          Text(
            errorText!,
            style: TextStyle(color: Theme.of(context).colorScheme.error),
          ),
        ],
      ],
    );
  }
}
