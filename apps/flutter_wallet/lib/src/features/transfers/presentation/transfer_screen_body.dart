import 'package:flutter/material.dart';

import '../../../models.dart';
import '../transfer_controller.dart';
import 'transfer_form.dart';

class TransferScreenBody extends StatelessWidget {
  const TransferScreenBody({
    super.key,
    required this.assets,
    required this.selectedAssetId,
    required this.onAssetChanged,
    required this.recipientController,
    required this.amountController,
    required this.scanning,
    required this.importing,
    required this.transfer,
    required this.onScanQr,
    required this.onImportQrImage,
    required this.onPreviewTransfer,
    required this.onSendTransfer,
    required this.onBlockIfEnergyInsufficientChanged,
  });

  final List<AssetSummary> assets;
  final String? selectedAssetId;
  final ValueChanged<String> onAssetChanged;
  final TextEditingController recipientController;
  final TextEditingController amountController;
  final bool scanning;
  final bool importing;
  final TransferState transfer;
  final VoidCallback onScanQr;
  final VoidCallback onImportQrImage;
  final VoidCallback onPreviewTransfer;
  final VoidCallback onSendTransfer;
  final ValueChanged<bool> onBlockIfEnergyInsufficientChanged;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Transfer')),
      body: TransferForm(
        assets: assets,
        selectedAssetId: selectedAssetId,
        onAssetChanged: onAssetChanged,
        recipientController: recipientController,
        amountController: amountController,
        scanning: scanning,
        importing: importing,
        transfer: transfer,
        onScanQr: onScanQr,
        onImportQrImage: onImportQrImage,
        onPreviewTransfer: onPreviewTransfer,
        onSendTransfer: onSendTransfer,
        onBlockIfEnergyInsufficientChanged: onBlockIfEnergyInsufficientChanged,
      ),
    );
  }
}
