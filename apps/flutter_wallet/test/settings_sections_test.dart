import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_wallet/src/app_update.dart';
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

  testWidgets('settings update section exposes android update action', (
    tester,
  ) async {
    var checked = false;
    var updated = false;

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: ListView(
            children: [
              SettingsUpdateSection(
                updateInfo: const AppUpdateInfo(
                  currentVersion: '1.0.0',
                  latestVersion: '1.2.3',
                  updateAvailable: true,
                  releasePageUrl:
                      'https://github.com/Accec/safe-wallet/releases/tag/v1.2.3',
                  platform: UpdatePlatform.android,
                  asset: AppUpdateAsset(
                    name: 'safe-wallet-android-v1.2.3.apk',
                    downloadUrl: 'https://example.invalid/android.apk',
                    size: 1024,
                  ),
                ),
                checking: false,
                updating: false,
                progress: null,
                onCheck: () {
                  checked = true;
                },
                onRunUpdateAction: () {
                  updated = true;
                },
              ),
            ],
          ),
        ),
      ),
    );

    expect(find.text('Version 1.2.3 is available.'), findsOneWidget);

    await tester.tap(find.text('Check for updates'));
    await tester.tap(find.text('Download and install'));

    expect(checked, isTrue);
    expect(updated, isTrue);
  });

  testWidgets('settings update section exposes ios release page action', (
    tester,
  ) async {
    var opened = false;

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: ListView(
            children: [
              SettingsUpdateSection(
                updateInfo: const AppUpdateInfo(
                  currentVersion: '1.0.0',
                  latestVersion: '1.2.3',
                  updateAvailable: true,
                  releasePageUrl:
                      'https://github.com/Accec/safe-wallet/releases/tag/v1.2.3',
                  platform: UpdatePlatform.ios,
                ),
                checking: false,
                updating: false,
                progress: null,
                onCheck: () {},
                onRunUpdateAction: () {
                  opened = true;
                },
              ),
            ],
          ),
        ),
      ),
    );

    await tester.tap(find.text('Open download page'));

    expect(opened, isTrue);
  });
}
