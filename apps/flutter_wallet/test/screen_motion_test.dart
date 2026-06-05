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

    await tester.pump(
      safeWalletLayerDelay(1) - const Duration(milliseconds: 1),
    );
    expect(_animatedOpacity(tester), 0);

    await tester.pump(const Duration(milliseconds: 1));
    expect(_animatedOpacity(tester), 1);
  });

  testWidgets(
    'MotionEntrance renders immediately when animations are disabled',
    (tester) async {
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
    },
  );

  testWidgets('AnimatedPageStack gives the incoming page visual priority', (
    tester,
  ) async {
    await tester.pumpWidget(_pageStack(index: 0));

    expect(find.text('Wallets'), findsOneWidget);
    expect(find.text('Assets'), findsNothing);

    await tester.pumpWidget(_pageStack(index: 1));
    await tester.pump();

    final slots = tester
        .widgetList<ExcludeSemantics>(
          find.descendant(
            of: find.byType(AnimatedPageStack),
            matching: find.byType(ExcludeSemantics),
          ),
        )
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

  testWidgets('AnimatedPageStack does not start inactive page entrances', (
    tester,
  ) async {
    await tester.pumpWidget(_delayedPageStack(index: 0));

    expect(find.text('Wallets'), findsOneWidget);
    expect(find.text('Delayed assets'), findsNothing);

    await tester.pump(
      safeWalletLayerDelay(1) + const Duration(milliseconds: 1),
    );
    expect(find.text('Delayed assets'), findsNothing);

    await tester.pumpWidget(_delayedPageStack(index: 1));
    await tester.pump();

    expect(find.text('Delayed assets'), findsOneWidget);
    expect(_motionEntranceOpacity(tester), 0);

    await tester.pump(
      safeWalletLayerDelay(1) - const Duration(milliseconds: 1),
    );
    expect(_motionEntranceOpacity(tester), 0);

    await tester.pump(const Duration(milliseconds: 1));
    expect(_motionEntranceOpacity(tester), 1);
  });

  test('AnimatedPageStack validates children and index', () {
    expect(
      () => AnimatedPageStack(index: 0, children: const []),
      throwsAssertionError,
    );
    expect(
      () => AnimatedPageStack(index: -1, children: const [Text('Wallets')]),
      throwsAssertionError,
    );
    expect(
      () => AnimatedPageStack(index: 1, children: const [Text('Wallets')]),
      throwsAssertionError,
    );
  });

  testWidgets('AnimatedPageStack routes taps to the active page', (
    tester,
  ) async {
    var walletTaps = 0;
    var assetTaps = 0;

    await tester.pumpWidget(
      _buttonPageStack(
        index: 0,
        onWalletTap: () => walletTaps++,
        onAssetTap: () => assetTaps++,
      ),
    );

    final buttonCenter = tester.getCenter(find.text('Wallet action'));
    await tester.tapAt(buttonCenter);
    expect(walletTaps, 1);
    expect(assetTaps, 0);

    await tester.pumpWidget(
      _buttonPageStack(
        index: 1,
        onWalletTap: () => walletTaps++,
        onAssetTap: () => assetTaps++,
      ),
    );
    await tester.pump();

    await tester.tapAt(buttonCenter);
    expect(walletTaps, 1);
    expect(assetTaps, 1);
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

  testWidgets('UnlockScreenBody exposes layered entrance delays', (
    tester,
  ) async {
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

  testWidgets(
    'StatusLoadingScreen separates brand and progress motion layers',
    (tester) async {
      await tester.pumpWidget(_host(const StatusLoadingScreen()));

      final delays = tester
          .widgetList<MotionEntrance>(find.byType(MotionEntrance))
          .map((widget) => widget.delay)
          .toList();

      expect(delays, [Duration.zero, safeWalletLayerDelay(1)]);
    },
  );
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
        children: const [Text('Wallets'), Text('Assets')],
      ),
    ),
  );
}

Widget _delayedPageStack({required int index}) {
  return _host(
    SizedBox(
      width: 320,
      height: 480,
      child: AnimatedPageStack(
        index: index,
        children: [
          const Text('Wallets'),
          MotionEntrance(
            delay: safeWalletLayerDelay(1),
            child: const Text('Delayed assets'),
          ),
        ],
      ),
    ),
  );
}

Widget _buttonPageStack({
  required int index,
  required VoidCallback onWalletTap,
  required VoidCallback onAssetTap,
}) {
  return _host(
    SizedBox(
      width: 320,
      height: 480,
      child: AnimatedPageStack(
        index: index,
        children: [
          Center(
            child: SizedBox(
              width: 220,
              height: 80,
              child: ElevatedButton(
                onPressed: onWalletTap,
                child: const Text('Wallet action'),
              ),
            ),
          ),
          Center(
            child: SizedBox(
              width: 220,
              height: 80,
              child: ElevatedButton(
                onPressed: onAssetTap,
                child: const Text('Asset action'),
              ),
            ),
          ),
        ],
      ),
    ),
  );
}

double _animatedOpacity(WidgetTester tester) {
  return tester.widget<AnimatedOpacity>(find.byType(AnimatedOpacity)).opacity;
}

double _motionEntranceOpacity(WidgetTester tester) {
  return tester
      .widget<AnimatedOpacity>(
        find.descendant(
          of: find.byType(MotionEntrance),
          matching: find.byType(AnimatedOpacity),
        ),
      )
      .opacity;
}
