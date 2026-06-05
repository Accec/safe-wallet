import '../../../models.dart';
import '../../../wallet_api.dart';
import '../native_response_parsers.dart';
import '../native_wallet_gateway.dart';

class NativeNetworkClient {
  const NativeNetworkClient(this._gateway);

  final NativeWalletGateway _gateway;

  Future<NetworkPrivacySettings> privacySettings() async {
    final body = await _gateway.invokeDb(
      domain: 'network',
      action: 'get_privacy',
    );
    return networkPrivacySettingsFromNativeJson(body);
  }

  Future<void> savePrivacySettings(NetworkPrivacySettingsDraft draft) {
    return _gateway.invokeDb(
      domain: 'network',
      action: 'save_privacy',
      payload: <String, Object?>{
        'proxy_enabled': draft.proxyEnabled,
        'proxy_mode': draft.proxyMode,
        'proxy_url': draft.proxyUrl,
      },
    );
  }

  Future<void> testProxyConnection(NetworkPrivacySettingsDraft draft) {
    return _gateway.invokeDb(
      domain: 'network',
      action: 'test_proxy',
      payload: <String, Object?>{
        'proxy_enabled': draft.proxyEnabled,
        'proxy_mode': draft.proxyMode,
        'proxy_url': draft.proxyUrl,
      },
    );
  }

  Future<List<NetworkSettings>> listSettings() async {
    final body = await _gateway.invokeDb(
      domain: 'network',
      action: 'list_settings',
    );
    if (body is! List<dynamic>) {
      throw const WalletApiException('Invalid native response');
    }
    return body.map(networkSettingsFromNativeJson).toList();
  }

  Future<void> updateChainRpc({required String chain, required String rpcUrl}) {
    return _gateway.invokeDb(
      domain: 'network',
      action: 'update_chain_rpc',
      payload: <String, Object?>{'chain': chain, 'rpc_url': rpcUrl},
    );
  }

  Future<void> saveSettings(NetworkSettingsDraft draft) {
    return _gateway.invokeDb(
      domain: 'network',
      action: 'save_settings',
      payload: <String, Object?>{
        'network_name': draft.networkName,
        'rpc_url': draft.rpcUrl,
        'chain_id': draft.chainId,
        'currency_symbol': draft.currencySymbol,
        'block_explorer_url': draft.blockExplorerUrl,
        'indexer_endpoint': draft.indexerEndpoint,
      },
    );
  }

  Future<void> updateIndexerSettings({
    required String chain,
    required String endpoint,
    String? apiKey,
  }) {
    return _gateway.invokeDb(
      domain: 'network',
      action: 'update_indexer',
      payload: <String, Object?>{
        'chain': chain,
        'endpoint': endpoint,
        'api_key': apiKey,
      },
    );
  }
}
