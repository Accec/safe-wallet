import 'package:flutter/material.dart';

import 'network_privacy_state.dart';

class NetworkPrivacyForm extends StatelessWidget {
  const NetworkPrivacyForm({
    super.key,
    required this.controller,
    required this.busy,
    required this.statusText,
    required this.statusIsError,
    required this.onProxyEnabledChanged,
    required this.onProxyModeChanged,
  });

  final NetworkPrivacyDraftController controller;
  final bool busy;
  final String? statusText;
  final bool statusIsError;
  final ValueChanged<bool> onProxyEnabledChanged;
  final ValueChanged<String> onProxyModeChanged;

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 480),
      child: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Text(
              'Routes Safe Wallet RPC and indexer requests through a proxy. This can reduce IP exposure to network services, but it does not guarantee anonymity.',
            ),
            const SizedBox(height: 12),
            SwitchListTile(
              contentPadding: EdgeInsets.zero,
              value: controller.proxyEnabled,
              onChanged: busy ? null : onProxyEnabledChanged,
              title: const Text('Use proxy'),
            ),
            const SizedBox(height: 8),
            SegmentedButton<String>(
              segments: const [
                ButtonSegment(
                  value: 'custom',
                  icon: Icon(Icons.tune_outlined),
                  label: Text('Custom proxy'),
                ),
                ButtonSegment(
                  value: 'tor',
                  icon: Icon(Icons.route_outlined),
                  label: Text('Tor'),
                ),
              ],
              selected: {controller.proxyMode},
              onSelectionChanged: busy || !controller.proxyEnabled
                  ? null
                  : (values) => onProxyModeChanged(values.single),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: controller.proxyUrl,
              enabled: !busy && controller.proxyEnabled,
              decoration: InputDecoration(
                labelText: 'Proxy URL',
                hintText: controller.proxyMode == 'tor'
                    ? defaultTorProxyUrl
                    : 'http://127.0.0.1:8080',
                prefixIcon: const Icon(Icons.link_outlined),
              ),
            ),
            if (controller.proxyMode == 'tor') ...[
              const SizedBox(height: 8),
              const Text(
                'Requires Tor or a compatible SOCKS proxy running locally.',
              ),
            ],
            if (statusText != null) ...[
              const SizedBox(height: 12),
              Text(
                statusText!,
                style: TextStyle(
                  color: statusIsError
                      ? Theme.of(context).colorScheme.error
                      : Theme.of(context).colorScheme.primary,
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }
}
