import 'package:flutter/material.dart';

import 'network_settings_form_controllers.dart';
import 'network_settings_presets.dart';
import 'network_settings_url_input.dart';

class NetworkSettingsForm extends StatelessWidget {
  const NetworkSettingsForm({
    super.key,
    required this.controllers,
    required this.busy,
    required this.errorText,
  });

  final NetworkSettingsFormControllers controllers;
  final bool busy;
  final String? errorText;

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 520),
      child: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: controllers.networkName,
              enabled: !busy,
              decoration: const InputDecoration(
                labelText: 'Network name',
                hintText: 'Enter network name',
              ),
            ),
            const SizedBox(height: 12),
            NetworkSettingsUrlInput(
              controller: controllers.rpc,
              enabled: !busy,
              labelText: 'Default RPC URL',
              presets: rpcUrlPresets,
            ),
            const SizedBox(height: 12),
            TextField(
              controller: controllers.chainId,
              enabled: !busy,
              keyboardType: TextInputType.number,
              decoration: const InputDecoration(
                labelText: 'Chain ID',
                hintText: 'Enter chain ID',
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: controllers.symbol,
              enabled: !busy,
              decoration: const InputDecoration(
                labelText: 'Currency symbol',
                hintText: 'Enter symbol',
              ),
            ),
            const SizedBox(height: 12),
            NetworkSettingsUrlInput(
              controller: controllers.explorer,
              enabled: !busy,
              labelText: 'Block explorer URL',
              presets: explorerUrlPresets,
            ),
            const SizedBox(height: 12),
            NetworkSettingsUrlInput(
              controller: controllers.indexer,
              enabled: !busy,
              labelText: 'Indexer API URL',
              presets: indexerUrlPresets,
            ),
            if (errorText != null) ...[
              const SizedBox(height: 12),
              Text(
                errorText!,
                style: TextStyle(color: Theme.of(context).colorScheme.error),
              ),
            ],
          ],
        ),
      ),
    );
  }
}
