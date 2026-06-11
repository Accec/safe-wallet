import 'package:flutter/material.dart';

import '../../../core/ui/screen_motion.dart';
import '../../../models.dart';

class TransferPreviewSection extends StatelessWidget {
  const TransferPreviewSection({
    super.key,
    required this.preview,
    required this.blockIfEnergyInsufficient,
    required this.onBlockIfEnergyInsufficientChanged,
  });

  final TransferPreview? preview;
  final bool blockIfEnergyInsufficient;
  final ValueChanged<bool> onBlockIfEnergyInsufficientChanged;

  @override
  Widget build(BuildContext context) {
    final preview = this.preview;
    return MotionSwitcher(
      child: preview == null
          ? const SizedBox.shrink(key: ValueKey('no-preview'))
          : Padding(
              key: const ValueKey('transfer-preview'),
              padding: const EdgeInsets.only(top: 16),
              child: Column(
                children: [
                  TransferPreviewCard(preview: preview),
                  if (preview.resourceStatus != null)
                    CheckboxListTile(
                      value: blockIfEnergyInsufficient,
                      onChanged: (value) {
                        if (value != null) {
                          onBlockIfEnergyInsufficientChanged(value);
                        }
                      },
                      controlAffinity: ListTileControlAffinity.leading,
                      contentPadding: EdgeInsets.zero,
                      title: const Text('Block when energy is insufficient'),
                    ),
                ],
              ),
            ),
    );
  }
}

class TransferPreviewCard extends StatelessWidget {
  const TransferPreviewCard({super.key, required this.preview});

  final TransferPreview preview;

  @override
  Widget build(BuildContext context) {
    final resourceStatus = preview.resourceStatus;
    return ListTile(
      leading: const Icon(Icons.receipt_long),
      title: Text(
        '${preview.amount} ${preview.assetSymbol} to ${preview.toAddress}',
      ),
      subtitle: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('Fee: ${preview.feeEstimate} · ${preview.rpcUrl}'),
          if (resourceStatus != null) ...[
            Text(
              'Energy: ${resourceStatus.energyAvailable} / ${resourceStatus.energyRequired}'
              '${resourceStatus.hasEnoughEnergy ? '' : ' (insufficient)'}',
            ),
            Text('Bandwidth: ${resourceStatus.bandwidthAvailable}'),
            Text(
              'TRX reserve: ${_formatSun(resourceStatus.trxBalanceSun)} / '
              '${_formatSun(resourceStatus.trxFeeReserveRequiredSun)}',
            ),
          ],
        ],
      ),
    );
  }
}

String _formatSun(int sun) {
  final whole = sun ~/ 1000000;
  final fraction = (sun % 1000000).toString().padLeft(6, '0');
  final trimmedFraction = fraction.replaceFirst(RegExp(r'0+$'), '');
  if (trimmedFraction.isEmpty) {
    return '$whole TRX';
  }
  return '$whole.$trimmedFraction TRX';
}
