import 'dart:async';

import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';

import '../../../core/ui/busy_icon.dart';
import '../../../models.dart';
import 'activity_list.dart';

class ActivityScreen extends StatefulWidget {
  const ActivityScreen({
    super.key,
    required this.activity,
    this.wallets = const [],
    this.selectedWallet,
    this.onSelectedWalletChanged,
    this.onSync,
  });

  final List<ActivitySummary> activity;
  final List<WalletSummary> wallets;
  final WalletSummary? selectedWallet;
  final ValueChanged<WalletSummary>? onSelectedWalletChanged;
  final Future<void> Function()? onSync;

  @override
  State<ActivityScreen> createState() => _ActivityScreenState();
}

class _ActivityScreenState extends State<ActivityScreen> {
  bool _syncing = false;

  Future<void> _sync() async {
    final onSync = widget.onSync;
    if (onSync == null || _syncing) {
      return;
    }
    setState(() {
      _syncing = true;
    });
    try {
      await onSync();
    } catch (error) {
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text('Activity sync failed: $error')));
      }
    } finally {
      if (mounted) {
        setState(() {
          _syncing = false;
        });
      }
    }
  }

  Future<void> _openActivityUrl(Uri url) async {
    try {
      final opened = await launchUrl(url, mode: LaunchMode.externalApplication);
      if (!opened && mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Unable to open scan URL')),
        );
      }
    } catch (error) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Unable to open scan URL: $error')),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Activity'),
        actions: [
          IconButton(
            tooltip: 'Sync history',
            onPressed: widget.selectedWallet == null || _syncing ? null : _sync,
            icon: _syncing ? const BusyIcon() : const Icon(Icons.sync),
          ),
        ],
      ),
      body: ActivityList(
        activity: widget.activity,
        onOpenActivityUrl: (url) {
          unawaited(_openActivityUrl(url));
        },
      ),
    );
  }
}
