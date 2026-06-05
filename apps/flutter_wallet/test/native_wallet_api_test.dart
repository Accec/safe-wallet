import 'dart:convert';

import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_wallet/src/models.dart';
import 'package:flutter_wallet/src/native_wallet_api.dart';
import 'package:flutter_wallet/src/wallet_api.dart';

void main() {
  test('native wallet api awaits asynchronous v2 command runner', () async {
    final api = NativeWalletApi(
      dbPath: '/tmp/wallet.sqlite',
      commandRunner: (command) async {
        await Future<void>.delayed(const Duration(milliseconds: 1));
        expectV2(command, domain: 'wallets', action: 'generate_mnemonic');
        return okV2(command, <String, Object?>{
          'mnemonic':
              'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about',
        });
      },
    );

    final mnemonic = await api.generateMnemonic();

    expect(mnemonic, startsWith('abandon abandon'));
  });

  test('v2 structured errors become wallet api exceptions', () async {
    final api = NativeWalletApi(
      dbPath: '/tmp/wallet.sqlite',
      commandRunner: (command) async {
        return errorV2(command, code: 'wallet_error', message: 'Unlock failed');
      },
    );

    expect(
      () => api.unlockApp('wrong-password'),
      throwsA(isA<WalletApiException>()),
    );
  });

  test('parsed payment rejects malformed optional native fields', () {
    expect(
      () => parsedPaymentFromNativeResponse(<String, dynamic>{
        'address': 'TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7',
        'amount': 12.5,
      }),
      throwsA(isA<WalletApiException>()),
    );
  });

  test('app status uses v2 app domain and parses biometric unlock state', () async {
    final api = NativeWalletApi(
      dbPath: '/tmp/wallet.sqlite',
      commandRunner: (command) async {
        expectV2(command, domain: 'app', action: 'status');
        expectPayload(command, 'db_path', '/tmp/wallet.sqlite');
        return okV2(command, <String, Object?>{
          'initialized': true,
          'locked': true,
          'biometric_enabled': true,
        });
      },
    );

    final status = await api.appStatus();

    expect(status.biometricEnabled, isTrue);
  });

  test('asset and activity operations send v2 domain actions', () async {
    final commands = <Map<String, Object?>>[];
    final api = NativeWalletApi(
      dbPath: '/tmp/wallet.sqlite',
      commandRunner: (command) async {
        commands.add(command);
        return okV2(command, <String, Object?>{});
      },
    );

    await api.refreshAssets('wallet-1', chain: 'tron');
    await api.discoverAssets('wallet-1', chain: 'bsc');
    await api.syncActivity('wallet-1', chain: 'ethereum');

    expectV2(commands[0], domain: 'assets', action: 'refresh');
    expectPayload(commands[0], 'wallet_id', 'wallet-1');
    expectPayload(commands[0], 'chain', 'tron');
    expectV2(commands[1], domain: 'assets', action: 'discover');
    expectPayload(commands[1], 'chain', 'bsc');
    expectV2(commands[2], domain: 'activity', action: 'sync');
    expectPayload(commands[2], 'chain', 'ethereum');
  });

  test('security settings send v2 auth commands', () async {
    final commands = <Map<String, Object?>>[];
    final api = NativeWalletApi(
      dbPath: '/tmp/wallet.sqlite',
      commandRunner: (command) async {
        commands.add(command);
        return okV2(command, <String, Object?>{});
      },
    );

    await api.setDuressPassword(
      masterPassword: 'master-password',
      duressPassword: 'duress-password',
    );
    await api.updateBiometricUnlock(password: 'master-password', enabled: true);

    expectV2(commands[0], domain: 'auth', action: 'set_duress_password');
    expectPayload(commands[0], 'master_password', 'master-password');
    expectPayload(commands[0], 'duress_password', 'duress-password');
    expectV2(commands[1], domain: 'auth', action: 'update_biometric_unlock');
    expectPayload(commands[1], 'enabled', true);
  });

  test('network privacy commands round trip through v2 native wallet api', () async {
    final commands = <Map<String, Object?>>[];
    final api = NativeWalletApi(
      dbPath: '/tmp/wallet.sqlite',
      commandRunner: (command) async {
        commands.add(command);
        switch ('${command['domain']}.${command['action']}') {
          case 'network.get_privacy':
            return okV2(command, <String, Object?>{
              'proxy_enabled': true,
              'proxy_mode': 'tor',
              'proxy_url': 'socks5h://127.0.0.1:9050',
            });
          case 'network.save_privacy':
          case 'network.test_proxy':
            return okV2(command, <String, Object?>{});
        }
        throw const WalletApiException('unexpected command');
      },
    );

    final settings = await api.networkPrivacySettings();
    expect(settings.proxyEnabled, isTrue);
    expect(settings.proxyMode, 'tor');
    expect(settings.proxyUrl, 'socks5h://127.0.0.1:9050');

    await api.saveNetworkPrivacySettings(
      const NetworkPrivacySettingsDraft(
        proxyEnabled: true,
        proxyMode: 'custom',
        proxyUrl: 'http://127.0.0.1:8080',
      ),
    );
    await api.testProxyConnection(
      const NetworkPrivacySettingsDraft(proxyEnabled: true, proxyMode: 'tor'),
    );

    expectV2(commands[0], domain: 'network', action: 'get_privacy');
    expectV2(commands[1], domain: 'network', action: 'save_privacy');
    expectPayload(commands[1], 'proxy_url', 'http://127.0.0.1:8080');
    expectV2(commands[2], domain: 'network', action: 'test_proxy');
    expectPayload(commands[2], 'proxy_mode', 'tor');
  });

  test('import keystore sends v2 wallet command and parses wallet', () async {
    final api = NativeWalletApi(
      dbPath: '/tmp/wallet.sqlite',
      commandRunner: (command) async {
        expectV2(command, domain: 'wallets', action: 'import_keystore');
        expectPayload(command, 'db_path', '/tmp/wallet.sqlite');
        expectPayload(command, 'label', 'Imported');
        expectPayload(command, 'keystore_json', '{"ciphertext_b64":"abc"}');
        expectPayload(command, 'keystore_password', 'keystore-password');
        expectPayload(command, 'password', 'master-password');
        return okV2(command, <String, Object?>{
          'id': 'wallet-1',
          'label': 'Imported',
        });
      },
    );

    final wallet = await api.importKeystore(
      label: 'Imported',
      keystoreJson: '{"ciphertext_b64":"abc"}',
      keystorePassword: 'keystore-password',
      password: 'master-password',
    );

    expect(wallet.label, 'Imported');
  });

  test('multisig commands round trip through v2 native wallet api', () async {
    final commands = <Map<String, Object?>>[];
    final api = NativeWalletApi(
      dbPath: '/tmp/wallet.sqlite',
      commandRunner: (command) async {
        commands.add(command);
        switch ('${command['domain']}.${command['action']}') {
          case 'multisig.import_account':
            return okV2(command, multisigAccountJson());
          case 'multisig.list_accounts':
            return okV2(command, <Object?>[multisigAccountJson()]);
          case 'multisig.create_proposal':
            return okV2(command, proposalJson(status: 'pending_signatures'));
          case 'multisig.add_signature':
            return okV2(command, proposalJson(status: 'ready', weight: 2));
          case 'multisig.list_proposals':
            return okV2(command, <Object?>[proposalJson(status: 'ready', weight: 2)]);
        }
        throw const WalletApiException('unexpected command');
      },
    );

    final account = await api.importMultisigAccount(
      const ImportMultisigAccountDraft(
        label: 'Treasury Safe',
        chain: 'ethereum',
        kind: 'evm_safe',
        address: '0x1111111111111111111111111111111111111111',
        threshold: 2,
        owners: [
          MultisigOwnerDraft(
            address: '0x2222222222222222222222222222222222222222',
            weight: 1,
          ),
          MultisigOwnerDraft(
            address: '0x3333333333333333333333333333333333333333',
            weight: 1,
          ),
        ],
      ),
    );
    final accounts = await api.listMultisigAccounts();
    final proposal = await api.createMultisigProposal(
      const CreateMultisigProposalDraft(
        multisigAccountId: 'multisig-1',
        toAddress: '0x4444444444444444444444444444444444444444',
        assetSymbol: 'ETH',
        amount: '1.25',
      ),
    );
    final signed = await api.addMultisigSignature(
      proposalId: 'proposal-1',
      ownerAddress: '0x3333333333333333333333333333333333333333',
      signature: '0xsig2',
    );
    final proposals = await api.listMultisigProposals();

    expect(account.kind, 'evm_safe');
    expect(accounts.single.label, 'Treasury Safe');
    expect(proposal.status, 'pending_signatures');
    expect(signed.status, 'ready');
    expect(proposals.single.signatureWeight, 2);
    expectV2(commands[0], domain: 'multisig', action: 'import_account');
    expectPayload(commands[0], 'owners', [
      {'address': '0x2222222222222222222222222222222222222222', 'weight': 1},
      {'address': '0x3333333333333333333333333333333333333333', 'weight': 1},
    ]);
    expectV2(commands[2], domain: 'multisig', action: 'create_proposal');
    expectV2(commands[3], domain: 'multisig', action: 'add_signature');
    expectV2(commands[4], domain: 'multisig', action: 'list_proposals');
  });
}

void expectV2(
  Map<String, Object?> command, {
  required String domain,
  required String action,
}) {
  expect(command['protocol_version'], 2);
  expect(command['request_id'], isA<String>());
  expect(command['domain'], domain);
  expect(command['action'], action);
  expect(command['payload'], isA<Map<String, Object?>>());
}

void expectPayload(Map<String, Object?> command, String key, Object? value) {
  final payload = command['payload'] as Map<String, Object?>;
  expect(payload[key], value);
}

Map<String, dynamic> okV2(Map<String, Object?> command, Object? data) {
  return <String, dynamic>{
    'ok': true,
    'body_json': jsonEncode(<String, Object?>{
      'request_id': command['request_id'],
      'ok': true,
      'data': data,
    }),
    'error': null,
  };
}

Map<String, dynamic> errorV2(
  Map<String, Object?> command, {
  required String code,
  required String message,
}) {
  return <String, dynamic>{
    'ok': true,
    'body_json': jsonEncode(<String, Object?>{
      'request_id': command['request_id'],
      'ok': false,
      'error': <String, Object?>{
        'code': code,
        'message': message,
        'retryable': false,
      },
    }),
    'error': null,
  };
}

Map<String, Object?> multisigAccountJson() {
  return <String, Object?>{
    'id': 'multisig-1',
    'label': 'Treasury Safe',
    'chain': 'ethereum',
    'kind': 'evm_safe',
    'address': '0x1111111111111111111111111111111111111111',
    'threshold': 2,
    'permission_id': null,
  };
}

Map<String, Object?> proposalJson({required String status, int weight = 0}) {
  return <String, Object?>{
    'id': 'proposal-1',
    'multisig_account_id': 'multisig-1',
    'chain': 'ethereum',
    'to_address': '0x4444444444444444444444444444444444444444',
    'asset_symbol': 'ETH',
    'amount': '1.25',
    'payload_json': '{"kind":"evm_safe"}',
    'status': status,
    'threshold': 2,
    'signature_weight': weight,
  };
}
