import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import '../../../wallet_api.dart';
import '../transfer_controller.dart';

void showTransferError(BuildContext context, Object error) {
  ScaffoldMessenger.of(
    context,
  ).showSnackBar(SnackBar(content: Text(transferErrorMessage(error))));
}

String transferErrorMessage(Object error) {
  return switch (error) {
    WalletApiException(:final message) => message,
    TransferStateException(:final message) => message,
    PlatformException(:final message)
        when message != null && message.isNotEmpty =>
      message,
    _ => 'Wallet action failed',
  };
}
