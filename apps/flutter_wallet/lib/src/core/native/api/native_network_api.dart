import '../../../models.dart';
import '../clients/native_network_client.dart';

mixin NativeNetworkApi {
  NativeNetworkClient get networkClient;

  Future<NetworkPrivacySettings> networkPrivacySettings() {
    return networkClient.privacySettings();
  }

  Future<void> saveNetworkPrivacySettings(NetworkPrivacySettingsDraft draft) {
    return networkClient.savePrivacySettings(draft);
  }

  Future<void> testProxyConnection(NetworkPrivacySettingsDraft draft) {
    return networkClient.testProxyConnection(draft);
  }

  Future<List<NetworkSettings>> listNetworkSettings() {
    return networkClient.listSettings();
  }

  Future<void> updateChainRpc({required String chain, required String rpcUrl}) {
    return networkClient.updateChainRpc(chain: chain, rpcUrl: rpcUrl);
  }

  Future<void> saveNetworkSettings(NetworkSettingsDraft draft) {
    return networkClient.saveSettings(draft);
  }

  Future<void> updateIndexerSettings({
    required String chain,
    required String endpoint,
    String? apiKey,
  }) {
    return networkClient.updateIndexerSettings(
      chain: chain,
      endpoint: endpoint,
      apiKey: apiKey,
    );
  }
}
