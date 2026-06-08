import 'package:flutter/material.dart';

void showPaymentRequestLoaded(BuildContext context) {
  ScaffoldMessenger.of(
    context,
  ).showSnackBar(const SnackBar(content: Text('Payment request loaded.')));
}

void showCreateWalletBeforePreview(BuildContext context) {
  ScaffoldMessenger.of(context).showSnackBar(
    const SnackBar(content: Text('Create a wallet before previewing.')),
  );
}

void showTransferBroadcasted(BuildContext context, String txHash) {
  ScaffoldMessenger.of(
    context,
  ).showSnackBar(SnackBar(content: Text('Transfer broadcasted: $txHash')));
}
