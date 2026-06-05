import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

Future<void> copyWalletAddress(BuildContext context, String address) async {
  await Clipboard.setData(ClipboardData(text: address));
  if (!context.mounted) {
    return;
  }
  ScaffoldMessenger.of(
    context,
  ).showSnackBar(const SnackBar(content: Text('Address copied.')));
}
