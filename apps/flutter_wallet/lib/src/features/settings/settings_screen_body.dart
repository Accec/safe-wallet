import 'package:flutter/material.dart';

import '../../models.dart';
import 'settings_sections.dart';

class SettingsScreenBody extends StatelessWidget {
  const SettingsScreenBody({
    super.key,
    required this.networks,
    required this.networkPrivacy,
    required this.biometricEnabled,
    required this.securityLoading,
    required this.securityBusy,
    required this.onEditNetwork,
    required this.onAddNetwork,
    required this.onOpenPrivacy,
    required this.onBiometricChanged,
    required this.onAdvancedSecurity,
    required this.onLock,
  });

  final Future<List<NetworkSettings>> networks;
  final Future<NetworkPrivacySettings> networkPrivacy;
  final bool biometricEnabled;
  final bool securityLoading;
  final bool securityBusy;
  final ValueChanged<NetworkSettings> onEditNetwork;
  final VoidCallback onAddNetwork;
  final ValueChanged<NetworkPrivacySettings> onOpenPrivacy;
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
