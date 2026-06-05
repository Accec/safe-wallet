import 'package:flutter/material.dart';

import '../../../wallet_api.dart';

void showWalletDialogError(BuildContext context, Object error) {
  final message = error is WalletApiException
      ? error.message
      : 'Wallet action failed';
  ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
}
