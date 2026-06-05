import 'package:flutter/material.dart';

import 'unlock_screen_body.dart';

class UnlockScreen extends StatefulWidget {
  const UnlockScreen({
    super.key,
    this.initialized = true,
    this.biometricEnabled = false,
    this.onSetup,
    this.onBiometricUnlock,
    required this.onUnlock,
  });

  final bool initialized;
  final bool biometricEnabled;
  final Future<void> Function(String password)? onSetup;
  final Future<void> Function()? onBiometricUnlock;
  final Future<void> Function(String password) onUnlock;

  @override
  State<UnlockScreen> createState() => _UnlockScreenState();
}

class _UnlockScreenState extends State<UnlockScreen> {
  final TextEditingController _passwordController = TextEditingController();
  bool _busy = false;
  bool _biometricBusy = false;
  String? _error;

  @override
  void dispose() {
    _passwordController.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    if (_busy || _biometricBusy) {
      return;
    }
    setState(() {
      _busy = true;
      _error = null;
    });

    try {
      if (widget.initialized) {
        await widget.onUnlock(_passwordController.text);
      } else {
        await widget.onSetup!(_passwordController.text);
      }
    } catch (error) {
      if (!mounted) {
        return;
      }
      setState(() {
        _error = 'Wallet action failed';
      });
    } finally {
      if (mounted) {
        setState(() {
          _busy = false;
        });
      }
    }
  }

  Future<void> _useBiometrics() async {
    final onBiometricUnlock = widget.onBiometricUnlock;
    if (onBiometricUnlock == null || _busy || _biometricBusy) {
      return;
    }
    setState(() {
      _biometricBusy = true;
      _error = null;
    });
    try {
      await onBiometricUnlock();
    } catch (error) {
      if (!mounted) {
        return;
      }
      setState(() {
        _error = 'Wallet action failed';
      });
    } finally {
      if (mounted) {
        setState(() {
          _biometricBusy = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return UnlockScreenBody(
      initialized: widget.initialized,
      biometricEnabled: widget.biometricEnabled,
      hasBiometricUnlock: widget.onBiometricUnlock != null,
      passwordController: _passwordController,
      busy: _busy,
      biometricBusy: _biometricBusy,
      error: _error,
      onSubmit: _submit,
      onBiometricUnlock: _useBiometrics,
    );
  }
}
