import 'package:flutter/material.dart';

import '../../models.dart';
import '../../wallet_api.dart';
import 'network_privacy_dialog_panel.dart';

Future<void> showNetworkPrivacyDialog({
  required BuildContext context,
  required WalletApi api,
  required NetworkPrivacySettings settings,
  required VoidCallback onSaved,
}) async {
  final saved = await showDialog<bool>(
    context: context,
    builder: (context) =>
        NetworkPrivacyDialogPanel(api: api, settings: settings),
  );
  if (saved == true) {
    onSaved();
  }
}
