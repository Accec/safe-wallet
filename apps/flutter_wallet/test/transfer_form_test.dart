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
            onBlockIfEnergyInsufficientChanged: (_) {},
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

  testWidgets('transfer form shows energy status and blocks send by default', (
    tester,
  ) async {
    bool? blockPreference;
    final recipientController = TextEditingController();
    final amountController = TextEditingController();
    addTearDown(recipientController.dispose);
    addTearDown(amountController.dispose);

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: TransferForm(
            assets: const [],
            selectedAssetId: null,
            onAssetChanged: (_) {},
            recipientController: recipientController,
            amountController: amountController,
            scanning: false,
            importing: false,
            transfer: const TransferState(
              preview: TransferPreview(
                chain: 'tron',
                fromAddress: 'TFrom',
                toAddress: 'TTo',
                assetSymbol: 'USDT',
                amount: '2.5',
                feeEstimate: 'TRON resources',
                rpcUrl: 'https://tron-rpc.publicnode.com',
                resourceStatus: TransferResourceStatus(
                  energyAvailable: 4000,
                  energyRequired: 8624,
                  bandwidthAvailable: 74,
                  bandwidthRequired: 350,
                  trxBalanceSun: 42012,
                  trxFeeReserveRequiredSun: 350000,
                  canSendWithoutBurningTrx: false,
                ),
              ),
            ),
            onScanQr: () {},
            onImportQrImage: () {},
            onPreviewTransfer: () {},
            onSendTransfer: () {},
            onBlockIfEnergyInsufficientChanged: (value) {
              blockPreference = value;
            },
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.textContaining('Energy: 4000 / 8624'), findsOneWidget);
    expect(find.textContaining('Bandwidth: 74 / 350'), findsOneWidget);
    expect(
      find.textContaining('TRX fees: 0.042012 TRX / 0.35 TRX'),
      findsOneWidget,
    );
    expect(
      find.widgetWithText(
        CheckboxListTile,
        'Block when energy is insufficient',
      ),
      findsOneWidget,
    );
    expect(find.widgetWithText(FilledButton, 'Send'), findsNothing);

    await tester.tap(find.byType(CheckboxListTile));
    await tester.pumpAndSettle();

    expect(blockPreference, false);
  });

  testWidgets('transfer form allows send when energy is sufficient', (
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
            assets: const [],
            selectedAssetId: null,
            onAssetChanged: (_) {},
            recipientController: recipientController,
            amountController: amountController,
            scanning: false,
            importing: false,
            transfer: const TransferState(
              preview: TransferPreview(
                chain: 'tron',
                fromAddress: 'TFrom',
                toAddress: 'TTo',
                assetSymbol: 'USDT',
                amount: '2.5',
                feeEstimate: 'TRON resources',
                rpcUrl: 'https://tron-rpc.publicnode.com',
                resourceStatus: TransferResourceStatus(
                  energyAvailable: 69969,
                  energyRequired: 8624,
                  bandwidthAvailable: 600,
                  bandwidthRequired: 350,
                  trxBalanceSun: 42012,
                  trxFeeReserveRequiredSun: 0,
                  canSendWithoutBurningTrx: true,
                ),
              ),
            ),
            onScanQr: () {},
            onImportQrImage: () {},
            onPreviewTransfer: () {},
            onSendTransfer: () {},
            onBlockIfEnergyInsufficientChanged: (_) {},
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.textContaining('Energy: 69969 / 8624'), findsOneWidget);
    expect(find.widgetWithText(FilledButton, 'Send'), findsOneWidget);
  });

  testWidgets(
    'transfer form hides send when bandwidth burn exceeds TRX balance',
    (tester) async {
      final recipientController = TextEditingController();
      final amountController = TextEditingController();
      addTearDown(recipientController.dispose);
      addTearDown(amountController.dispose);

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: TransferForm(
              assets: const [],
              selectedAssetId: null,
              onAssetChanged: (_) {},
              recipientController: recipientController,
              amountController: amountController,
              scanning: false,
              importing: false,
              transfer: const TransferState(
                preview: TransferPreview(
                  chain: 'tron',
                  fromAddress: 'TFrom',
                  toAddress: 'TTo',
                  assetSymbol: 'USDT',
                  amount: '2.5',
                  feeEstimate: 'TRON resources',
                  rpcUrl: 'https://tron-rpc.publicnode.com',
                  resourceStatus: TransferResourceStatus(
                    energyAvailable: 69969,
                    energyRequired: 8624,
                    bandwidthAvailable: 103,
                    bandwidthRequired: 350,
                    trxBalanceSun: 40000,
                    trxFeeReserveRequiredSun: 350000,
                    canSendWithoutBurningTrx: false,
                  ),
                ),
              ),
              onScanQr: () {},
              onImportQrImage: () {},
              onPreviewTransfer: () {},
              onSendTransfer: () {},
              onBlockIfEnergyInsufficientChanged: (_) {},
            ),
          ),
        ),
      );
      await tester.pumpAndSettle();

      expect(find.textContaining('Energy: 69969 / 8624'), findsOneWidget);
      expect(find.textContaining('Bandwidth: 103 / 350'), findsOneWidget);
      expect(
        find.textContaining('TRX fees: 0.04 TRX / 0.35 TRX'),
        findsOneWidget,
      );
      expect(find.widgetWithText(FilledButton, 'Send'), findsNothing);
    },
  );
}

Finder _textField(String label) {
  return find.byWidgetPredicate(
    (widget) => widget is TextField && widget.decoration?.labelText == label,
    description: 'TextField with label $label',
  );
}
