import 'package:flutter/material.dart';

import 'activity_labels.dart';

class ActivityStatusChip extends StatelessWidget {
  const ActivityStatusChip({super.key, required this.status});

  final String status;

  @override
  Widget build(BuildContext context) {
    final normalized = status.trim().toLowerCase();
    final color = switch (normalized) {
      'confirmed' => const Color(0xff047857),
      'failed' => const Color(0xffb91c1c),
      _ => const Color(0xffb45309),
    };
    final background = switch (normalized) {
      'confirmed' => const Color(0xffecfdf5),
      'failed' => const Color(0xfffff1f2),
      _ => const Color(0xfffffbeb),
    };
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: background,
        borderRadius: BorderRadius.circular(999),
      ),
      child: Text(
        activityTitleCase(status),
        maxLines: 1,
        overflow: TextOverflow.ellipsis,
        style: TextStyle(
          color: color,
          fontSize: 11,
          fontWeight: FontWeight.w800,
        ),
      ),
    );
  }
}
