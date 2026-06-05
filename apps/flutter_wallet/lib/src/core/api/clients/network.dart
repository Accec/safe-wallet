import '../../../models.dart';
import '../../../wallet_api.dart';

class NetworkClient {
  const NetworkClient(this._api);

  final WalletApi _api;

  Future<NetworkPrivacySettings> privacySettings() =>
      _api.networkPrivacySettings();
  Future<void> savePrivacySettings(NetworkPrivacySettingsDraft draft) =>
      _api.saveNetworkPrivacySettings(draft);
  Future<void> testProxyConnection(NetworkPrivacySettingsDraft draft) =>
      _api.testProxyConnection(draft);
  Future<List<NetworkSettings>> listSettings() => _api.listNetworkSettings();
  Future<void> saveSettings(NetworkSettingsDraft draft) =>
      _api.saveNetworkSettings(draft);
  Future<void> updateChainRpc({
    required String chain,
    required String rpcUrl,
  }) => _api.updateChainRpc(chain: chain, rpcUrl: rpcUrl);
  Future<void> updateIndexerSettings({
    required String chain,
    required String endpoint,
    String? apiKey,
  }) => _api.updateIndexerSettings(
    chain: chain,
    endpoint: endpoint,
    apiKey: apiKey,
  );
}
