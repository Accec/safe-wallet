import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_wallet/src/features/transfers/presentation/transfer_form.dart';
import 'package:flutter_wallet/src/features/transfers/transfer_controller.dart';
import 'package:flutter_wallet/src/models.dart';

void main() {
  testWidgets('transfer form renders asset selector and actions', (
    tester,
  ) async {
    final recipientController = TextEditingController();
    final amountController = TextEditingController();
    addTearDown(recipientController.dispose);
    addTearDown(amountController.dispose);

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: TransferForm(
            assets: const [
              AssetSummary(
                id: 'eth-native',
                chain: 'ethereum',
                symbol: 'ETH',
                name: 'Ethereum',
                decimals: 18,
                kind: 'native',
              ),
            ],
            selectedAssetId: null,
            onAssetChanged: (_) {},
            recipientController: recipientController,
            amountController: amountController,
            scanning: false,
            importing: false,
            transfer: const TransferState(),
            onScanQr: () {},
            onImportQrImage: () {},
            onPreviewTransfer: () {},
            onSendTransfer: () {},
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('ETH · ETHEREUM · 0'), findsOneWidget);
    expect(_textField('Recipient'), findsOneWidget);
    expect(_textField('Amount'), findsOneWidget);
    expect(find.widgetWithText(OutlinedButton, 'Scan QR'), findsOneWidget);
    expect(find.widgetWithText(OutlinedButton, 'Import QR'), findsOneWidget);
    expect(find.widgetWithText(FilledButton, 'Preview'), findsOneWidget);
    expect(find.widgetWithText(FilledButton, 'Send'), findsNothing);
  });
}

Finder _textField(String label) {
  return find.byWidgetPredicate(
    (widget) => widget is TextField && widget.decoration?.labelText == label,
    description: 'TextField with label $label',
  );
}
