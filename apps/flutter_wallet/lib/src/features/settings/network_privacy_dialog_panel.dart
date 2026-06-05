import 'package:flutter/material.dart';

import '../../models.dart';
import '../../wallet_api.dart';
import 'network_privacy_dialog_actions.dart';
import 'network_privacy_form.dart';
import 'network_privacy_state.dart';

class NetworkPrivacyDialogPanel extends StatefulWidget {
  const NetworkPrivacyDialogPanel({
    super.key,
    required this.api,
    required this.settings,
  });

  final WalletApi api;
  final NetworkPrivacySettings settings;

  @override
  State<NetworkPrivacyDialogPanel> createState() =>
      _NetworkPrivacyDialogPanelState();
}

class _NetworkPrivacyDialogPanelState extends State<NetworkPrivacyDialogPanel> {
  late final NetworkPrivacyDraftController _draftController;
  bool _busy = false;
  String? _statusText;
  bool _statusIsError = false;

  @override
  void initState() {
    super.initState();
    _draftController = NetworkPrivacyDraftController.fromSettings(
      widget.settings,
    );
  }

  @override
  void dispose() {
    _draftController.dispose();
    super.dispose();
  }

  void _setProxyEnabled(bool value) {
    setState(() {
      _draftController.setProxyEnabled(value);
      _statusText = null;
    });
  }

  void _setProxyMode(String mode) {
    setState(() {
      _draftController.setProxyMode(mode);
      _statusText = null;
    });
  }

  Future<void> _testConnection() async {
    _beginNetworkAction();
    try {
      await widget.api.testProxyConnection(_draftController.toDraft());
      if (!mounted) {
        return;
      }
      setState(() {
        _statusText = 'Proxy connection succeeded.';
        _statusIsError = false;
      });
    } catch (error) {
      if (!mounted) {
        return;
      }
      setState(() {
        _statusText = error is WalletApiException
            ? error.message
            : 'Proxy connection failed';
        _statusIsError = true;
      });
    } finally {
      _endNetworkAction();
    }
  }

  Future<void> _save() async {
    _beginNetworkAction();
    try {
      await widget.api.saveNetworkPrivacySettings(_draftController.toDraft());
      if (!mounted) {
        return;
      }
      Navigator.of(context).pop(true);
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Network privacy settings saved.')),
      );
    } catch (error) {
      if (!mounted) {
        return;
      }
      setState(() {
        _statusText = error is WalletApiException
            ? error.message
            : 'Invalid proxy settings';
        _statusIsError = true;
      });
    } finally {
      _endNetworkAction();
    }
  }

  void _beginNetworkAction() {
    setState(() {
      _busy = true;
      _statusText = null;
      _statusIsError = false;
    });
  }

  void _endNetworkAction() {
    if (mounted) {
      setState(() {
        _busy = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Network privacy'),
      content: NetworkPrivacyForm(
        controller: _draftController,
        busy: _busy,
        statusText: _statusText,
        statusIsError: _statusIsError,
        onProxyEnabledChanged: _setProxyEnabled,
        onProxyModeChanged: _setProxyMode,
      ),
      actions: [
        NetworkPrivacyDialogActions(
          busy: _busy,
          proxyEnabled: _draftController.proxyEnabled,
          onCancel: () => Navigator.of(context).pop(false),
          onTestConnection: _testConnection,
          onSave: _save,
        ),
      ],
    );
  }
}
