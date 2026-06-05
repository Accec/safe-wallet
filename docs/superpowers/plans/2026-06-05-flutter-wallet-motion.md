# Flutter Wallet Motion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Improve Safe Wallet's Flutter animation system with smoother page handoffs, layer-based screen entrances, and capped list stagger motion.

**Architecture:** Keep animation behavior centralized in `apps/flutter_wallet/lib/src/core/ui/screen_motion`. Add semantic motion tokens, then update existing wrapper widgets so feature screens express motion through small reusable APIs instead of local animation details. Apply layer delays only to login/loading screens where the visual hierarchy is clear.

**Tech Stack:** Flutter, Dart, `flutter_test`, existing `screen_motion` widgets, GSAP motion principles translated to Flutter transform/opacity patterns.

---

### Task 1: Motion Widget Regression Tests

**Files:**
- Create: `apps/flutter_wallet/test/screen_motion_test.dart`
- Modify: none

- [ ] **Step 1: Write the failing tests**

Create `apps/flutter_wallet/test/screen_motion_test.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_wallet/src/app/status_loading_screen.dart';
import 'package:flutter_wallet/src/core/ui/screen_motion.dart';
import 'package:flutter_wallet/src/features/session/presentation/unlock_screen_body.dart';

void main() {
  test('safeWalletListDelay caps long list staggers', () {
    expect(safeWalletListDelay(-4), Duration.zero);
    expect(safeWalletListDelay(0), Duration.zero);
    expect(
      safeWalletListDelay(safeWalletMaxListStaggerIndex + 20),
      safeWalletListDelay(safeWalletMaxListStaggerIndex),
    );
  });

  test('safeWalletLayerDelay creates predictable layer offsets', () {
    expect(safeWalletLayerDelay(-1), Duration.zero);
    expect(safeWalletLayerDelay(0), Duration.zero);
    expect(safeWalletLayerDelay(1), safeWalletLayerDelayStep);
    expect(
      safeWalletLayerDelay(2),
      Duration(milliseconds: safeWalletLayerDelayStep.inMilliseconds * 2),
    );
  });

  testWidgets('MotionEntrance waits for its configured delay', (tester) async {
    await tester.pumpWidget(
      _host(
        MotionEntrance(
          delay: safeWalletLayerDelay(1),
          child: const Text('Delayed content'),
        ),
      ),
    );

    expect(_animatedOpacity(tester), 0);

    await tester.pump(safeWalletLayerDelay(1) - const Duration(milliseconds: 1));
    expect(_animatedOpacity(tester), 0);

    await tester.pump(const Duration(milliseconds: 1));
    expect(_animatedOpacity(tester), 1);
  });

  testWidgets('MotionEntrance renders immediately when animations are disabled', (
    tester,
  ) async {
    await tester.pumpWidget(
      _host(
        MotionEntrance(
          delay: const Duration(seconds: 5),
          child: const Text('Static content'),
        ),
        disableAnimations: true,
      ),
    );

    expect(find.text('Static content'), findsOneWidget);
    expect(find.byType(AnimatedOpacity), findsNothing);
  });

  testWidgets('AnimatedPageStack gives the incoming page visual priority', (
    tester,
  ) async {
    await tester.pumpWidget(_pageStack(index: 0));

    expect(find.text('Wallets'), findsOneWidget);
    expect(find.text('Assets'), findsNothing);

    await tester.pumpWidget(_pageStack(index: 1));
    await tester.pump();

    final slots = tester
        .widgetList<ExcludeSemantics>(find.byType(ExcludeSemantics))
        .toList();
    expect(slots.length, 2);
    expect(slots.first.excluding, isTrue);
    expect(slots.last.excluding, isFalse);
    expect(find.text('Assets'), findsOneWidget);

    await tester.pump(safeWalletPageMotionDuration);
    await tester.pump();

    expect(find.text('Assets'), findsOneWidget);
    expect(find.text('Wallets'), findsNothing);
  });

  testWidgets('StaggeredListItem uses the capped delay before revealing', (
    tester,
  ) async {
    await tester.pumpWidget(
      _host(
        StaggeredListItem(
          index: safeWalletMaxListStaggerIndex + 30,
          child: const Text('Late row'),
        ),
      ),
    );

    expect(_animatedOpacity(tester), 0);

    await tester.pump(
      safeWalletListDelay(safeWalletMaxListStaggerIndex) -
          const Duration(milliseconds: 1),
    );
    expect(_animatedOpacity(tester), 0);

    await tester.pump(const Duration(milliseconds: 1));
    expect(_animatedOpacity(tester), 1);
  });

  testWidgets('UnlockScreenBody exposes layered entrance delays', (tester) async {
    final controller = TextEditingController();
    addTearDown(controller.dispose);

    await tester.pumpWidget(
      _host(
        UnlockScreenBody(
          initialized: true,
          biometricEnabled: true,
          hasBiometricUnlock: true,
          passwordController: controller,
          busy: false,
          biometricBusy: false,
          error: null,
          onSubmit: () {},
          onBiometricUnlock: () {},
        ),
      ),
    );

    final delays = tester
        .widgetList<MotionEntrance>(find.byType(MotionEntrance))
        .map((widget) => widget.delay)
        .toList();

    expect(delays, [
      Duration.zero,
      safeWalletLayerDelay(1),
      safeWalletLayerDelay(2),
    ]);
  });

  testWidgets('StatusLoadingScreen separates brand and progress motion layers', (
    tester,
  ) async {
    await tester.pumpWidget(_host(const StatusLoadingScreen()));

    final delays = tester
        .widgetList<MotionEntrance>(find.byType(MotionEntrance))
        .map((widget) => widget.delay)
        .toList();

    expect(delays, [Duration.zero, safeWalletLayerDelay(1)]);
  });
}

Widget _host(Widget child, {bool disableAnimations = false}) {
  return MaterialApp(
    home: MediaQuery(
      data: MediaQueryData(disableAnimations: disableAnimations),
      child: child,
    ),
  );
}

Widget _pageStack({required int index}) {
  return _host(
    SizedBox(
      width: 320,
      height: 480,
      child: AnimatedPageStack(
        index: index,
        children: const [
          Text('Wallets'),
          Text('Assets'),
        ],
      ),
    ),
  );
}

double _animatedOpacity(WidgetTester tester) {
  return tester.widget<AnimatedOpacity>(find.byType(AnimatedOpacity)).opacity;
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cd apps/flutter_wallet
flutter test test/screen_motion_test.dart
```

Expected: FAIL because `safeWalletMaxListStaggerIndex`, `safeWalletLayerDelay`, `safeWalletLayerDelayStep`, `safeWalletPageMotionDuration`, and `MotionEntrance.delay` do not exist yet, and because the screens do not yet expose layered `MotionEntrance` widgets.

### Task 2: Shared Motion Tokens

**Files:**
- Modify: `apps/flutter_wallet/lib/src/core/ui/screen_motion/constants.dart`
- Test: `apps/flutter_wallet/test/screen_motion_test.dart`

- [ ] **Step 1: Implement semantic motion constants**

Replace `apps/flutter_wallet/lib/src/core/ui/screen_motion/constants.dart` with:

```dart
import 'package:flutter/material.dart';

const safeWalletQuickMotionDuration = Duration(milliseconds: 160);
const safeWalletMotionDuration = Duration(milliseconds: 240);
const safeWalletLongMotionDuration = Duration(milliseconds: 340);
const safeWalletPageMotionDuration = Duration(milliseconds: 300);
const safeWalletExitMotionDuration = Duration(milliseconds: 150);

const safeWalletMotionCurve = Curves.easeOutCubic;
const safeWalletEmphasizedMotionCurve = Curves.easeOutQuart;
const safeWalletExitMotionCurve = Curves.easeInCubic;

const safeWalletEntranceOffset = Offset(0, 0.032);
const safeWalletPageOffset = Offset(0, 0.018);
const safeWalletPageExitOffset = Offset(0, -0.006);
const safeWalletSwitcherOffset = Offset(0, 0.02);
const safeWalletListOffset = Offset(0, 0.026);

const safeWalletLayerDelayStep = Duration(milliseconds: 55);
const safeWalletListStaggerStep = Duration(milliseconds: 28);
const safeWalletMaxListStaggerIndex = 8;

Duration safeWalletLayerDelay(int layer) {
  final boundedLayer = layer < 0 ? 0 : layer;
  return Duration(
    milliseconds: boundedLayer * safeWalletLayerDelayStep.inMilliseconds,
  );
}

Duration safeWalletListDelay(int index) {
  final boundedIndex = index < 0
      ? 0
      : index > safeWalletMaxListStaggerIndex
      ? safeWalletMaxListStaggerIndex
      : index;
  return Duration(
    milliseconds: boundedIndex * safeWalletListStaggerStep.inMilliseconds,
  );
}
```

- [ ] **Step 2: Run focused tests to verify remaining failures**

Run:

```bash
cd apps/flutter_wallet
flutter test test/screen_motion_test.dart
```

Expected: FAIL only on missing widget/API behavior, not on missing constants.

### Task 3: Shared Motion Components

**Files:**
- Modify: `apps/flutter_wallet/lib/src/core/ui/screen_motion/motion_entrance.dart`
- Modify: `apps/flutter_wallet/lib/src/core/ui/screen_motion/motion_switcher.dart`
- Modify: `apps/flutter_wallet/lib/src/core/ui/screen_motion/animated_page_stack.dart`
- Modify: `apps/flutter_wallet/lib/src/core/ui/screen_motion/staggered_list_item.dart`
- Test: `apps/flutter_wallet/test/screen_motion_test.dart`

- [ ] **Step 1: Add delay support to `MotionEntrance`**

Replace `apps/flutter_wallet/lib/src/core/ui/screen_motion/motion_entrance.dart` with:

```dart
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
```

- [ ] **Step 2: Tune `MotionSwitcher` with shared tokens**

Replace `apps/flutter_wallet/lib/src/core/ui/screen_motion/motion_switcher.dart` with:

```dart
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
```

- [ ] **Step 3: Tune `AnimatedPageStack` page handoff behavior**

Replace `apps/flutter_wallet/lib/src/core/ui/screen_motion/animated_page_stack.dart` with:

```dart
import 'dart:async';

import 'package:flutter/material.dart';

import 'constants.dart';
import 'motion_preferences.dart';

class AnimatedPageStack extends StatefulWidget {
  const AnimatedPageStack({
    super.key,
    required this.index,
    required this.children,
    this.duration = safeWalletPageMotionDuration,
  });

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
      _previousIndex = oldWidget.index;
      _settleTimer = Timer(widget.duration, () {
        if (mounted) {
          setState(() {
            _previousIndex = null;
          });
        }
      });
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
      return IndexedStack(index: widget.index, children: widget.children);
    }
    return Stack(
      fit: StackFit.expand,
      children: [
        for (var i = 0; i < widget.children.length; i++)
          if (i != widget.index)
            _AnimatedPageSlot(
              key: ValueKey('safe-wallet-page-slot-$i'),
              active: false,
              visible: i == _previousIndex,
              duration: safeWalletExitMotionDuration,
              curve: safeWalletExitMotionCurve,
              offset: i == _previousIndex
                  ? safeWalletPageExitOffset
                  : safeWalletPageOffset,
              child: widget.children[i],
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
}

class _AnimatedPageSlot extends StatelessWidget {
  const _AnimatedPageSlot({
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
```

- [ ] **Step 4: Tune `StaggeredListItem`**

Replace `apps/flutter_wallet/lib/src/core/ui/screen_motion/staggered_list_item.dart` with:

```dart
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
```

- [ ] **Step 5: Run focused tests**

Run:

```bash
cd apps/flutter_wallet
flutter test test/screen_motion_test.dart
```

Expected: FAIL only on the layered screen assertions, because `UnlockScreenBody` and `StatusLoadingScreen` are not updated yet.

### Task 4: Apply Layered Screen Entrances

**Files:**
- Modify: `apps/flutter_wallet/lib/src/features/session/presentation/unlock_screen_body.dart`
- Modify: `apps/flutter_wallet/lib/src/app/status_loading_screen.dart`
- Test: `apps/flutter_wallet/test/screen_motion_test.dart`

- [ ] **Step 1: Split unlock screen into motion layers**

Replace the `MotionEntrance` wrapper inside `UnlockScreenBody.build()` with layer-specific wrappers:

```dart
child: Column(
  mainAxisSize: MainAxisSize.min,
  crossAxisAlignment: CrossAxisAlignment.stretch,
  children: [
    MotionEntrance(
      child: UnlockBrandHeader(initialized: initialized),
    ),
    const SizedBox(height: 24),
    MotionEntrance(
      delay: safeWalletLayerDelay(1),
      child: UnlockPasswordForm(
        controller: passwordController,
        initialized: initialized,
        busy: busy,
        biometricBusy: biometricBusy,
        error: error,
        onSubmit: onSubmit,
      ),
    ),
    MotionEntrance(
      delay: safeWalletLayerDelay(2),
      child: UnlockBiometricButton(
        initialized: initialized,
        biometricEnabled: biometricEnabled,
        hasBiometricUnlock: hasBiometricUnlock,
        busy: busy,
        biometricBusy: biometricBusy,
        onBiometricUnlock: onBiometricUnlock,
      ),
    ),
  ],
),
```

The resulting `Padding` subtree should no longer have a single `MotionEntrance` around the entire `Column`.

- [ ] **Step 2: Split loading screen into brand and progress layers**

Replace the `MotionEntrance` wrapper inside `StatusLoadingScreen.build()` with:

```dart
child: Column(
  mainAxisSize: MainAxisSize.min,
  children: [
    MotionEntrance(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          ClipRRect(
            borderRadius: BorderRadius.circular(18),
            child: Image.asset(
              'assets/branding/safe_wallet_icon.png',
              width: 72,
              height: 72,
            ),
          ),
          const SizedBox(height: 16),
          Text(
            'Safe Wallet',
            style: Theme.of(context).textTheme.headlineMedium,
          ),
        ],
      ),
    ),
    const SizedBox(height: 20),
    MotionEntrance(
      delay: safeWalletLayerDelay(1),
      child: const SizedBox(width: 160, child: LinearProgressIndicator()),
    ),
  ],
),
```

- [ ] **Step 3: Run focused tests to verify pass**

Run:

```bash
cd apps/flutter_wallet
flutter test test/screen_motion_test.dart
```

Expected: PASS.

### Task 5: Full Flutter Verification

**Files:**
- Test only

- [ ] **Step 1: Run analyzer**

Run:

```bash
cd apps/flutter_wallet
flutter analyze
```

Expected: exit 0 with no analyzer errors.

- [ ] **Step 2: Run all Flutter tests**

Run:

```bash
cd apps/flutter_wallet
flutter test
```

Expected: all tests pass.

- [ ] **Step 3: Review changed files**

Run:

```bash
git diff -- apps/flutter_wallet/lib/src/core/ui/screen_motion/constants.dart apps/flutter_wallet/lib/src/core/ui/screen_motion/motion_entrance.dart apps/flutter_wallet/lib/src/core/ui/screen_motion/motion_switcher.dart apps/flutter_wallet/lib/src/core/ui/screen_motion/animated_page_stack.dart apps/flutter_wallet/lib/src/core/ui/screen_motion/staggered_list_item.dart apps/flutter_wallet/lib/src/features/session/presentation/unlock_screen_body.dart apps/flutter_wallet/lib/src/app/status_loading_screen.dart apps/flutter_wallet/test/screen_motion_test.dart
```

Expected: diff only contains motion token updates, shared animation wrapper changes, layered entrance wrappers, and the new motion tests.

### Task 6: Commit Implementation

**Files:**
- Stage only files modified by Tasks 1-4.

- [ ] **Step 1: Stage implementation files**

Run:

```bash
git add apps/flutter_wallet/lib/src/core/ui/screen_motion/constants.dart \
  apps/flutter_wallet/lib/src/core/ui/screen_motion/motion_entrance.dart \
  apps/flutter_wallet/lib/src/core/ui/screen_motion/motion_switcher.dart \
  apps/flutter_wallet/lib/src/core/ui/screen_motion/animated_page_stack.dart \
  apps/flutter_wallet/lib/src/core/ui/screen_motion/staggered_list_item.dart \
  apps/flutter_wallet/lib/src/features/session/presentation/unlock_screen_body.dart \
  apps/flutter_wallet/lib/src/app/status_loading_screen.dart \
  apps/flutter_wallet/test/screen_motion_test.dart
```

Expected: only the motion implementation and test files are staged.

- [ ] **Step 2: Commit implementation**

Run:

```bash
git commit -m "feat: refine flutter wallet motion"
```

Expected: commit succeeds after the focused tests, analyzer, and full Flutter tests have passed.
