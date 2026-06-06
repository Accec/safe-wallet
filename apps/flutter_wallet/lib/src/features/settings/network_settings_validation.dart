import '../../models.dart';

String? networkSettingsValidationError(NetworkSettingsDraft draft) {
  if (draft.networkName.isEmpty) {
    return 'Network name is required';
  }
  if (!_isSupportedChainId(draft.chainId)) {
    return 'Unsupported chain ID';
  }
  if (!_isHttpUrl(draft.rpcUrl)) {
    return 'Default RPC URL must be http or https';
  }
  if (draft.currencySymbol.isEmpty) {
    return 'Currency symbol is required';
  }
  final explorerUrl = draft.blockExplorerUrl;
  if (explorerUrl != null && !_isHttpUrl(explorerUrl)) {
    return 'Block explorer URL must be http or https';
  }
  final indexerEndpoint = draft.indexerEndpoint;
  if (indexerEndpoint != null && !_isHttpUrl(indexerEndpoint)) {
    return 'Scan URL must be http or https';
  }
  return null;
}

bool _isSupportedChainId(String chainId) {
  return const {'1', '10', '56', '137', '42161', '728126428'}.contains(chainId);
}

bool _isHttpUrl(String value) {
  final uri = Uri.tryParse(value);
  return uri != null &&
      uri.hasScheme &&
      (uri.scheme == 'http' || uri.scheme == 'https') &&
      uri.host.isNotEmpty;
}
