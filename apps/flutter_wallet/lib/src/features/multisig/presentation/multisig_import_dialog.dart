import 'package:flutter/material.dart';

import '../../../models.dart';
import 'multisig_import_form.dart';

Future<ImportMultisigAccountDraft?> showImportMultisigDialog(
  BuildContext context,
) {
  return showDialog<ImportMultisigAccountDraft>(
    context: context,
    builder: (context) => const MultisigImportForm(),
  );
}
