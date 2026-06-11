import 'package:flutter/material.dart';

import '../../../models.dart';
import '../transfer_controller.dart';
import 'transfer_action_bar.dart';
import 'transfer_asset_selector.dart';
import 'transfer_preview_card.dart';

class TransferForm extends StatelessWidget {
  const TransferForm({
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
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        if (assets.isNotEmpty) ...[
          TransferAssetSelector(
            assets: assets,
            selectedAssetId: selectedAssetId,
            onAssetChanged: onAssetChanged,
          ),
          const SizedBox(height: 12),
        ],
        TextField(
          controller: recipientController,
          decoration: const InputDecoration(
            labelText: 'Recipient',
            prefixIcon: Icon(Icons.person_outline),
          ),
        ),
        const SizedBox(height: 12),
        TextField(
          controller: amountController,
          keyboardType: const TextInputType.numberWithOptions(decimal: true),
          decoration: const InputDecoration(
            labelText: 'Amount',
            prefixIcon: Icon(Icons.payments_outlined),
          ),
        ),
        const SizedBox(height: 16),
        TransferActionBar(
          scanning: scanning,
          importing: importing,
          previewing: transfer.previewing,
          sending: transfer.sending,
          canSend: transfer.canSend,
          onScanQr: onScanQr,
          onImportQrImage: onImportQrImage,
          onPreviewTransfer: onPreviewTransfer,
          onSendTransfer: onSendTransfer,
        ),
        TransferPreviewSection(
          preview: transfer.preview,
          blockIfEnergyInsufficient: transfer.blockIfEnergyInsufficient,
          onBlockIfEnergyInsufficientChanged:
              onBlockIfEnergyInsufficientChanged,
        ),
      ],
    );
  }
}
