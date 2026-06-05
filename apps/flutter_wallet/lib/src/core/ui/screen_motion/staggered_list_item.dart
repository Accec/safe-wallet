import 'dart:async';

import 'package:flutter/material.dart';

import 'constants.dart';
import 'motion_preferences.dart';

class StaggeredListItem extends StatefulWidget {
  const StaggeredListItem({
    super.key,
    required this.index,
    required this.child,
    this.duration = safeWalletMotionDuration,
  });

  final int index;
  final Widget child;
  final Duration duration;

  @override
  State<StaggeredListItem> createState() => _StaggeredListItemState();
}

class _StaggeredListItemState extends State<StaggeredListItem> {
  Timer? _timer;
  bool _started = false;
  bool _visible = false;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    if (_started) {
      return;
    }
    _started = true;
    if (motionDisabled(context)) {
      _visible = true;
      return;
    }
    _timer = Timer(safeWalletListDelay(widget.index), () {
      if (mounted) {
        setState(() {
          _visible = true;
        });
      }
    });
  }

  @override
  void dispose() {
    _timer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (motionDisabled(context)) {
      return widget.child;
    }
    return AnimatedOpacity(
      opacity: _visible ? 1 : 0,
      duration: widget.duration,
      curve: safeWalletMotionCurve,
      child: AnimatedSlide(
        offset: _visible ? Offset.zero : safeWalletListOffset,
        duration: widget.duration,
        curve: safeWalletMotionCurve,
        child: widget.child,
      ),
    );
  }
}
