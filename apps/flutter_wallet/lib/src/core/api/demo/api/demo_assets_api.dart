import '../../../../models.dart';
import '../demo_assets_client.dart';

mixin DemoAssetsApi {
  DemoAssetsClient get assetsClient;

  Future<List<AssetSummary>> listAssets(String walletId) {
    return assetsClient.list(walletId);
  }

  Future<void> refreshAssets(String walletId, {String? chain}) {
    return assetsClient.refresh(walletId, chain: chain);
  }

  Future<void> discoverAssets(String walletId, {String? chain}) {
    return assetsClient.discover(walletId, chain: chain);
  }

  Future<AssetSummary> addCustomToken({
    required String chain,
    required String contractAddress,
    required String tokenName,
  }) {
    return assetsClient.addCustomToken(
      chain: chain,
      contractAddress: contractAddress,
      tokenName: tokenName,
    );
  }

  Future<void> removeCustomToken(String assetId) {
    return assetsClient.removeCustomToken(assetId);
  }
}
