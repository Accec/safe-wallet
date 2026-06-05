import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_wallet/src/core/state/wallet_providers.dart';
import 'package:flutter_wallet/src/features/transfers/presentation/transfer_screen.dart';
import 'package:flutter_wallet/src/models.dart';
import 'package:flutter_wallet/src/wallet_api.dart';

void main() {
  testWidgets('qr scan sends payload and prefills transfer fields', (
    tester,
  ) async {
    final api = _FakeWalletApi(
      parsedPayment: const ParsedPayment(
        chain: 'tron',
        address: 'TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7',
        amount: '12.5',
      ),
    );

    await tester.pumpWidget(
      _transferTestApp(
        api: api,
        home: TransferScreen(scanPayload: () async => 'tron-payment'),
      ),
    );

    await tester.tap(find.text('Scan QR'));
    await tester.pumpAndSettle();

    final recipient = tester.widget<TextField>(find.byType(TextField).first);
    final amount = tester.widget<TextField>(find.byType(TextField).at(1));
    expect(api.lastPayload, 'tron-payment');
    expect(recipient.controller?.text, 'TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7');
    expect(amount.controller?.text, '12.5');
  });

  testWidgets('qr scan shows safe error text', (tester) async {
    await tester.pumpWidget(
      _transferTestApp(
        api: _FakeWalletApi(error: Exception('internal private_key path')),
        home: TransferScreen(scanPayload: () async => 'bad-payload'),
      ),
    );

    await tester.tap(find.text('Scan QR'));
    await tester.pump();

    expect(find.text('Wallet action failed'), findsOneWidget);
    expect(find.textContaining('private_key'), findsNothing);
  });

  testWidgets('qr scan surfaces camera bridge errors', (tester) async {
    await tester.pumpWidget(
      _transferTestApp(
        api: _FakeWalletApi(),
        home: TransferScreen(
          scanPayload: () async {
            throw PlatformException(
              code: 'permission_denied',
              message: 'Camera access was denied.',
            );
          },
        ),
      ),
    );

    await tester.tap(find.text('Scan QR'));
    await tester.pump();

    expect(find.text('Camera access was denied.'), findsOneWidget);
    expect(find.text('Wallet action failed'), findsNothing);
  });

  testWidgets(
    'qr image import sends decoded payload and prefills transfer fields',
    (tester) async {
      final api = _FakeWalletApi(
        parsedPayment: const ParsedPayment(
          chain: 'ethereum',
          address: '0x0000000000000000000000000000000000000000',
          amount: '0.42',
        ),
      );

      await tester.pumpWidget(
        _transferTestApp(
          api: api,
          home: TransferScreen(
            importImagePayload: () async => 'ethereum-payment',
          ),
        ),
      );

      await tester.tap(find.text('Import QR'));
      await tester.pumpAndSettle();

      final recipient = tester.widget<TextField>(find.byType(TextField).first);
      final amount = tester.widget<TextField>(find.byType(TextField).at(1));
      expect(api.lastPayload, 'ethereum-payment');
      expect(
        recipient.controller?.text,
        '0x0000000000000000000000000000000000000000',
      );
      expect(amount.controller?.text, '0.42');
    },
  );

  testWidgets('transfer preview uses the selected asset', (tester) async {
    final api = _FakeWalletApi(
      preview: const TransferPreview(
        chain: 'ethereum',
        fromAddress: '0x1111111111111111111111111111111111111111',
        toAddress: '0x0000000000000000000000000000000000000000',
        assetSymbol: 'USDT',
        amount: '2.5',
        feeEstimate: '~ network fee',
        rpcUrl: 'https://ethereum-rpc.publicnode.com',
      ),
    );

    await tester.pumpWidget(
      _transferTestApp(
        api: api,
        home: TransferScreen(
          wallet: const WalletSummary(id: 'wallet-1', label: 'Primary'),
          assets: const [
            AssetSummary(
              id: 'eth-native',
              chain: 'ethereum',
              symbol: 'ETH',
              name: 'Ethereum',
              decimals: 18,
              kind: 'native',
            ),
            AssetSummary(
              id: 'usdt-token',
              chain: 'ethereum',
              symbol: 'USDT',
              name: 'Tether USD',
              decimals: 6,
              kind: 'erc20',
              contractAddress: '0x0000000000000000000000000000000000000000',
            ),
          ],
        ),
      ),
    );

    await tester.tap(find.textContaining('ETH'));
    await tester.pumpAndSettle();
    await tester.tap(find.textContaining('USDT').last);
    await tester.pumpAndSettle();
    await tester.enterText(
      find.byType(TextField).first,
      '0x0000000000000000000000000000000000000000',
    );
    await tester.enterText(find.byType(TextField).at(1), '2.5');
    await tester.tap(find.text('Preview'));
    await tester.pumpAndSettle();

    expect(api.lastDraft?.assetId, 'usdt-token');
    expect(api.lastDraft?.chain, 'ethereum');
    expect(find.textContaining('2.5 USDT'), findsOneWidget);
  });
}

Widget _transferTestApp({required WalletApi api, required Widget home}) {
  return ProviderScope(
    overrides: [walletApiProvider.overrideWithValue(api)],
    child: MaterialApp(home: home),
  );
}

class _FakeWalletApi extends DemoWalletApi {
  _FakeWalletApi({this.parsedPayment, this.preview, this.error});

  final ParsedPayment? parsedPayment;
  final TransferPreview? preview;
  final Object? error;
  String? lastPayload;
  TransferDraft? lastDraft;

  @override
  Future<ParsedPayment> parsePaymentUri(String payload) async {
    lastPayload = payload;
    final error = this.error;
    if (error != null) {
      throw error;
    }
    return parsedPayment!;
  }

  @override
  Future<TransferPreview> previewTransfer(TransferDraft draft) async {
    lastDraft = draft;
    final error = this.error;
    if (error != null) {
      throw error;
    }
    return preview!;
  }
}
