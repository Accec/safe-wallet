import 'package:flutter/material.dart';

import '../../core/ui/busy_icon.dart';
import '../../models.dart';
import '../../wallet_api.dart';
import 'network_settings_form.dart';
import 'network_settings_validation.dart';

Future<void> showNetworkSettingsDialog({
  required BuildContext context,
  required WalletApi api,
  required VoidCallback onSaved,
  NetworkSettings? network,
}) async {
  final form = NetworkSettingsFormControllers.fromNetwork(network);
  var busy = false;
  String? errorText;

  await showDialog<void>(
    context: context,
    builder: (context) {
      return StatefulBuilder(
        builder: (context, setState) {
          Future<void> submit() async {
            final draft = form.toDraft();
            final validationError = networkSettingsValidationError(draft);
            if (validationError != null) {
              setState(() {
                errorText = validationError;
              });
              return;
            }
            setState(() {
              busy = true;
              errorText = null;
            });
            try {
              await api.saveNetworkSettings(draft);
              if (!context.mounted) {
                return;
              }
              Navigator.of(context).pop();
              onSaved();
              ScaffoldMessenger.of(
                context,
              ).showSnackBar(const SnackBar(content: Text('Network saved.')));
            } catch (error) {
              if (context.mounted) {
                setState(() {
                  errorText = error is WalletApiException
                      ? error.message
                      : 'Wallet action failed';
                });
              }
            } finally {
              if (context.mounted) {
                setState(() {
                  busy = false;
                });
              }
            }
          }

          return AlertDialog(
            title: Text(network == null ? 'Add network' : 'Edit network'),
            content: NetworkSettingsForm(
              controllers: form,
              busy: busy,
              errorText: errorText,
            ),
            actions: [
              TextButton(
                onPressed: busy ? null : () => Navigator.of(context).pop(),
                child: const Text('Cancel'),
              ),
              FilledButton.icon(
                onPressed: busy ? null : submit,
                icon: busy ? const BusyIcon() : const Icon(Icons.save_outlined),
                label: const Text('Save network'),
              ),
            ],
          );
        },
      );
    },
  );

  await Future<void>.delayed(kThemeAnimationDuration);
  form.dispose();
}
