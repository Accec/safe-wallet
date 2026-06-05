import '../../../models.dart';
import '../../../wallet_api.dart';
import '../native_response_parsers.dart';
import '../native_wallet_gateway.dart';

part 'native_wallets_exports.dart';
part 'native_wallets_generation.dart';
part 'native_wallets_imports.dart';
part 'native_wallets_queries.dart';

class NativeWalletsClient
    with
        NativeWalletsGeneration,
        NativeWalletsImports,
        NativeWalletsExports,
        NativeWalletsQueries {
  const NativeWalletsClient(this._gateway);

  @override
  final NativeWalletGateway _gateway;
}
