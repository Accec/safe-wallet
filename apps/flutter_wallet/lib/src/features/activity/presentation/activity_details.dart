import 'package:flutter/material.dart';

import '../../../models.dart';
import 'activity_labels.dart';
import 'activity_summary_parser.dart';

class ActivityDetails {
  const ActivityDetails({
    required this.title,
    required this.amountLabel,
    required this.amountColor,
    required this.icon,
    required this.iconColor,
    required this.iconBackground,
  });

  final String title;
  final String amountLabel;
  final Color amountColor;
  final IconData icon;
  final Color iconColor;
  final Color iconBackground;

  factory ActivityDetails.from(ActivitySummary record) {
    final parsed = parseActivityTransferSummary(record.summary);
    if (parsed != null) {
      final outgoing = parsed.direction == ActivityTransferDirection.sent;
      final received = parsed.direction == ActivityTransferDirection.received;
      final failed = record.status.toLowerCase() == 'failed';
      return ActivityDetails(
        title: switch (parsed.direction) {
          ActivityTransferDirection.sent => 'Sent',
          ActivityTransferDirection.received => 'Received',
          ActivityTransferDirection.transfer => 'Transfer',
        },
        amountLabel: switch (parsed.direction) {
          ActivityTransferDirection.sent =>
            '-${parsed.amount} ${parsed.symbol}',
          ActivityTransferDirection.received =>
            '+${parsed.amount} ${parsed.symbol}',
          ActivityTransferDirection.transfer =>
            '${parsed.amount} ${parsed.symbol}',
        },
        amountColor: failed
            ? const Color(0xff6b7280)
            : outgoing
            ? const Color(0xff111827)
            : received
            ? const Color(0xff047857)
            : const Color(0xff111827),
        icon: switch (parsed.direction) {
          ActivityTransferDirection.sent => Icons.arrow_upward,
          ActivityTransferDirection.received => Icons.arrow_downward,
          ActivityTransferDirection.transfer => Icons.receipt_long,
        },
        iconColor: outgoing
            ? const Color(0xffb45309)
            : received
            ? const Color(0xff047857)
            : const Color(0xff2563eb),
        iconBackground: outgoing
            ? const Color(0xfffffbeb)
            : received
            ? const Color(0xffecfdf5)
            : const Color(0xffeff6ff),
      );
    }
    return ActivityDetails(
      title: activityKindLabel(record.kind),
      amountLabel: record.summary,
      amountColor: const Color(0xff111827),
      icon: Icons.receipt_long,
      iconColor: const Color(0xff2563eb),
      iconBackground: const Color(0xffeff6ff),
    );
  }
}
