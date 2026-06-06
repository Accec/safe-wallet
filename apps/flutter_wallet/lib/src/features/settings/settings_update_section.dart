import 'package:flutter/material.dart';

import '../../app_update.dart';
import '../../core/ui/busy_icon.dart';

class SettingsUpdateSection extends StatelessWidget {
  const SettingsUpdateSection({
    super.key,
    required this.updateInfo,
    required this.checking,
    required this.updating,
    required this.progress,
    required this.onCheck,
    required this.onRunUpdateAction,
  });

  final AppUpdateInfo? updateInfo;
  final bool checking;
  final bool updating;
  final double? progress;
  final VoidCallback onCheck;
  final VoidCallback onRunUpdateAction;

  @override
  Widget build(BuildContext context) {
    final info = updateInfo;
    final showDownloadProgress =
        info?.asset != null && (updating || progress != null);
    final rawProgress = progress;
    final progressValue = rawProgress == null || rawProgress <= 0
        ? null
        : rawProgress.clamp(0.0, 1.0).toDouble();
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('App update', style: Theme.of(context).textTheme.titleMedium),
        const SizedBox(height: 8),
        ListTile(
          leading: const Icon(Icons.system_update_alt),
          title: const Text('Safe Wallet'),
          subtitle: Text(_statusText(info)),
        ),
        if (showDownloadProgress) ...[
          const SizedBox(height: 8),
          LinearProgressIndicator(value: progressValue),
        ],
        const SizedBox(height: 8),
        Wrap(
          spacing: 12,
          runSpacing: 12,
          children: [
            OutlinedButton.icon(
              onPressed: checking || updating ? null : onCheck,
              icon: checking
                  ? const BusyIcon(dimension: 18)
                  : const Icon(Icons.manage_search),
              label: const Text('Check for updates'),
            ),
            if (info?.updateAvailable == true)
              FilledButton.icon(
                onPressed: updating ? null : onRunUpdateAction,
                icon: updating
                    ? const BusyIcon(dimension: 18)
                    : const Icon(Icons.download),
                label: Text(info!.actionLabel),
              ),
          ],
        ),
      ],
    );
  }

  String _statusText(AppUpdateInfo? info) {
    if (info == null) {
      return 'Check GitHub Releases for a newer app version.';
    }
    if (!info.updateAvailable) {
      return 'Current version ${info.currentVersion} is up to date.';
    }
    return 'Version ${info.latestVersion} is available.';
  }
}
