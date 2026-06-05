import '../../../models.dart';
import '../wallet_api_contract.dart';
import 'demo_wallet_store.dart';

class DemoNetworkClient {
  const DemoNetworkClient(this._store);

  final DemoWalletStore _store;

  Future<NetworkPrivacySettings> privacySettings() async {
    return _store.networkPrivacySettings;
  }

  Future<void> savePrivacySettings(NetworkPrivacySettingsDraft draft) async {
    _store.networkPrivacySettings = NetworkPrivacySettings(
      proxyEnabled: draft.proxyEnabled,
      proxyMode: draft.proxyMode,
      proxyUrl: draft.proxyEnabled && draft.proxyMode == 'tor'
          ? (draft.proxyUrl ?? 'socks5://127.0.0.1:9050')
          : draft.proxyUrl,
    );
  }

  Future<void> testProxyConnection(NetworkPrivacySettingsDraft draft) async {}

  Future<List<NetworkSettings>> listSettings() async {
    return _store.networks;
  }

  Future<void> updateChainRpc({
    required String chain,
    required String rpcUrl,
  }) async {}

  Future<void> saveSettings(NetworkSettingsDraft draft) async {
    final chain = _chainForNetworkId(draft.chainId);
    if (chain == null) {
      throw const WalletApiException('Invalid network settings');
    }
    final existingIndex = _store.networks.indexWhere(
      (network) => network.chain == chain,
    );
    final existing = existingIndex == -1
        ? null
        : _store.networks[existingIndex];
    final network = NetworkSettings(
      chain: chain,
      networkName: draft.networkName,
      chainId: draft.chainId,
      enabled: true,
      defaultRpcUrl: existing?.defaultRpcUrl ?? draft.rpcUrl,
      userRpcUrl: draft.rpcUrl,
      indexerEndpoint: draft.indexerEndpoint,
      explorerUrl: draft.blockExplorerUrl,
      nativeSymbol: draft.currencySymbol,
      nativeDecimals: existing?.nativeDecimals ?? 18,
    );
    if (existingIndex == -1) {
      _store.networks.add(network);
    } else {
      _store.networks[existingIndex] = network;
    }
  }

  Future<void> updateIndexerSettings({
    required String chain,
    required String endpoint,
    String? apiKey,
  }) async {}
}

String? _chainForNetworkId(String chainId) {
  return switch (chainId) {
    '1' => 'ethereum',
    '10' => 'optimism',
    '56' => 'bsc',
    '137' => 'polygon',
    '42161' => 'arbitrum',
    '728126428' => 'tron',
    _ => null,
  };
}
