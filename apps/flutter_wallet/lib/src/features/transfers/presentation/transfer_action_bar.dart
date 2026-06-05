import 'package:flutter/material.dart';

import '../../../core/ui/busy_icon.dart';

class TransferActionBar extends StatelessWidget {
  const TransferActionBar({
    super.key,
    required this.scanning,
    required this.importing,
    required this.previewing,
    required this.sending,
    required this.canSend,
    required this.onScanQr,
    required this.onImportQrImage,
    required this.onPreviewTransfer,
    required this.onSendTransfer,
  });

  final bool scanning;
  final bool importing;
  final bool previewing;
  final bool sending;
  final bool canSend;
  final VoidCallback onScanQr;
  final VoidCallback onImportQrImage;
  final VoidCallback onPreviewTransfer;
  final VoidCallback onSendTransfer;

  @override
  Widget build(BuildContext context) {
    return Wrap(
      spacing: 12,
      runSpacing: 12,
      children: [
        OutlinedButton.icon(
          onPressed: scanning || importing ? null : onScanQr,
          icon: scanning ? const BusyIcon() : const Icon(Icons.qr_code_scanner),
          label: const Text('Scan QR'),
        ),
        OutlinedButton.icon(
          onPressed: scanning || importing ? null : onImportQrImage,
          icon: importing ? const BusyIcon() : const Icon(Icons.image_search),
          label: const Text('Import QR'),
        ),
        FilledButton.icon(
          onPressed: previewing ? null : onPreviewTransfer,
          icon: previewing ? const BusyIcon() : const Icon(Icons.receipt_long),
          label: const Text('Preview'),
        ),
        if (canSend)
          FilledButton.icon(
            onPressed: sending ? null : onSendTransfer,
            icon: sending ? const BusyIcon() : const Icon(Icons.send),
            label: const Text('Send'),
          ),
      ],
    );
  }
}
