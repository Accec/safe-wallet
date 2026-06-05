import 'package:flutter/material.dart';

import '../../../models.dart';
import 'activity_amount_label.dart';
import 'activity_details.dart';
import 'activity_icon.dart';
import 'activity_scan_url.dart';
import 'activity_text_column.dart';

class ActivityRow extends StatelessWidget {
  const ActivityRow({super.key, required this.record, this.onOpenActivityUrl});

  final ActivitySummary record;
  final ValueChanged<Uri>? onOpenActivityUrl;

  @override
  Widget build(BuildContext context) {
    final details = ActivityDetails.from(record);
    final scanUrl = activityScanUrl(record);
    final onTap = scanUrl == null || onOpenActivityUrl == null
        ? null
        : () => onOpenActivityUrl!(scanUrl);
    return Semantics(
      button: onTap != null,
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(8),
        child: Container(
          margin: const EdgeInsets.only(bottom: 8),
          padding: const EdgeInsets.all(14),
          decoration: BoxDecoration(
            color: Colors.white,
            borderRadius: BorderRadius.circular(8),
            border: Border.all(color: const Color(0xffe5e7eb)),
          ),
          child: LayoutBuilder(
            builder: (context, constraints) {
              final compact = constraints.maxWidth < 420;
              return Row(
                crossAxisAlignment: compact
                    ? CrossAxisAlignment.start
                    : CrossAxisAlignment.center,
                children: [
                  ActivityIcon(details: details),
                  const SizedBox(width: 12),
                  Expanded(
                    child: ActivityTextColumn(
                      record: record,
                      details: details,
                      compact: compact,
                    ),
                  ),
                  if (!compact) ...[
                    const SizedBox(width: 12),
                    ActivityAmountLabel(details: details),
                  ],
                ],
              );
            },
          ),
        ),
      ),
    );
  }
}
