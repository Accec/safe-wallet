import 'package:flutter/material.dart';

import 'activity_details.dart';

class ActivityIcon extends StatelessWidget {
  const ActivityIcon({super.key, required this.details});

  final ActivityDetails details;

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 40,
      height: 40,
      decoration: BoxDecoration(
        color: details.iconBackground,
        borderRadius: BorderRadius.circular(8),
      ),
      child: Icon(details.icon, color: details.iconColor, size: 22),
    );
  }
}
