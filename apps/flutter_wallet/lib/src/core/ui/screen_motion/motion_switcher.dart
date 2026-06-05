import 'package:flutter/material.dart';

import 'constants.dart';
import 'motion_preferences.dart';

class MotionSwitcher extends StatelessWidget {
  const MotionSwitcher({
    super.key,
    required this.child,
    this.duration = safeWalletMotionDuration,
  });

  final Widget child;
  final Duration duration;

  @override
  Widget build(BuildContext context) {
    if (motionDisabled(context)) {
      return child;
    }
    return AnimatedSwitcher(
      duration: duration,
      switchInCurve: safeWalletMotionCurve,
      switchOutCurve: safeWalletExitMotionCurve,
      transitionBuilder: (child, animation) {
        final curved = CurvedAnimation(
          parent: animation,
          curve: safeWalletMotionCurve,
          reverseCurve: safeWalletExitMotionCurve,
        );
        return FadeTransition(
          opacity: curved,
          child: SlideTransition(
            position: Tween<Offset>(
              begin: safeWalletSwitcherOffset,
              end: Offset.zero,
            ).animate(curved),
            child: child,
          ),
        );
      },
      child: child,
    );
  }
}
