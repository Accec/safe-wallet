import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_wallet/src/features/assets/presentation/assets_dialogs.dart';
import 'package:flutter_wallet/src/wallet_api.dart';

void main() {
  testWidgets('add token dialog keeps native validation errors visible', (
    tester,
  ) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Builder(
          builder: (context) {
            return FilledButton(
              onPressed: () => showAddTokenDialog(
                context: context,
                selectedChain: 'ethereum',
                onAddCustomToken:
                    ({
                      required chain,
                      required contractAddress,
                      required tokenName,
                    }) async {
                      throw const WalletApiException('Invalid token contract');
                    },
              ),
              child: const Text('Open'),
            );
          },
        ),
      ),
    );

    await tester.tap(find.text('Open'));
    await tester.pumpAndSettle();
    await tester.enterText(_textField('Token name'), 'USDT');
    await tester.enterText(_textField('Contract address'), 'not-a-contract');
    await tester.tap(find.widgetWithText(FilledButton, 'Add token'));
    await tester.pumpAndSettle();

    expect(find.text('Invalid token contract'), findsOneWidget);
    expect(_textField('Contract address'), findsOneWidget);
  });
}

Finder _textField(String label) {
  return find.byWidgetPredicate(
    (widget) => widget is TextField && widget.decoration?.labelText == label,
    description: 'TextField with label $label',
  );
}
