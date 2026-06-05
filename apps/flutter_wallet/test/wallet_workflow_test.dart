import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_wallet/src/biometric_auth.dart';
import 'package:flutter_wallet/main.dart';
import 'package:flutter_wallet/src/core/state/wallet_providers.dart';
import 'package:flutter_wallet/src/features/activity/presentation/activity_screen.dart';
import 'package:flutter_wallet/src/features/assets/presentation/assets_screen.dart';
import 'package:flutter_wallet/src/features/settings/settings_screen.dart';
import 'package:flutter_wallet/src/features/wallets/presentation/wallets_screen.dart';
import 'package:flutter_wallet/src/models.dart';
import 'package:flutter_wallet/src/wallet_api.dart';

void main() {
  testWidgets(
    'setup, wallet creation, token addition, and transfer preview use wallet api',
    (tester) async {
      final api = _WorkflowApi();

      await tester.pumpWidget(WalletApp(api: api));
      await tester.pumpAndSettle();

      await tester.enterText(_textField('Master password'), 'master-password');
      await tester.tap(find.text('Create password'));
      await tester.pumpAndSettle();

      await tester.tap(find.byTooltip('Create wallet'));
      await tester.pumpAndSettle();
      await tester.enterText(_textField('Wallet label'), 'Primary');
      await tester.tap(find.text('Save wallet'));
      await tester.pumpAndSettle();

      expect(api.createdWalletLabel, 'Primary');
      expect(find.text('Primary'), findsWidgets);

      await tester.tap(find.text('Assets'));
      await tester.pumpAndSettle();
      await tester.tap(find.byTooltip('Add token'));
      await tester.pumpAndSettle();
      await tester.enterText(_textField('Token name'), 'USDT');
      await tester.enterText(
        _textField('Contract address'),
        '0x0000000000000000000000000000000000000000',
      );
      await tester.tap(find.widgetWithText(FilledButton, 'Add token'));
      await tester.pumpAndSettle();

      expect(
        api.addedTokenContract,
        '0x0000000000000000000000000000000000000000',
      );
      expect(api.addedTokenName, 'USDT');
      expect(find.text('USDT'), findsOneWidget);

      await tester.tap(find.text('Transfer'));
      await tester.pumpAndSettle();
      await tester.enterText(
        _textField('Recipient'),
        '0x0000000000000000000000000000000000000000',
      );
      await tester.enterText(_textField('Amount'), '1.25');
      await tester.tap(find.text('Preview'));
      await tester.pumpAndSettle();

      expect(api.previewedAmount, '1.25');
      expect(find.textContaining('Fee: 0.00042'), findsOneWidget);

      await tester.tap(find.text('Send'));
      await tester.pumpAndSettle();

      expect(api.sentAmount, '1.25');
      expect(find.textContaining('0xabab'), findsOneWidget);

      await tester.tap(find.text('Settings'));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Add network'));
      await tester.pumpAndSettle();
      await tester.enterText(_textField('Network name'), 'BNB Smart Chain');
      await tester.enterText(
        _textField('Default RPC URL'),
        'https://example.invalid/rpc',
      );
      await tester.enterText(_textField('Chain ID'), '56');
      await tester.enterText(_textField('Currency symbol'), 'BNB');
      await tester.enterText(
        _textField('Block explorer URL'),
        'https://example.invalid/explorer',
      );
      await tester.enterText(
        _textField('Indexer API URL'),
        'https://example.invalid/indexer',
      );
      await tester.tap(find.widgetWithText(FilledButton, 'Save network'));
      await tester.pumpAndSettle();

      expect(api.updatedChain, 'bsc');
      expect(api.updatedRpcUrl, 'https://example.invalid/rpc');
      expect(api.updatedIndexerEndpoint, 'https://example.invalid/indexer');
    },
  );

  testWidgets('wallet import submits the entered recovery phrase', (
    tester,
  ) async {
    final api = _WorkflowApi();

    await tester.pumpWidget(WalletApp(api: api));
    await tester.pumpAndSettle();

    await tester.enterText(_textField('Master password'), 'master-password');
    await tester.tap(find.text('Create password'));
    await tester.pumpAndSettle();

    await tester.tap(find.byTooltip('Import wallet'));
    await tester.pumpAndSettle();
    await tester.enterText(_textField('Wallet label'), 'Imported');
    await tester.enterText(
      _textField('Recovery phrase'),
      'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about',
    );
    await tester.tap(find.widgetWithText(FilledButton, 'Import wallet'));
    await tester.pumpAndSettle();

    expect(api.importedWalletLabel, 'Imported');
    expect(
      api.importedMnemonic,
      'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about',
    );
  });

  testWidgets('create wallet dialog shows loading while mnemonic is generated', (
    tester,
  ) async {
    final api = _WorkflowApi();
    final mnemonic = Completer<String>();
    api.mnemonicCompleter = mnemonic;

    await tester.pumpWidget(WalletApp(api: api));
    await tester.pumpAndSettle();

    await tester.enterText(_textField('Master password'), 'master-password');
    await tester.tap(find.text('Create password'));
    await tester.pumpAndSettle();

    await tester.tap(find.byTooltip('Create wallet'));
    await tester.pump();

    expect(find.text('Create wallet'), findsOneWidget);
    expect(find.text('Generating'), findsOneWidget);
    expect(find.byType(CircularProgressIndicator), findsWidgets);

    mnemonic.complete(
      'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about',
    );
    await tester.pumpAndSettle();

    final field = tester.widget<TextField>(_textField('Recovery phrase'));
    expect(field.controller?.text, contains('abandon abandon'));
    expect(find.text('Generating'), findsNothing);
  });

  testWidgets('private key import and keystore export use wallet api', (
    tester,
  ) async {
    final api = _WorkflowApi();
    const privateKey =
        '0000000000000000000000000000000000000000000000000000000000000001';

    await tester.pumpWidget(WalletApp(api: api));
    await tester.pumpAndSettle();

    await tester.enterText(_textField('Master password'), 'master-password');
    await tester.tap(find.text('Create password'));
    await tester.pumpAndSettle();

    await tester.tap(find.byTooltip('Import private key'));
    await tester.pumpAndSettle();
    await tester.enterText(_textField('Wallet label'), 'Key Wallet');
    await tester.enterText(_textField('Private key'), privateKey);
    await tester.tap(find.widgetWithText(FilledButton, 'Import private key'));
    await tester.pumpAndSettle();

    expect(api.importedPrivateKeyLabel, 'Key Wallet');
    expect(api.importedPrivateKey, privateKey);
    expect(find.text('Key Wallet'), findsWidgets);

    await tester.tap(find.byTooltip('Export keystore'));
    await tester.pumpAndSettle();
    await tester.enterText(_textField('Master password'), 'master-password');
    await tester.tap(find.widgetWithText(FilledButton, 'Export'));
    await tester.pumpAndSettle();

    expect(api.exportedWalletId, 'wallet-1');
    expect(api.exportPassword, 'master-password');
    expect(find.textContaining('ciphertext_b64'), findsOneWidget);
    expect(find.textContaining(privateKey), findsNothing);
  });

  testWidgets('wallet delete prompts for password and removes wallet', (
    tester,
  ) async {
    final api = _WorkflowApi();

    await tester.pumpWidget(WalletApp(api: api));
    await tester.pumpAndSettle();

    await tester.enterText(_textField('Master password'), 'master-password');
    await tester.tap(find.text('Create password'));
    await tester.pumpAndSettle();
    await tester.tap(find.byTooltip('Create wallet'));
    await tester.pumpAndSettle();
    await tester.enterText(_textField('Wallet label'), 'Primary');
    await tester.tap(find.text('Save wallet'));
    await tester.pumpAndSettle();

    await tester.tap(find.byTooltip('Delete wallet'));
    await tester.pumpAndSettle();
    await tester.enterText(_textField('Master password'), 'master-password');
    await tester.tap(find.widgetWithText(FilledButton, 'Delete wallet'));
    await tester.pumpAndSettle();

    expect(api.deletedWalletId, 'wallet-1');
    expect(api.deletedWalletPassword, 'master-password');
    expect(find.text('No wallets'), findsOneWidget);
  });

  testWidgets('keystore import submits json and passwords', (tester) async {
    final api = _WorkflowApi();

    await tester.pumpWidget(WalletApp(api: api));
    await tester.pumpAndSettle();

    await tester.enterText(_textField('Master password'), 'master-password');
    await tester.tap(find.text('Create password'));
    await tester.pumpAndSettle();

    await tester.tap(find.byTooltip('Import keystore'));
    await tester.pumpAndSettle();
    await tester.enterText(_textField('Wallet label'), 'Imported');
    await tester.enterText(
      _textField('Keystore JSON'),
      '{"ciphertext_b64":"abc"}',
    );
    await tester.enterText(_textField('Keystore password'), 'key-password');
    await tester.enterText(_textField('Master password'), 'master-password');
    await tester.tap(find.widgetWithText(FilledButton, 'Import keystore'));
    await tester.pumpAndSettle();

    expect(api.importedKeystoreLabel, 'Imported');
    expect(api.importedKeystoreJson, '{"ciphertext_b64":"abc"}');
    expect(api.importedKeystorePassword, 'key-password');
    expect(api.importedKeystoreMasterPassword, 'master-password');
    expect(find.text('Imported'), findsWidgets);
  });

  testWidgets('biometric unlock opens the wallet without storing password', (
    tester,
  ) async {
    final api = _BiometricUnlockApi();
    final biometricAuth = _FakeBiometricAuth(result: true);

    await tester.pumpWidget(WalletApp(api: api, biometricAuth: biometricAuth));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Use biometrics'));
    await tester.pumpAndSettle();

    expect(biometricAuth.authenticateCalls, 1);
    expect(find.text('Primary'), findsWidgets);
    expect(api.unlockedPassword, isNull);
  });

  testWidgets('wallet chain dropdown filters visible account address', (
    tester,
  ) async {
    var selectedChain = 'ethereum';
    late StateSetter updateState;
    await tester.pumpWidget(
      MaterialApp(
        home: StatefulBuilder(
          builder: (context, setState) {
            updateState = setState;
            return WalletsScreen(
              wallets: const [WalletSummary(id: 'wallet-1', label: 'Primary')],
              selectedWallet: const WalletSummary(
                id: 'wallet-1',
                label: 'Primary',
              ),
              accounts: const [
                AccountSummary(
                  walletId: 'wallet-1',
                  chain: 'ethereum',
                  address: '0xethereum',
                  derivationPath: "m/44'/60'/0'/0/0",
                ),
                AccountSummary(
                  walletId: 'wallet-1',
                  chain: 'btc',
                  address: 'bc1bitcoin',
                  derivationPath: "m/84'/0'/0'/0/0",
                ),
              ],
              selectedChain: selectedChain,
              onSelectedWalletChanged: (_) {},
              onSelectedChainChanged: (chain) {
                setState(() {
                  selectedChain = chain;
                });
              },
              onRefresh: () {},
              onGenerateMnemonic: () async => '',
              onCreateWallet: ({required label, required mnemonic}) async {},
              onImportWallet: ({required label, required mnemonic}) async {},
              onImportPrivateKey:
                  ({required label, required privateKey}) async {},
              onImportKeystore:
                  ({
                    required label,
                    required keystoreJson,
                    required keystorePassword,
                    required password,
                  }) async {},
              onExportKeystore:
                  ({required walletId, required password}) async =>
                      const KeystoreExport(
                        walletId: 'wallet-1',
                        label: 'Primary',
                        secretKind: 'mnemonic',
                        ciphertextB64: 'ciphertext',
                        nonceB64: 'nonce',
                        saltB64: 'salt',
                        kdfName: 'scrypt',
                        kdfParamsJson: '{}',
                        cipherName: 'AES-256-GCM',
                        version: 1,
                      ),
              onDeleteWallet: ({required walletId, required password}) async {},
            );
          },
        ),
      ),
    );

    expect(find.text('0xethereum'), findsOneWidget);
    expect(find.text('bc1bitcoin'), findsNothing);

    updateState(() {
      selectedChain = 'btc';
    });
    await tester.pump();

    expect(find.text('bc1bitcoin'), findsOneWidget);
    expect(find.text('0xethereum'), findsNothing);
  });

  testWidgets('wallet account address can be copied', (tester) async {
    final clipboardCalls = <MethodCall>[];
    tester.binding.defaultBinaryMessenger.setMockMethodCallHandler(
      SystemChannels.platform,
      (call) async {
        if (call.method == 'Clipboard.setData') {
          clipboardCalls.add(call);
        }
        return null;
      },
    );
    addTearDown(() {
      tester.binding.defaultBinaryMessenger.setMockMethodCallHandler(
        SystemChannels.platform,
        null,
      );
    });

    await tester.pumpWidget(
      MaterialApp(
        home: WalletsScreen(
          wallets: const [WalletSummary(id: 'wallet-1', label: 'Primary')],
          selectedWallet: const WalletSummary(id: 'wallet-1', label: 'Primary'),
          accounts: const [
            AccountSummary(
              walletId: 'wallet-1',
              chain: 'ethereum',
              address: '0xethereum',
              derivationPath: "m/44'/60'/0'/0/0",
            ),
          ],
          selectedChain: 'ethereum',
          onSelectedWalletChanged: (_) {},
          onSelectedChainChanged: (_) {},
          onRefresh: () {},
          onGenerateMnemonic: () async => '',
          onCreateWallet: ({required label, required mnemonic}) async {},
          onImportWallet: ({required label, required mnemonic}) async {},
          onImportPrivateKey: ({required label, required privateKey}) async {},
          onImportKeystore:
              ({
                required label,
                required keystoreJson,
                required keystorePassword,
                required password,
              }) async {},
          onExportKeystore: ({required walletId, required password}) async =>
              const KeystoreExport(
                walletId: 'wallet-1',
                label: 'Primary',
                secretKind: 'mnemonic',
                ciphertextB64: 'ciphertext',
                nonceB64: 'nonce',
                saltB64: 'salt',
                kdfName: 'scrypt',
                kdfParamsJson: '{}',
                cipherName: 'AES-256-GCM',
                version: 1,
              ),
          onDeleteWallet: ({required walletId, required password}) async {},
        ),
      ),
    );

    await tester.tap(find.byTooltip('Copy address'));
    await tester.pump();

    expect(clipboardCalls, hasLength(1));
    expect(clipboardCalls.single.arguments, {'text': '0xethereum'});
    expect(find.text('Address copied.'), findsOneWidget);
  });

  testWidgets('assets chain dropdown filters visible assets', (tester) async {
    var selectedChain = 'ethereum';
    late StateSetter updateState;
    await tester.pumpWidget(
      MaterialApp(
        home: StatefulBuilder(
          builder: (context, setState) {
            updateState = setState;
            return AssetsScreen(
              wallet: const WalletSummary(id: 'wallet-1', label: 'Primary'),
              wallets: const [WalletSummary(id: 'wallet-1', label: 'Primary')],
              assets: const [
                AssetSummary(
                  id: 'eth-native',
                  chain: 'ethereum',
                  symbol: 'ETH',
                  name: 'ETH',
                  decimals: 18,
                  kind: 'native',
                ),
                AssetSummary(
                  id: 'btc-native',
                  chain: 'btc',
                  symbol: 'BTC',
                  name: 'BTC',
                  decimals: 8,
                  kind: 'native',
                ),
              ],
              selectedChain: selectedChain,
              onSelectedWalletChanged: (_) {},
              onSelectedChainChanged: (chain) {
                setState(() {
                  selectedChain = chain;
                });
              },
              onRefresh: () {},
              onDiscoverAssets: () {},
              onAddCustomToken:
                  ({
                    required chain,
                    required contractAddress,
                    required tokenName,
                  }) async {},
              onRemoveCustomToken: (_) async {},
            );
          },
        ),
      ),
    );

    expect(find.text('ETH'), findsOneWidget);
    expect(find.text('BTC'), findsNothing);

    updateState(() {
      selectedChain = 'btc';
    });
    await tester.pump();

    expect(find.text('BTC'), findsOneWidget);
    expect(find.text('ETH'), findsNothing);
  });

  testWidgets('sidebar wallet switch keeps the selected chain', (tester) async {
    final api = _MultiWalletApi();

    await tester.pumpWidget(WalletApp(api: api));
    await tester.pumpAndSettle();

    expect(find.byTooltip('Select chain'), findsOneWidget);
    expect(find.byTooltip('Switch to Primary wallet'), findsOneWidget);
    expect(find.byTooltip('Switch to Trading wallet'), findsOneWidget);

    await tester.tap(find.text('Assets'));
    await tester.pumpAndSettle();
    expect(find.text('ETH'), findsOneWidget);
    expect(find.text('BNB'), findsNothing);

    await tester.tap(find.byTooltip('Select chain'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('BSC').last);
    await tester.pumpAndSettle();

    expect(find.text('PBNB'), findsOneWidget);

    await tester.tap(find.byTooltip('Switch to Trading wallet'));
    await tester.pumpAndSettle();

    expect(api.lastListedAssetWalletId, 'wallet-2');
    expect(find.text('BNB'), findsOneWidget);
    expect(find.text('TETH'), findsNothing);
  });

  testWidgets('assets list shows balance amounts', (tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: AssetsScreen(
          wallet: const WalletSummary(id: 'wallet-1', label: 'Primary'),
          wallets: const [WalletSummary(id: 'wallet-1', label: 'Primary')],
          assets: const [
            AssetSummary(
              id: 'eth-native',
              chain: 'ethereum',
              symbol: 'ETH',
              name: 'ETH',
              decimals: 18,
              kind: 'native',
            ),
          ],
          selectedChain: 'ethereum',
          onSelectedWalletChanged: (_) {},
          onSelectedChainChanged: (_) {},
          onRefresh: () {},
          onDiscoverAssets: () {},
          onAddCustomToken:
              ({
                required chain,
                required contractAddress,
                required tokenName,
              }) async {},
          onRemoveCustomToken: (_) async {},
        ),
      ),
    );

    expect(find.text('0 ETH'), findsOneWidget);
    expect(find.text('native'), findsNothing);
  });

  testWidgets('assets refresh uses wallet api', (tester) async {
    final api = _WorkflowApi();

    await tester.pumpWidget(WalletApp(api: api));
    await tester.pumpAndSettle();
    await tester.enterText(_textField('Master password'), 'master-password');
    await tester.tap(find.text('Create password'));
    await tester.pumpAndSettle();
    await tester.tap(find.byTooltip('Create wallet'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Save wallet'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Assets'));
    await tester.pumpAndSettle();

    await tester.tap(find.byTooltip('Refresh assets'));
    await tester.pumpAndSettle();

    expect(api.refreshedWalletId, 'wallet-1');
    expect(api.refreshedChain, 'ethereum');
    expect(api.discoveredWalletId, isNull);
  });

  testWidgets('assets discovery uses wallet api only from discover button', (
    tester,
  ) async {
    final api = _WorkflowApi();

    await tester.pumpWidget(WalletApp(api: api));
    await tester.pumpAndSettle();
    await tester.enterText(_textField('Master password'), 'master-password');
    await tester.tap(find.text('Create password'));
    await tester.pumpAndSettle();
    await tester.tap(find.byTooltip('Create wallet'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Save wallet'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Assets'));
    await tester.pumpAndSettle();

    expect(api.discoveredWalletId, isNull);

    await tester.tap(find.byTooltip('Discover assets'));
    await tester.pumpAndSettle();

    expect(api.discoveredWalletId, 'wallet-1');
    expect(api.discoveredChain, 'ethereum');
  });

  testWidgets('contract token can be removed from assets', (tester) async {
    String? removedAssetId;
    var assets = const [
      AssetSummary(
        id: 'eth-native',
        chain: 'ethereum',
        symbol: 'ETH',
        name: 'ETH',
        decimals: 18,
        kind: 'native',
      ),
      AssetSummary(
        id: 'usdt-token',
        chain: 'ethereum',
        symbol: 'USDT',
        name: 'USDT',
        decimals: 18,
        kind: 'erc20',
        contractAddress: '0x0000000000000000000000000000000000000000',
      ),
    ];

    await tester.pumpWidget(
      MaterialApp(
        home: StatefulBuilder(
          builder: (context, setState) {
            return AssetsScreen(
              wallet: const WalletSummary(id: 'wallet-1', label: 'Primary'),
              wallets: const [WalletSummary(id: 'wallet-1', label: 'Primary')],
              assets: assets,
              selectedChain: 'ethereum',
              onSelectedWalletChanged: (_) {},
              onSelectedChainChanged: (_) {},
              onRefresh: () {},
              onDiscoverAssets: () {},
              onAddCustomToken:
                  ({
                    required chain,
                    required contractAddress,
                    required tokenName,
                  }) async {},
              onRemoveCustomToken: (assetId) async {
                removedAssetId = assetId;
                setState(() {
                  assets = assets
                      .where((asset) => asset.id != assetId)
                      .toList();
                });
              },
            );
          },
        ),
      ),
    );

    expect(find.text('USDT'), findsOneWidget);
    expect(find.byTooltip('Remove token'), findsOneWidget);

    await tester.tap(find.byTooltip('Remove token'));
    await tester.pumpAndSettle();
    await tester.tap(find.widgetWithText(FilledButton, 'Remove'));
    await tester.pumpAndSettle();

    expect(removedAssetId, 'usdt-token');
    expect(find.text('USDT'), findsNothing);
    expect(find.text('ETH'), findsOneWidget);
  });

  testWidgets(
    'add token dialog uses token name and keeps invalid contract errors visible',
    (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: AssetsScreen(
            wallet: const WalletSummary(id: 'wallet-1', label: 'Primary'),
            wallets: const [WalletSummary(id: 'wallet-1', label: 'Primary')],
            assets: const [
              AssetSummary(
                id: 'eth-native',
                chain: 'ethereum',
                symbol: 'ETH',
                name: 'ETH',
                decimals: 18,
                kind: 'native',
              ),
            ],
            selectedChain: 'ethereum',
            onSelectedWalletChanged: (_) {},
            onSelectedChainChanged: (_) {},
            onRefresh: () {},
            onDiscoverAssets: () {},
            onAddCustomToken:
                ({
                  required chain,
                  required contractAddress,
                  required tokenName,
                }) async {
                  throw const WalletApiException('Invalid token contract');
                },
            onRemoveCustomToken: (_) async {},
          ),
        ),
      );

      await tester.tap(find.byTooltip('Add token'));
      await tester.pumpAndSettle();

      expect(_textField('Token name'), findsOneWidget);
      expect(_textField('Chain'), findsNothing);

      await tester.enterText(_textField('Token name'), 'USDT');
      await tester.enterText(_textField('Contract address'), 'not-a-contract');
      await tester.tap(find.widgetWithText(FilledButton, 'Add token'));
      await tester.pumpAndSettle();

      expect(find.text('Invalid token contract'), findsOneWidget);
      expect(find.text('Add token'), findsWidgets);
      expect(_textField('Contract address'), findsOneWidget);
    },
  );

  testWidgets('settings lists existing networks and edits one', (tester) async {
    final api = _WorkflowApi();

    await tester.pumpWidget(_settingsHarness(api));
    await tester.pumpAndSettle();

    expect(find.text('Ethereum'), findsOneWidget);
    expect(
      find.textContaining('https://ethereum-rpc.publicnode.com'),
      findsOneWidget,
    );

    await tester.tap(find.text('Ethereum'));
    await tester.pumpAndSettle();
    await tester.enterText(
      _textField('Default RPC URL'),
      'https://example.invalid/edited-rpc',
    );
    await tester.tap(find.widgetWithText(FilledButton, 'Save network'));
    await tester.pumpAndSettle();

    expect(api.updatedChain, 'ethereum');
    expect(api.updatedRpcUrl, 'https://example.invalid/edited-rpc');
  });

  testWidgets('activity sync button invokes wallet api callback', (
    tester,
  ) async {
    var synced = false;

    await tester.pumpWidget(
      MaterialApp(
        home: ActivityScreen(
          activity: const [],
          selectedWallet: const WalletSummary(id: 'wallet-1', label: 'Primary'),
          onSync: () async {
            synced = true;
          },
        ),
      ),
    );
    await tester.pumpAndSettle();

    await tester.tap(find.byTooltip('Sync history'));
    await tester.pumpAndSettle();

    expect(synced, isTrue);
  });

  testWidgets('activity rows show clear transaction details', (tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: ActivityScreen(
          activity: const [
            ActivitySummary(
              chain: 'ethereum',
              txHash:
                  '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef',
              kind: 'native_transfer',
              status: 'pending',
              summary: 'Sent 1.25 ETH',
            ),
          ],
          selectedWallet: WalletSummary(id: 'wallet-1', label: 'Primary'),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Sent'), findsOneWidget);
    expect(find.text('-1.25 ETH'), findsOneWidget);
    expect(find.text('Pending'), findsOneWidget);
    expect(find.text('Ethereum'), findsOneWidget);
    expect(find.text('0x123456...90abcdef'), findsOneWidget);
    expect(find.text('native_transfer'), findsNothing);
  });

  testWidgets('synced transfer activity rows do not overflow on narrow screens', (
    tester,
  ) async {
    tester.view.devicePixelRatio = 1.0;
    tester.view.physicalSize = const Size(390, 844);
    addTearDown(tester.view.resetDevicePixelRatio);
    addTearDown(tester.view.resetPhysicalSize);
    final flutterErrors = <FlutterErrorDetails>[];
    final previousOnError = FlutterError.onError;
    FlutterError.onError = flutterErrors.add;
    addTearDown(() {
      FlutterError.onError = previousOnError;
    });

    await tester.pumpWidget(
      MaterialApp(
        home: ActivityScreen(
          activity: const [
            ActivitySummary(
              chain: 'tron',
              txHash:
                  'b99d92f5abcdef1234567890abcdef1234567890abcdef1234567890b0e520ed',
              kind: 'token_transfer',
              status: 'confirmed',
              summary:
                  'Transfer 1951.438573 USDT to TYnQwJiYo1yzGf123456789abcdefJ2s',
            ),
          ],
          selectedWallet: WalletSummary(id: 'wallet-1', label: 'Primary'),
        ),
      ),
    );
    await tester.pumpAndSettle();
    FlutterError.onError = previousOnError;

    expect(find.text('Transfer'), findsOneWidget);
    expect(find.text('1951.438573 USDT'), findsOneWidget);
    expect(find.textContaining('TYnQwJiYo1yzGf'), findsNothing);
    expect(
      flutterErrors.where(
        (details) =>
            details.exceptionAsString().contains('RenderFlex overflowed'),
      ),
      isEmpty,
    );
  });

  testWidgets('multisig account import and proposal creation use wallet api', (
    tester,
  ) async {
    final api = _WorkflowApi();

    await tester.pumpWidget(WalletApp(api: api));
    await tester.pumpAndSettle();
    await tester.enterText(_textField('Master password'), 'master-password');
    await tester.tap(find.text('Create password'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Multisig'));
    await tester.pumpAndSettle();
    await tester.tap(find.byTooltip('Import multisig'));
    await tester.pumpAndSettle();
    await tester.enterText(_textField('Label'), 'Treasury Safe');
    await tester.enterText(
      _textField('Multisig address'),
      '0x1111111111111111111111111111111111111111',
    );
    await tester.enterText(_textField('Threshold'), '2');
    await tester.enterText(
      _textField('Owners'),
      '0x2222222222222222222222222222222222222222,1\n'
      '0x3333333333333333333333333333333333333333,1',
    );
    await tester.tap(find.widgetWithText(FilledButton, 'Import'));
    await tester.pumpAndSettle();

    expect(api.importedMultisigLabel, 'Treasury Safe');
    expect(api.importedMultisigKind, 'evm_safe');
    expect(find.text('Treasury Safe'), findsOneWidget);

    await tester.tap(find.byTooltip('Create multisig proposal'));
    await tester.pumpAndSettle();
    await tester.enterText(
      _textField('Recipient'),
      '0x4444444444444444444444444444444444444444',
    );
    await tester.enterText(_textField('Asset symbol'), 'ETH');
    await tester.enterText(_textField('Amount'), '1.25');
    await tester.tap(find.widgetWithText(FilledButton, 'Create proposal'));
    await tester.pumpAndSettle();

    expect(api.createdMultisigProposalAmount, '1.25');
    expect(find.textContaining('pending_signatures'), findsOneWidget);

    await tester.tap(find.byTooltip('Add multisig signature'));
    await tester.pumpAndSettle();
    await tester.enterText(
      _textField('Owner address'),
      '0x2222222222222222222222222222222222222222',
    );
    await tester.enterText(_textField('Signature'), '0xsig1');
    await tester.tap(find.widgetWithText(FilledButton, 'Add signature'));
    await tester.pumpAndSettle();

    expect(
      api.lastMultisigSigner,
      '0x2222222222222222222222222222222222222222',
    );
  });

  testWidgets('settings keeps alternate unlock behind advanced security', (
    tester,
  ) async {
    final api = _WorkflowApi();
    api.initialized = true;

    await tester.pumpWidget(_settingsHarness(api));
    await tester.pumpAndSettle();

    await tester.scrollUntilVisible(
      find.text('Biometrics'),
      300,
      scrollable: find.byType(Scrollable).last,
    );
    await tester.pumpAndSettle();
    await tester.tap(find.byType(Switch).first);
    await tester.pumpAndSettle();
    await tester.enterText(_textField('Master password'), 'master-password');
    await tester.tap(find.widgetWithText(FilledButton, 'Enable'));
    await tester.pumpAndSettle();

    expect(api.biometricPassword, 'master-password');
    expect(api.biometricEnabled, isTrue);

    expect(find.text('Duress password'), findsNothing);
    expect(find.textContaining('duress', findRichText: true), findsNothing);
    expect(find.textContaining('Duress', findRichText: true), findsNothing);

    await tester.ensureVisible(find.text('Advanced security'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Advanced security'));
    await tester.pumpAndSettle();
    await tester.enterText(_textField('Master password'), 'master-password');
    await tester.tap(find.widgetWithText(FilledButton, 'Continue'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Alternate unlock'));
    await tester.pumpAndSettle();
    expect(
      find.text('Using an alternate password opens an isolated wallet space.'),
      findsOneWidget,
    );
    expect(find.textContaining('duress', findRichText: true), findsNothing);
    expect(find.textContaining('Duress', findRichText: true), findsNothing);

    await tester.enterText(
      _textField('Alternate unlock password'),
      'duress-password',
    );
    await tester.enterText(
      _textField('Confirm alternate unlock password'),
      'duress-password',
    );
    await tester.tap(find.widgetWithText(FilledButton, 'Save'));
    await tester.pumpAndSettle();

    expect(api.duressMasterPassword, 'master-password');
    expect(api.duressPassword, 'duress-password');
  });

  testWidgets('settings saves and tests tor proxy settings', (tester) async {
    final api = _WorkflowApi();
    api.initialized = true;

    await tester.pumpWidget(_settingsHarness(api));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Network privacy'));
    await tester.pumpAndSettle();
    await tester.tap(find.byType(Switch).last);
    await tester.pumpAndSettle();
    await tester.tap(find.text('Tor'));
    await tester.pumpAndSettle();

    await tester.tap(find.widgetWithText(OutlinedButton, 'Test connection'));
    await tester.pumpAndSettle();
    expect(find.text('Proxy connection succeeded.'), findsOneWidget);

    await tester.tap(find.widgetWithText(FilledButton, 'Save'));
    await tester.pumpAndSettle();

    expect(api.savedNetworkPrivacy?.proxyEnabled, isTrue);
    expect(api.savedNetworkPrivacy?.proxyMode, 'tor');
    expect(api.savedNetworkPrivacy?.proxyUrl, 'socks5://127.0.0.1:9050');
    expect(api.proxyTested, isTrue);
  });
}

Finder _textField(String label) {
  return find.byWidgetPredicate((widget) {
    return widget is TextField && widget.decoration?.labelText == label;
  });
}

Widget _settingsHarness(_WorkflowApi api) {
  return ProviderScope(
    overrides: [walletApiProvider.overrideWithValue(api)],
    child: const MaterialApp(home: SettingsScreen()),
  );
}

class _FakeBiometricAuth implements BiometricAuth {
  _FakeBiometricAuth({required this.result});

  final bool result;
  int authenticateCalls = 0;

  @override
  Future<bool> canAuthenticate() async => true;

  @override
  Future<bool> authenticate() async {
    authenticateCalls += 1;
    return result;
  }
}

class _BiometricUnlockApi extends _WorkflowApi {
  _BiometricUnlockApi() {
    initialized = true;
    locked = true;
    _wallet = const WalletSummary(id: 'wallet-1', label: 'Primary');
  }

  String? unlockedPassword;

  @override
  Future<AppStatus> appStatus() async {
    return const AppStatus(
      initialized: true,
      locked: true,
      biometricEnabled: true,
    );
  }

  @override
  Future<void> unlockApp(String password) async {
    unlockedPassword = password;
    await super.unlockApp(password);
  }
}

class _WorkflowApi implements WalletApi {
  bool initialized = false;
  bool locked = true;
  String? createdWalletLabel;
  String? importedWalletLabel;
  String? importedMnemonic;
  String? importedPrivateKeyLabel;
  String? importedPrivateKey;
  String? importedKeystoreLabel;
  String? importedKeystoreJson;
  String? importedKeystorePassword;
  String? importedKeystoreMasterPassword;
  String? exportedWalletId;
  String? exportPassword;
  String? deletedWalletId;
  String? deletedWalletPassword;
  String? addedTokenName;
  String? addedTokenContract;
  String? previewedAmount;
  String? sentAmount;
  String? updatedChain;
  String? updatedRpcUrl;
  String? updatedIndexerEndpoint;
  String? refreshedWalletId;
  String? refreshedChain;
  String? discoveredWalletId;
  String? discoveredChain;
  String? syncedActivityWalletId;
  String? syncedActivityChain;
  String? importedMultisigLabel;
  String? importedMultisigKind;
  String? createdMultisigProposalAmount;
  String? lastMultisigSigner;
  String? duressMasterPassword;
  String? duressPassword;
  String? biometricPassword;
  bool? biometricEnabled;
  NetworkPrivacySettings networkPrivacy = const NetworkPrivacySettings(
    proxyEnabled: false,
    proxyMode: 'custom',
  );
  NetworkPrivacySettingsDraft? savedNetworkPrivacy;
  bool proxyTested = false;
  Completer<String>? mnemonicCompleter;
  WalletSummary? _wallet;
  final List<MultisigAccountSummary> _multisigAccounts = [];
  final List<MultisigProposalSummary> _multisigProposals = [];
  final List<AssetSummary> _assets = [
    const AssetSummary(
      id: 'eth-native',
      chain: 'ethereum',
      symbol: 'ETH',
      name: 'ETH',
      decimals: 18,
      kind: 'native',
    ),
  ];

  @override
  Future<AppStatus> appStatus() async {
    return AppStatus(initialized: initialized, locked: locked);
  }

  @override
  Future<AssetSummary> addCustomToken({
    required String chain,
    required String contractAddress,
    required String tokenName,
  }) async {
    addedTokenName = tokenName;
    addedTokenContract = contractAddress;
    final token = AssetSummary(
      id: 'token-1',
      chain: 'ethereum',
      symbol: tokenName,
      name: tokenName,
      decimals: 18,
      kind: 'erc20',
      contractAddress: '0x0000000000000000000000000000000000000000',
    );
    _assets.add(token);
    return token;
  }

  @override
  Future<void> removeCustomToken(String assetId) async {
    _assets.removeWhere(
      (asset) => asset.id == assetId && asset.contractAddress != null,
    );
  }

  @override
  Future<WalletSummary> createWallet({
    required String label,
    required String mnemonic,
    required String password,
  }) async {
    createdWalletLabel = label;
    final wallet = WalletSummary(id: 'wallet-1', label: label);
    _wallet = wallet;
    return wallet;
  }

  @override
  Future<String> generateMnemonic() async {
    final completer = mnemonicCompleter;
    if (completer != null) {
      return completer.future;
    }
    return 'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about';
  }

  @override
  Future<WalletSummary> importWallet({
    required String label,
    required String mnemonic,
    required String password,
  }) async {
    importedWalletLabel = label;
    importedMnemonic = mnemonic;
    return createWallet(label: label, mnemonic: mnemonic, password: password);
  }

  @override
  Future<WalletSummary> importPrivateKey({
    required String label,
    required String privateKey,
    required String password,
  }) async {
    importedPrivateKeyLabel = label;
    importedPrivateKey = privateKey;
    final wallet = WalletSummary(id: 'wallet-1', label: label);
    _wallet = wallet;
    return wallet;
  }

  @override
  Future<WalletSummary> importKeystore({
    required String label,
    required String keystoreJson,
    required String keystorePassword,
    required String password,
  }) async {
    importedKeystoreLabel = label;
    importedKeystoreJson = keystoreJson;
    importedKeystorePassword = keystorePassword;
    importedKeystoreMasterPassword = password;
    final wallet = WalletSummary(id: 'wallet-1', label: label);
    _wallet = wallet;
    return wallet;
  }

  @override
  Future<KeystoreExport> exportKeystore({
    required String walletId,
    required String password,
  }) async {
    exportedWalletId = walletId;
    exportPassword = password;
    return KeystoreExport(
      walletId: walletId,
      label: _wallet?.label ?? 'Wallet',
      secretKind: 'private_key',
      ciphertextB64: 'encrypted-ciphertext',
      nonceB64: 'encrypted-nonce',
      saltB64: 'encrypted-salt',
      kdfName: 'scrypt',
      kdfParamsJson: '{}',
      cipherName: 'AES-256-GCM',
      version: 1,
    );
  }

  @override
  Future<void> deleteWallet({
    required String walletId,
    required String password,
  }) async {
    deletedWalletId = walletId;
    deletedWalletPassword = password;
    if (_wallet?.id == walletId) {
      _wallet = null;
    }
  }

  @override
  Future<List<AccountSummary>> listAccounts(String walletId) async {
    return const [
      AccountSummary(
        walletId: 'wallet-1',
        chain: 'ethereum',
        address: '0x9858effd232b4033e47d90003d41ec34ecaeda94',
        derivationPath: "m/44'/60'/0'/0/0",
      ),
    ];
  }

  @override
  Future<List<ActivitySummary>> listActivity(String walletId) async => const [];

  @override
  Future<void> syncActivity(String walletId, {String? chain}) async {
    syncedActivityWalletId = walletId;
    syncedActivityChain = chain;
  }

  @override
  Future<MultisigAccountSummary> importMultisigAccount(
    ImportMultisigAccountDraft draft,
  ) async {
    importedMultisigLabel = draft.label;
    importedMultisigKind = draft.kind;
    final account = MultisigAccountSummary(
      id: 'multisig-1',
      label: draft.label,
      chain: draft.chain,
      kind: draft.kind,
      address: draft.address,
      threshold: draft.threshold,
      permissionId: draft.permissionId,
    );
    _multisigAccounts.add(account);
    return account;
  }

  @override
  Future<List<MultisigAccountSummary>> listMultisigAccounts() async =>
      _multisigAccounts;

  @override
  Future<MultisigProposalSummary> createMultisigProposal(
    CreateMultisigProposalDraft draft,
  ) async {
    createdMultisigProposalAmount = draft.amount;
    final account = _multisigAccounts.first;
    final proposal = MultisigProposalSummary(
      id: 'proposal-1',
      multisigAccountId: account.id,
      chain: account.chain,
      toAddress: draft.toAddress,
      assetSymbol: draft.assetSymbol,
      amount: draft.amount,
      payloadJson: '{"kind":"${account.kind}"}',
      status: 'pending_signatures',
      threshold: account.threshold,
      signatureWeight: 0,
    );
    _multisigProposals.add(proposal);
    return proposal;
  }

  @override
  Future<List<MultisigProposalSummary>> listMultisigProposals({
    String? multisigAccountId,
  }) async => _multisigProposals;

  @override
  Future<MultisigProposalSummary> addMultisigSignature({
    required String proposalId,
    required String ownerAddress,
    required String signature,
  }) async {
    lastMultisigSigner = ownerAddress;
    final proposal = _multisigProposals.first;
    final signed = MultisigProposalSummary(
      id: proposal.id,
      multisigAccountId: proposal.multisigAccountId,
      chain: proposal.chain,
      toAddress: proposal.toAddress,
      assetSymbol: proposal.assetSymbol,
      amount: proposal.amount,
      payloadJson: proposal.payloadJson,
      status: 'pending_signatures',
      threshold: proposal.threshold,
      signatureWeight: proposal.signatureWeight + 1,
    );
    _multisigProposals[0] = signed;
    return signed;
  }

  @override
  Future<NetworkPrivacySettings> networkPrivacySettings() async {
    return networkPrivacy;
  }

  @override
  Future<void> saveNetworkPrivacySettings(
    NetworkPrivacySettingsDraft draft,
  ) async {
    savedNetworkPrivacy = draft;
    networkPrivacy = NetworkPrivacySettings(
      proxyEnabled: draft.proxyEnabled,
      proxyMode: draft.proxyMode,
      proxyUrl: draft.proxyUrl,
    );
  }

  @override
  Future<void> testProxyConnection(NetworkPrivacySettingsDraft draft) async {
    proxyTested = true;
  }

  @override
  Future<List<NetworkSettings>> listNetworkSettings() async {
    return const [
      NetworkSettings(
        chain: 'ethereum',
        networkName: 'Ethereum',
        chainId: '1',
        enabled: true,
        defaultRpcUrl: 'https://ethereum-rpc.publicnode.com',
        nativeSymbol: 'ETH',
        nativeDecimals: 18,
        explorerUrl: 'https://etherscan.io',
      ),
    ];
  }

  @override
  Future<List<AssetSummary>> listAssets(String walletId) async => _assets;

  @override
  Future<void> refreshAssets(String walletId, {String? chain}) async {
    refreshedWalletId = walletId;
    refreshedChain = chain;
  }

  @override
  Future<void> discoverAssets(String walletId, {String? chain}) async {
    discoveredWalletId = walletId;
    discoveredChain = chain;
  }

  @override
  Future<List<WalletSummary>> listWallets() async {
    final wallet = _wallet;
    return wallet == null ? const [] : [wallet];
  }

  @override
  Future<ParsedPayment> parsePaymentUri(String payload) async {
    return ParsedPayment(address: payload);
  }

  @override
  Future<TransferPreview> previewTransfer(TransferDraft draft) async {
    previewedAmount = draft.amount;
    return TransferPreview(
      chain: draft.chain,
      fromAddress: '0x9858effd232b4033e47d90003d41ec34ecaeda94',
      toAddress: draft.toAddress,
      assetSymbol: 'ETH',
      amount: draft.amount,
      feeEstimate: '0.00042',
      rpcUrl: 'https://ethereum-rpc.publicnode.com',
    );
  }

  @override
  Future<TransferResult> sendTransfer(
    TransferDraft draft,
    String password,
  ) async {
    sentAmount = draft.amount;
    return const TransferResult(
      chain: 'ethereum',
      txHash: '0xabab',
      status: 'broadcasted',
    );
  }

  @override
  Future<void> setMasterPassword(String password) async {
    initialized = true;
    locked = false;
  }

  @override
  Future<void> unlockApp(String password) async {
    locked = false;
  }

  @override
  Future<void> setDuressPassword({
    required String masterPassword,
    required String duressPassword,
  }) async {
    duressMasterPassword = masterPassword;
    this.duressPassword = duressPassword;
  }

  @override
  Future<void> updateBiometricUnlock({
    required String password,
    required bool enabled,
  }) async {
    biometricPassword = password;
    biometricEnabled = enabled;
  }

  @override
  Future<void> updateChainRpc({
    required String chain,
    required String rpcUrl,
  }) async {
    updatedChain = chain;
    updatedRpcUrl = rpcUrl;
  }

  @override
  Future<void> saveNetworkSettings(NetworkSettingsDraft draft) async {
    updatedChain = switch (draft.chainId) {
      '1' => 'ethereum',
      '10' => 'optimism',
      '56' => 'bsc',
      '137' => 'polygon',
      '42161' => 'arbitrum',
      '728126428' => 'tron',
      _ => draft.chainId,
    };
    updatedRpcUrl = draft.rpcUrl;
    updatedIndexerEndpoint = draft.indexerEndpoint;
  }

  @override
  Future<void> updateIndexerSettings({
    required String chain,
    required String endpoint,
    String? apiKey,
  }) async {
    updatedIndexerEndpoint = endpoint;
  }
}

class _MultiWalletApi extends DemoWalletApi {
  String? lastListedAssetWalletId;

  @override
  Future<AppStatus> appStatus() async {
    return const AppStatus(initialized: true, locked: false);
  }

  @override
  Future<List<WalletSummary>> listWallets() async {
    return const [
      WalletSummary(id: 'wallet-1', label: 'Primary'),
      WalletSummary(id: 'wallet-2', label: 'Trading'),
    ];
  }

  @override
  Future<List<AccountSummary>> listAccounts(String walletId) async {
    return [
      AccountSummary(
        walletId: walletId,
        chain: 'ethereum',
        address: walletId == 'wallet-2' ? '0xtradingeth' : '0xethereum',
        derivationPath: "m/44'/60'/0'/0/0",
      ),
      AccountSummary(
        walletId: walletId,
        chain: 'bsc',
        address: walletId == 'wallet-2' ? '0xtradingbsc' : '0xprimarybsc',
        derivationPath: "m/44'/60'/0'/0/0",
      ),
    ];
  }

  @override
  Future<List<AssetSummary>> listAssets(String walletId) async {
    lastListedAssetWalletId = walletId;
    if (walletId == 'wallet-2') {
      return const [
        AssetSummary(
          id: 'trading-eth-native',
          chain: 'ethereum',
          symbol: 'TETH',
          name: 'Trading ETH',
          decimals: 18,
          kind: 'native',
        ),
        AssetSummary(
          id: 'bnb-native',
          chain: 'bsc',
          symbol: 'BNB',
          name: 'BNB',
          decimals: 18,
          kind: 'native',
        ),
      ];
    }
    return const [
      AssetSummary(
        id: 'eth-native',
        chain: 'ethereum',
        symbol: 'ETH',
        name: 'ETH',
        decimals: 18,
        kind: 'native',
      ),
      AssetSummary(
        id: 'primary-bnb-native',
        chain: 'bsc',
        symbol: 'PBNB',
        name: 'Primary BNB',
        decimals: 18,
        kind: 'native',
      ),
    ];
  }

  @override
  Future<List<ActivitySummary>> listActivity(String walletId) async => const [];
}
