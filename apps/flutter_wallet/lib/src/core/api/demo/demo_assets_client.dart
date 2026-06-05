import '../../../models.dart';
import 'demo_wallet_store.dart';

class DemoAssetsClient {
  const DemoAssetsClient(this._store);

  final DemoWalletStore _store;

  Future<List<AssetSummary>> list(String walletId) async {
    return _store.assets;
  }

  Future<void> refresh(String walletId, {String? chain}) async {}

  Future<void> discover(String walletId, {String? chain}) async {}

  Future<AssetSummary> addCustomToken({
    required String chain,
    required String contractAddress,
    required String tokenName,
  }) async {
    final token = AssetSummary(
      id: 'token-${_store.assets.length + 1}',
      chain: chain,
      symbol: tokenName,
      name: tokenName,
      decimals: chain == 'tron' ? 6 : 18,
      kind: chain == 'tron' ? 'trc20' : 'erc20',
      contractAddress: contractAddress,
    );
    _store.assets.add(token);
    return token;
  }

  Future<void> removeCustomToken(String assetId) async {
    _store.assets.removeWhere(
      (asset) => asset.id == assetId && asset.contractAddress != null,
    );
  }
}
