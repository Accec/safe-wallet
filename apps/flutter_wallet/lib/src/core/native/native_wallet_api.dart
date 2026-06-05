import '../../wallet_api.dart';
import 'api/native_activity_api.dart';
import 'api/native_app_api.dart';
import 'api/native_assets_api.dart';
import 'api/native_auth_api.dart';
import 'api/native_multisig_api.dart';
import 'api/native_network_api.dart';
import 'api/native_transfers_api.dart';
import 'api/native_wallets_api.dart';
import 'clients/native_activity_client.dart';
import 'clients/native_app_client.dart';
import 'clients/native_assets_client.dart';
import 'clients/native_auth_client.dart';
import 'clients/native_multisig_client.dart';
import 'clients/native_network_client.dart';
import 'clients/native_transfers_client.dart';
import 'clients/native_wallets_client.dart';
import 'native_library.dart';
import 'native_wallet_gateway.dart';

export 'native_library.dart' show NativeCommandRunner;
export 'native_response_parsers.dart' show parsedPaymentFromNativeResponse;

class NativeWalletApi
    with
        NativeAppApi,
        NativeAuthApi,
        NativeWalletsApi,
        NativeAssetsApi,
        NativeTransfersApi,
        NativeActivityApi,
        NativeMultisigApi,
        NativeNetworkApi
    implements WalletApi {
  NativeWalletApi({
    NativeWalletLibrary? library,
    required this.dbPath,
    NativeCommandRunner? commandRunner,
  }) : _gateway = NativeWalletGateway(
         dbPath: dbPath,
         commandRunner:
             commandRunner ??
             (library == null
                 ? runNativeCommandInBackground
                 : LocalNativeCommandRunner(library).call),
       ) {
    _app = NativeAppClient(_gateway);
    _auth = NativeAuthClient(_gateway);
    _wallets = NativeWalletsClient(_gateway);
    _assets = NativeAssetsClient(_gateway);
    _transfers = NativeTransfersClient(_gateway);
    _activity = NativeActivityClient(_gateway);
    _multisig = NativeMultisigClient(_gateway);
    _network = NativeNetworkClient(_gateway);
  }

  final String dbPath;
  final NativeWalletGateway _gateway;
  late final NativeAppClient _app;
  late final NativeAuthClient _auth;
  late final NativeWalletsClient _wallets;
  late final NativeAssetsClient _assets;
  late final NativeTransfersClient _transfers;
  late final NativeActivityClient _activity;
  late final NativeMultisigClient _multisig;
  late final NativeNetworkClient _network;

  @override
  NativeAppClient get appClient => _app;

  @override
  NativeAuthClient get authClient => _auth;

  @override
  NativeWalletsClient get walletsClient => _wallets;

  @override
  NativeAssetsClient get assetsClient => _assets;

  @override
  NativeTransfersClient get transfersClient => _transfers;

  @override
  NativeActivityClient get activityClient => _activity;

  @override
  NativeMultisigClient get multisigClient => _multisig;

  @override
  NativeNetworkClient get networkClient => _network;
}
