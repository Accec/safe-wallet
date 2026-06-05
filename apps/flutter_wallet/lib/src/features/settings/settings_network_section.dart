import 'package:flutter/material.dart';

import '../../core/ui/busy_icon.dart';
import '../../core/ui/chain_selector.dart';
import '../../models.dart';

class SettingsNetworkSection extends StatelessWidget {
  const SettingsNetworkSection({
    super.key,
    required this.networks,
    required this.onEditNetwork,
    required this.onAddNetwork,
  });

  final Future<List<NetworkSettings>> networks;
  final ValueChanged<NetworkSettings> onEditNetwork;
  final VoidCallback onAddNetwork;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Networks', style: Theme.of(context).textTheme.titleMedium),
        const SizedBox(height: 8),
        FutureBuilder<List<NetworkSettings>>(
          future: networks,
          builder: (context, snapshot) {
            if (snapshot.connectionState != ConnectionState.done) {
              return const ListTile(
                leading: SizedBox(
                  width: 24,
                  height: 24,
                  child: BusyIcon(dimension: 18),
                ),
                title: Text('Loading networks'),
              );
            }
            if (snapshot.hasError) {
              return const ListTile(
                leading: Icon(Icons.error_outline),
                title: Text('Unable to load networks'),
              );
            }
            final networks = snapshot.data ?? const <NetworkSettings>[];
            if (networks.isEmpty) {
              return const ListTile(
                leading: Icon(Icons.hub_outlined),
                title: Text('No networks'),
              );
            }
            return Column(
              children: [
                for (final network in networks)
                  ListTile(
                    leading: const Icon(Icons.hub_outlined),
                    title: Text(
                      network.networkName.isEmpty
                          ? chainLabel(network.chain)
                          : network.networkName,
                    ),
                    subtitle: Text(
                      '${chainLabel(network.chain)} · ${network.displayRpcUrl}',
                      overflow: TextOverflow.ellipsis,
                    ),
                    trailing: const Icon(Icons.edit_outlined),
                    onTap: () => onEditNetwork(network),
                  ),
              ],
            );
          },
        ),
        const Divider(height: 32),
        ListTile(
          leading: const Icon(Icons.hub_outlined),
          title: const Text('Add network'),
          subtitle: const Text(
            'Configure RPC, chain ID, symbol, and explorer.',
          ),
          trailing: const Icon(Icons.add_circle_outline),
          onTap: onAddNetwork,
        ),
      ],
    );
  }
}
