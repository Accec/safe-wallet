import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../app_update.dart';
import '../../core/state/wallet_providers.dart';
import '../../models.dart';
import 'network_privacy_dialog.dart';
import 'network_settings_dialog.dart';
import 'settings_screen_body.dart';
import 'settings_security_actions.dart';
import 'settings_update_actions.dart';

class SettingsScreen extends ConsumerStatefulWidget {
  const SettingsScreen({super.key});

  @override
  ConsumerState<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends ConsumerState<SettingsScreen> {
  late Future<List<NetworkSettings>> _networks;
  late Future<NetworkPrivacySettings> _networkPrivacy;
  AppUpdateInfo? _updateInfo;
  bool _securityLoading = true;
  bool _securityBusy = false;
  bool _biometricEnabled = false;
  bool _checkingUpdate = false;
  bool _updating = false;
  double? _updateProgress;

  SettingsSecurityActions get _securityActions => SettingsSecurityActions(
    context: context,
    ref: ref,
    isMounted: () => mounted,
    setBiometricEnabled: _setBiometricEnabled,
    setSecurityBusy: _setSecurityBusy,
  );

  SettingsUpdateActions get _updateActions => SettingsUpdateActions(
    context: context,
    ref: ref,
    isMounted: () => mounted,
    setCheckingUpdate: _setCheckingUpdate,
    setUpdating: _setUpdating,
    setUpdateProgress: _setUpdateProgress,
    setUpdateInfo: _setUpdateInfo,
  );

  @override
  void initState() {
    super.initState();
    _networks = ref.read(walletApiProvider).listNetworkSettings();
    _networkPrivacy = ref.read(walletApiProvider).networkPrivacySettings();
    _loadSecurityStatus();
  }

  void _reloadNetworks() {
    setState(() {
      _networks = ref.read(walletApiProvider).listNetworkSettings();
    });
  }

  void _reloadNetworkPrivacy() {
    setState(() {
      _networkPrivacy = ref.read(walletApiProvider).networkPrivacySettings();
    });
  }

  void _setSecurityBusy(bool busy) {
    setState(() {
      _securityBusy = busy;
    });
  }

  void _setBiometricEnabled(bool enabled) {
    setState(() {
      _biometricEnabled = enabled;
    });
  }

  void _setCheckingUpdate(bool checking) {
    setState(() {
      _checkingUpdate = checking;
    });
  }

  void _setUpdating(bool updating) {
    setState(() {
      _updating = updating;
    });
  }

  void _setUpdateProgress(double? progress) {
    setState(() {
      _updateProgress = progress;
    });
  }

  void _setUpdateInfo(AppUpdateInfo? info) {
    setState(() {
      _updateInfo = info;
    });
  }

  Future<void> _loadSecurityStatus() async {
    try {
      final status = await ref.read(walletApiProvider).appStatus();
      if (!mounted) {
        return;
      }
      setState(() {
        _biometricEnabled = status.biometricEnabled;
        _securityLoading = false;
      });
    } catch (_) {
      if (!mounted) {
        return;
      }
      setState(() {
        _securityLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final api = ref.read(walletApiProvider);
    final securityActions = _securityActions;
    final updateActions = _updateActions;

    return SettingsScreenBody(
      networks: _networks,
      networkPrivacy: _networkPrivacy,
      updateInfo: _updateInfo,
      biometricEnabled: _biometricEnabled,
      securityLoading: _securityLoading,
      securityBusy: _securityBusy,
      checkingUpdate: _checkingUpdate,
      updating: _updating,
      updateProgress: _updateProgress,
      onEditNetwork: (network) => showNetworkSettingsDialog(
        context: context,
        api: api,
        network: network,
        onSaved: _reloadNetworks,
      ),
      onAddNetwork: () => showNetworkSettingsDialog(
        context: context,
        api: api,
        onSaved: _reloadNetworks,
      ),
      onOpenPrivacy: (settings) => showNetworkPrivacyDialog(
        context: context,
        api: api,
        settings: settings,
        onSaved: _reloadNetworkPrivacy,
      ),
      onCheckForUpdates: updateActions.checkForUpdates,
      onRunUpdateAction: () => updateActions.runUpdateAction(_updateInfo),
      onBiometricChanged: securityActions.toggleBiometrics,
      onAdvancedSecurity: securityActions.openAdvancedSecurity,
      onLock: securityActions.lock,
    );
  }
}
