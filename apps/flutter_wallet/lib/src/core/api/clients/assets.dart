import '../../../models.dart';
import '../../../wallet_api.dart';

class AssetsClient {
  const AssetsClient(this._api);

  final WalletApi _api;

  Future<List<AssetSummary>> list(String walletId) => _api.listAssets(walletId);
  Future<void> refresh(String walletId, {String? chain}) =>
      _api.refreshAssets(walletId, chain: chain);
  Future<void> discover(String walletId, {String? chain}) =>
      _api.discoverAssets(walletId, chain: chain);
  Future<AssetSummary> addCustomToken({
    required String chain,
    required String contractAddress,
    required String tokenName,
  }) => _api.addCustomToken(
    chain: chain,
    contractAddress: contractAddress,
    tokenName: tokenName,
  );
  Future<void> removeCustomToken(String assetId) =>
      _api.removeCustomToken(assetId);
}
