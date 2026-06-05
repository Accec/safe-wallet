import 'package:flutter/widgets.dart';

import '../../../models.dart';
import '../../../qr_scanner.dart';
import 'transfer_payment_loader.dart';

class TransferQrActions {
  const TransferQrActions({
    required this.assets,
    required this.recipientController,
    required this.amountController,
    required this.parsePaymentUri,
    required this.qrScanner,
    this.scanPayload,
    this.importImagePayload,
  });

  final List<AssetSummary> assets;
  final TextEditingController recipientController;
  final TextEditingController amountController;
  final PaymentUriParser parsePaymentUri;
  final QrScanner qrScanner;
  final Future<String> Function()? scanPayload;
  final Future<String?> Function()? importImagePayload;

  Future<TransferQrActionResult?> loadFromCamera() async {
    final payload = scanPayload != null
        ? await scanPayload!()
        : await qrScanner.scanQr();
    return _loadPayload(payload);
  }

  Future<TransferQrActionResult?> loadFromImage() async {
    final payload = importImagePayload != null
        ? await importImagePayload!()
        : await qrScanner.pickQrImagePayload();
    return _loadPayload(payload);
  }

  Future<TransferQrActionResult?> _loadPayload(String? payload) async {
    if (payload == null) {
      return null;
    }
    final result = await TransferPaymentLoader(
      assets: assets,
      recipientController: recipientController,
      amountController: amountController,
      parsePaymentUri: parsePaymentUri,
    ).load(payload);
    return TransferQrActionResult(selectedAssetId: result.selectedAssetId);
  }
}

class TransferQrActionResult {
  const TransferQrActionResult({required this.selectedAssetId});

  final String? selectedAssetId;
}
