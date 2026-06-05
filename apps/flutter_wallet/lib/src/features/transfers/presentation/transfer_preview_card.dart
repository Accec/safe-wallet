import 'package:flutter/material.dart';

import '../../../core/ui/screen_motion.dart';
import '../../../models.dart';

class TransferPreviewSection extends StatelessWidget {
  const TransferPreviewSection({super.key, required this.preview});

  final TransferPreview? preview;

  @override
  Widget build(BuildContext context) {
    final preview = this.preview;
    return MotionSwitcher(
      child: preview == null
          ? const SizedBox.shrink(key: ValueKey('no-preview'))
          : Padding(
              key: const ValueKey('transfer-preview'),
              padding: const EdgeInsets.only(top: 16),
              child: TransferPreviewCard(preview: preview),
            ),
    );
  }
}

class TransferPreviewCard extends StatelessWidget {
  const TransferPreviewCard({super.key, required this.preview});

  final TransferPreview preview;

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: const Icon(Icons.receipt_long),
      title: Text(
        '${preview.amount} ${preview.assetSymbol} to ${preview.toAddress}',
      ),
      subtitle: Text('Fee: ${preview.feeEstimate} · ${preview.rpcUrl}'),
    );
  }
}
