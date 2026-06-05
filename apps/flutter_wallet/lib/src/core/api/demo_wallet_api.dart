import 'demo/api/demo_activity_api.dart';
import 'demo/api/demo_app_api.dart';
import 'demo/api/demo_assets_api.dart';
import 'demo/api/demo_auth_api.dart';
import 'demo/api/demo_multisig_api.dart';
import 'demo/api/demo_network_api.dart';
import 'demo/api/demo_transfers_api.dart';
import 'demo/api/demo_wallets_api.dart';
import 'demo/demo_activity_client.dart';
import 'demo/demo_assets_client.dart';
import 'demo/demo_auth_client.dart';
import 'demo/demo_multisig_client.dart';
import 'demo/demo_network_client.dart';
import 'demo/demo_transfers_client.dart';
import 'demo/demo_wallet_store.dart';
import 'demo/demo_wallets_client.dart';
import 'wallet_api_contract.dart';

class DemoWalletApi
    with
        DemoAppApi,
        DemoAuthApi,
        DemoWalletsApi,
        DemoAssetsApi,
        DemoTransfersApi,
        DemoActivityApi,
        DemoMultisigApi,
        DemoNetworkApi
    implements WalletApi {
  DemoWalletApi([DemoWalletStore? store])
    : _store = store ?? DemoWalletStore() {
    _auth = DemoAuthClient(_store);
    _wallets = DemoWalletsClient(_store);
    _assets = DemoAssetsClient(_store);
    _transfers = DemoTransfersClient(_store);
    _activity = DemoActivityClient(_store);
    _multisig = DemoMultisigClient(_store);
    _network = DemoNetworkClient(_store);
  }

  final DemoWalletStore _store;
  late final DemoAuthClient _auth;
  late final DemoWalletsClient _wallets;
  late final DemoAssetsClient _assets;
  late final DemoTransfersClient _transfers;
  late final DemoActivityClient _activity;
  late final DemoMultisigClient _multisig;
  late final DemoNetworkClient _network;

  @override
  DemoAuthClient get authClient => _auth;

  @override
  DemoWalletsClient get walletsClient => _wallets;

  @override
  DemoAssetsClient get assetsClient => _assets;

  @override
  DemoTransfersClient get transfersClient => _transfers;

  @override
  DemoActivityClient get activityClient => _activity;

  @override
  DemoMultisigClient get multisigClient => _multisig;

  @override
  DemoNetworkClient get networkClient => _network;
}
