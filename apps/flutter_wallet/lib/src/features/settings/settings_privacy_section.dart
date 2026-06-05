import 'package:flutter/material.dart';

import '../../models.dart';

class SettingsPrivacySection extends StatelessWidget {
  const SettingsPrivacySection({
    super.key,
    required this.networkPrivacy,
    required this.onOpenPrivacy,
  });

  final Future<NetworkPrivacySettings> networkPrivacy;
  final ValueChanged<NetworkPrivacySettings> onOpenPrivacy;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Privacy', style: Theme.of(context).textTheme.titleMedium),
        const SizedBox(height: 8),
        FutureBuilder<NetworkPrivacySettings>(
          future: networkPrivacy,
          builder: (context, snapshot) {
            final settings = snapshot.data;
            return ListTile(
              leading: const Icon(Icons.shield_outlined),
              title: const Text('Network privacy'),
              subtitle: Text(
                settings == null
                    ? 'Loading proxy settings'
                    : settings.proxyEnabled
                    ? 'Proxy enabled'
                    : 'Direct connection',
              ),
              trailing: const Icon(Icons.chevron_right),
              onTap: settings == null ? null : () => onOpenPrivacy(settings),
            );
          },
        ),
      ],
    );
  }
}
