import 'package:flutter/material.dart';

import 'alternate_unlock_dialog_panel.dart';
import 'alternate_unlock_draft.dart';

Future<AlternateUnlockDraft?> showAlternateUnlockDialog(
  BuildContext context,
  String masterPassword,
) {
  return showDialog<AlternateUnlockDraft>(
    context: context,
    builder: (context) =>
        AlternateUnlockDialogPanel(masterPassword: masterPassword),
  );
}
