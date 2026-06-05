import 'package:flutter/material.dart';

import '../../../core/ui/screen_motion.dart';
import '../../../models.dart';
import 'activity_row.dart';

class ActivityList extends StatelessWidget {
  const ActivityList({
    super.key,
    required this.activity,
    this.onOpenActivityUrl,
  });

  final List<ActivitySummary> activity;
  final ValueChanged<Uri>? onOpenActivityUrl;

  @override
  Widget build(BuildContext context) {
    return ListView(padding: const EdgeInsets.all(16), children: _buildRows());
  }

  List<Widget> _buildRows() {
    if (activity.isEmpty) {
      return const [
        StaggeredListItem(
          index: 0,
          child: ListTile(
            leading: Icon(Icons.history),
            title: Text('No indexed activity yet'),
            subtitle: Text('Transfers and token activity appear here.'),
          ),
        ),
      ];
    }

    return [
      for (var index = 0; index < activity.length; index++)
        StaggeredListItem(
          index: index,
          child: ActivityRow(
            record: activity[index],
            onOpenActivityUrl: onOpenActivityUrl,
          ),
        ),
    ];
  }
}
