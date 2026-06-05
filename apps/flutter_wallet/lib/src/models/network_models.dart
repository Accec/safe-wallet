class NetworkSettingsDraft {
  const NetworkSettingsDraft({
    required this.networkName,
    required this.rpcUrl,
    required this.chainId,
    required this.currencySymbol,
    this.blockExplorerUrl,
    this.indexerEndpoint,
  });

  final String networkName;
  final String rpcUrl;
  final String chainId;
  final String currencySymbol;
  final String? blockExplorerUrl;
  final String? indexerEndpoint;
}

class NetworkSettings {
  const NetworkSettings({
    required this.chain,
    required this.networkName,
    required this.chainId,
    required this.enabled,
    required this.defaultRpcUrl,
    required this.nativeSymbol,
    required this.nativeDecimals,
    this.userRpcUrl,
    this.indexerEndpoint,
    this.explorerUrl,
  });

  final String chain;
  final String networkName;
  final String? chainId;
  final bool enabled;
  final String defaultRpcUrl;
  final String? userRpcUrl;
  final String? indexerEndpoint;
  final String? explorerUrl;
  final String nativeSymbol;
  final int nativeDecimals;

  String get displayRpcUrl => userRpcUrl ?? defaultRpcUrl;
}

class NetworkPrivacySettingsDraft {
  const NetworkPrivacySettingsDraft({
    required this.proxyEnabled,
    required this.proxyMode,
    this.proxyUrl,
  });

  final bool proxyEnabled;
  final String proxyMode;
  final String? proxyUrl;
}

class NetworkPrivacySettings {
  const NetworkPrivacySettings({
    required this.proxyEnabled,
    required this.proxyMode,
    this.proxyUrl,
  });

  final bool proxyEnabled;
  final String proxyMode;
  final String? proxyUrl;
}
