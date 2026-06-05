import 'package:flutter/material.dart';

import '../../app_update.dart';
import '../../models.dart';
import 'settings_sections.dart';

class SettingsScreenBody extends StatelessWidget {
  const SettingsScreenBody({
    super.key,
    required this.networks,
    required this.networkPrivacy,
    required this.updateInfo,
    required this.biometricEnabled,
    required this.securityLoading,
    required this.securityBusy,
    required this.checkingUpdate,
    required this.updating,
    required this.updateProgress,
    required this.onEditNetwork,
    required this.onAddNetwork,
    required this.onOpenPrivacy,
    required this.onCheckForUpdates,
    required this.onRunUpdateAction,
    required this.onBiometricChanged,
    required this.onAdvancedSecurity,
    required this.onLock,
  });

  final Future<List<NetworkSettings>> networks;
  final Future<NetworkPrivacySettings> networkPrivacy;
  final AppUpdateInfo? updateInfo;
  final bool biometricEnabled;
  final bool securityLoading;
  final bool securityBusy;
  final bool checkingUpdate;
  final bool updating;
  final double? updateProgress;
  final ValueChanged<NetworkSettings> onEditNetwork;
  final VoidCallback onAddNetwork;
  final ValueChanged<NetworkPrivacySettings> onOpenPrivacy;
  final VoidCallback onCheckForUpdates;
  final VoidCallback onRunUpdateAction;
  final ValueChanged<bool> onBiometricChanged;
  final VoidCallback onAdvancedSecurity;
  final VoidCallback onLock;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Settings')),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          SettingsNetworkSection(
            networks: networks,
            onEditNetwork: onEditNetwork,
            onAddNetwork: onAddNetwork,
          ),
          const Divider(height: 32),
          SettingsPrivacySection(
            networkPrivacy: networkPrivacy,
            onOpenPrivacy: onOpenPrivacy,
          ),
          const Divider(height: 32),
          SettingsUpdateSection(
            updateInfo: updateInfo,
            checking: checkingUpdate,
            updating: updating,
            progress: updateProgress,
            onCheck: onCheckForUpdates,
            onRunUpdateAction: onRunUpdateAction,
          ),
          const Divider(height: 32),
          SettingsSecuritySection(
            biometricEnabled: biometricEnabled,
            securityLoading: securityLoading,
            securityBusy: securityBusy,
            onBiometricChanged: onBiometricChanged,
            onAdvancedSecurity: onAdvancedSecurity,
            onLock: onLock,
          ),
        ],
      ),
    );
  }
}
