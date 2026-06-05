import 'package:flutter/material.dart';

import 'activity_details.dart';

class ActivityAmountLabel extends StatelessWidget {
  const ActivityAmountLabel({
    super.key,
    required this.details,
    this.textAlign = TextAlign.right,
  });

  final ActivityDetails details;
  final TextAlign textAlign;

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 180),
      child: Text(
        details.amountLabel,
        maxLines: 1,
        overflow: TextOverflow.ellipsis,
        textAlign: textAlign,
        style: TextStyle(
          color: details.amountColor,
          fontSize: 15,
          fontWeight: FontWeight.w800,
        ),
      ),
    );
  }
}
