import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_wallet/src/features/settings/settings_sections.dart';
import 'package:flutter_wallet/src/models.dart';

void main() {
  testWidgets('settings network section renders networks and add action', (
    tester,
  ) async {
    var added = false;

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: ListView(
            children: [
              SettingsNetworkSection(
                networks: Future.value(const [
                  NetworkSettings(
                    chain: 'ethereum',
                    networkName: 'Ethereum',
                    chainId: '1',
                    enabled: true,
                    defaultRpcUrl: 'https://ethereum-rpc.publicnode.com',
                    nativeSymbol: 'ETH',
                    nativeDecimals: 18,
                  ),
                ]),
                onEditNetwork: (_) {},
                onAddNetwork: () {
                  added = true;
                },
              ),
            ],
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Networks'), findsOneWidget);
    expect(find.text('Ethereum'), findsOneWidget);
    expect(
      find.textContaining('https://ethereum-rpc.publicnode.com'),
      findsOneWidget,
    );
    await tester.tap(find.text('Add network'));
    expect(added, isTrue);
  });
}
