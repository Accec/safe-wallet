import 'package:flutter/material.dart';

import '../../../core/ui/chain_selector.dart';
import '../../../models.dart';
import 'activity_amount_label.dart';
import 'activity_details.dart';
import 'activity_labels.dart';
import 'activity_status_chip.dart';

class ActivityTextColumn extends StatelessWidget {
  const ActivityTextColumn({
    super.key,
    required this.record,
    required this.details,
    required this.compact,
  });

  final ActivitySummary record;
  final ActivityDetails details;
  final bool compact;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Expanded(
              child: Text(
                details.title,
                overflow: TextOverflow.ellipsis,
                style: const TextStyle(
                  fontSize: 15,
                  fontWeight: FontWeight.w800,
                  color: Color(0xff111827),
                ),
              ),
            ),
            const SizedBox(width: 8),
            ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 120),
              child: ActivityStatusChip(status: record.status),
            ),
          ],
        ),
        if (compact) ...[
          const SizedBox(height: 6),
          ActivityAmountLabel(details: details, textAlign: TextAlign.left),
        ],
        const SizedBox(height: 6),
        Wrap(
          spacing: 8,
          runSpacing: 4,
          crossAxisAlignment: WrapCrossAlignment.center,
          children: [
            Text(
              chainLabel(record.chain),
              style: const TextStyle(
                color: Color(0xff4b5563),
                fontSize: 12,
                fontWeight: FontWeight.w600,
              ),
            ),
            Text(
              shortActivityHash(record.txHash),
              style: const TextStyle(
                color: Color(0xff6b7280),
                fontSize: 12,
                fontFeatures: [FontFeature.tabularFigures()],
              ),
            ),
          ],
        ),
      ],
    );
  }
}
