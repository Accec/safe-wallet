import 'package:flutter/material.dart';

import '../../models.dart';

class NetworkSettingsFormControllers {
  NetworkSettingsFormControllers({
    required this.networkName,
    required this.rpc,
    required this.chainId,
    required this.symbol,
    required this.explorer,
    required this.indexer,
  });

  factory NetworkSettingsFormControllers.fromNetwork(NetworkSettings? network) {
    return NetworkSettingsFormControllers(
      networkName: TextEditingController(text: network?.networkName),
      rpc: TextEditingController(text: network?.displayRpcUrl),
      chainId: TextEditingController(text: network?.chainId),
      symbol: TextEditingController(text: network?.nativeSymbol),
      explorer: TextEditingController(text: network?.explorerUrl),
      indexer: TextEditingController(text: network?.indexerEndpoint),
    );
  }

  final TextEditingController networkName;
  final TextEditingController rpc;
  final TextEditingController chainId;
  final TextEditingController symbol;
  final TextEditingController explorer;
  final TextEditingController indexer;

  NetworkSettingsDraft toDraft() {
    final explorerText = explorer.text.trim();
    final indexerText = indexer.text.trim();
    return NetworkSettingsDraft(
      networkName: networkName.text.trim(),
      rpcUrl: rpc.text.trim(),
      chainId: chainId.text.trim(),
      currencySymbol: symbol.text.trim(),
      blockExplorerUrl: explorerText.isEmpty ? null : explorerText,
      indexerEndpoint: indexerText.isEmpty ? null : indexerText,
    );
  }

  void dispose() {
    networkName.dispose();
    rpc.dispose();
    chainId.dispose();
    symbol.dispose();
    explorer.dispose();
    indexer.dispose();
  }
}
