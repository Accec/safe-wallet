import 'package:flutter/material.dart';

import '../../core/ui/busy_icon.dart';

class SettingsSecuritySection extends StatelessWidget {
  const SettingsSecuritySection({
    super.key,
    required this.biometricEnabled,
    required this.securityLoading,
    required this.securityBusy,
    required this.onBiometricChanged,
    required this.onAdvancedSecurity,
    required this.onLock,
  });

  final bool biometricEnabled;
  final bool securityLoading;
  final bool securityBusy;
  final ValueChanged<bool> onBiometricChanged;
  final VoidCallback onAdvancedSecurity;
  final VoidCallback onLock;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Security', style: Theme.of(context).textTheme.titleMedium),
        const SizedBox(height: 8),
        SwitchListTile(
          value: biometricEnabled,
          onChanged: securityLoading || securityBusy
              ? null
              : onBiometricChanged,
          secondary: const Icon(Icons.fingerprint),
          title: const Text('Biometrics'),
          subtitle: Text(biometricEnabled ? 'Enabled' : 'Disabled'),
        ),
        ListTile(
          leading: const Icon(Icons.security_outlined),
          title: const Text('Advanced security'),
          subtitle: const Text('Protected settings'),
          trailing: securityBusy
              ? const BusyIcon()
              : const Icon(Icons.chevron_right),
          onTap: securityBusy ? null : onAdvancedSecurity,
        ),
        const Divider(),
        ListTile(
          leading: const Icon(Icons.lock),
          title: const Text('Lock'),
          onTap: onLock,
        ),
      ],
    );
  }
}
