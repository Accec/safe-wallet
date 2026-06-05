import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_wallet/main.dart';

void main() {
  testWidgets('wallet app starts on setup screen', (tester) async {
    await tester.pumpWidget(const WalletApp());
    await tester.pumpAndSettle();
    expect(find.byType(TextField), findsOneWidget);
    expect(find.text('Create password'), findsOneWidget);
  });

  testWidgets('master password setup opens wallet home', (tester) async {
    await tester.pumpWidget(const WalletApp());
    await tester.pumpAndSettle();
    await tester.enterText(find.byType(TextField), 'password');
    await tester.tap(find.text('Create password'));
    await tester.pumpAndSettle();

    expect(find.text('Wallets'), findsWidgets);
    expect(find.byTooltip('Create wallet'), findsOneWidget);
  });

  testWidgets('wallet home uses bottom navigation on narrow screens', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    await tester.pumpWidget(const WalletApp());
    await tester.pumpAndSettle();
    await tester.enterText(find.byType(TextField), 'password');
    await tester.tap(find.text('Create password'));
    await tester.pumpAndSettle();

    expect(find.byType(NavigationBar), findsOneWidget);
  });
}
