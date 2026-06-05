import 'dart:async';

import 'package:flutter/material.dart';

import 'constants.dart';
import 'motion_preferences.dart';

class AnimatedPageStack extends StatefulWidget {
  AnimatedPageStack({
    super.key,
    required this.index,
    required this.children,
    this.duration = safeWalletPageMotionDuration,
  }) : assert(children.isNotEmpty),
       assert(index >= 0 && index < children.length);

  final int index;
  final List<Widget> children;
  final Duration duration;

  @override
  State<AnimatedPageStack> createState() => _AnimatedPageStackState();
}

class _AnimatedPageStackState extends State<AnimatedPageStack> {
  Timer? _settleTimer;
  int? _previousIndex;

  @override
  void didUpdateWidget(covariant AnimatedPageStack oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.index != widget.index) {
      _settleTimer?.cancel();
      _settleTimer = null;
      _previousIndex = _isValidPreviousIndex(oldWidget.index)
          ? oldWidget.index
          : null;
      if (_previousIndex != null) {
        _settleTimer = Timer(widget.duration, () {
          if (mounted) {
            setState(() {
              _previousIndex = null;
            });
          }
        });
      }
    }
    if (_previousIndex != null && !_isValidPreviousIndex(_previousIndex!)) {
      _settleTimer?.cancel();
      _settleTimer = null;
      _previousIndex = null;
    }
  }

  @override
  void dispose() {
    _settleTimer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (motionDisabled(context)) {
      return Stack(
        fit: StackFit.expand,
        children: [
          KeyedSubtree(
            key: ValueKey('safe-wallet-page-slot-${widget.index}'),
            child: widget.children[widget.index],
          ),
        ],
      );
    }
    final previousIndex =
        _previousIndex != null && _isValidPreviousIndex(_previousIndex!)
        ? _previousIndex
        : null;
    return Stack(
      fit: StackFit.expand,
      children: [
        if (previousIndex != null)
          _AnimatedPageSlot(
            key: ValueKey('safe-wallet-page-slot-$previousIndex'),
            active: false,
            visible: true,
            duration: safeWalletExitMotionDuration,
            curve: safeWalletExitMotionCurve,
            offset: safeWalletPageExitOffset,
            child: widget.children[previousIndex],
          ),
        _AnimatedPageSlot(
          key: ValueKey('safe-wallet-page-slot-${widget.index}'),
          active: true,
          visible: true,
          duration: widget.duration,
          curve: safeWalletEmphasizedMotionCurve,
          offset: safeWalletPageOffset,
          child: widget.children[widget.index],
        ),
      ],
    );
  }

  bool _isValidPreviousIndex(int index) {
    return index >= 0 &&
        index < widget.children.length &&
        index != widget.index;
  }
}

class _AnimatedPageSlot extends StatelessWidget {
  const _AnimatedPageSlot({
    super.key,
    required this.active,
    required this.visible,
    required this.duration,
    required this.curve,
    required this.offset,
    required this.child,
  });

  final bool active;
  final bool visible;
  final Duration duration;
  final Curve curve;
  final Offset offset;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Offstage(
      offstage: !visible,
      child: IgnorePointer(
        ignoring: !active,
        child: ExcludeSemantics(
          excluding: !active,
          child: AnimatedOpacity(
            opacity: active ? 1 : 0,
            duration: duration,
            curve: curve,
            child: AnimatedSlide(
              offset: active ? Offset.zero : offset,
              duration: duration,
              curve: curve,
              child: TickerMode(enabled: active, child: child),
            ),
          ),
        ),
      ),
    );
  }
}
