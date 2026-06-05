import 'dart:async';

import 'package:flutter/material.dart';

import 'constants.dart';
import 'motion_preferences.dart';

class MotionEntrance extends StatefulWidget {
  const MotionEntrance({
    super.key,
    required this.child,
    this.duration = safeWalletLongMotionDuration,
    this.delay = Duration.zero,
    this.offset = safeWalletEntranceOffset,
  });

  final Widget child;
  final Duration duration;
  final Duration delay;
  final Offset offset;

  @override
  State<MotionEntrance> createState() => _MotionEntranceState();
}

class _MotionEntranceState extends State<MotionEntrance> {
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
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) {
        return;
      }
      if (widget.delay == Duration.zero) {
        setState(() {
          _visible = true;
        });
        return;
      }
      _timer = Timer(widget.delay, () {
        if (mounted) {
          setState(() {
            _visible = true;
          });
        }
      });
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
        offset: _visible ? Offset.zero : widget.offset,
        duration: widget.duration,
        curve: safeWalletEmphasizedMotionCurve,
        child: widget.child,
      ),
    );
  }
}
