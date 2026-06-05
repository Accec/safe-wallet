import 'dart:async';

import 'package:flutter/material.dart';

import '../../../models.dart';
import '../../../wallet_api.dart';
import 'asset_dialog_callbacks.dart';
import 'assets_screen_actions.dart';
import 'assets_screen_body.dart';

class AssetsScreen extends StatefulWidget {
  const AssetsScreen({
    super.key,
    required this.wallet,
    required this.wallets,
    required this.assets,
    required this.onSelectedWalletChanged,
    required this.selectedChain,
    required this.onSelectedChainChanged,
    required this.onRefresh,
    required this.onDiscoverAssets,
    required this.onAddCustomToken,
    required this.onRemoveCustomToken,
  });

  final WalletSummary? wallet;
  final List<WalletSummary> wallets;
  final List<AssetSummary> assets;
  final ValueChanged<WalletSummary> onSelectedWalletChanged;
  final String? selectedChain;
  final ValueChanged<String> onSelectedChainChanged;
  final FutureOr<void> Function() onRefresh;
  final FutureOr<void> Function() onDiscoverAssets;
  final AddCustomTokenCallback onAddCustomToken;
  final RemoveCustomTokenCallback onRemoveCustomToken;

  @override
  State<AssetsScreen> createState() => _AssetsScreenState();
}

class _AssetsScreenState extends State<AssetsScreen> {
  bool _refreshing = false;
  bool _discovering = false;

  Future<void> _refreshAssets() async {
    setState(() {
      _refreshing = true;
    });
    try {
      await Future<void>.sync(widget.onRefresh);
    } catch (error) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              error is WalletApiException ? error.message : 'Refresh failed',
            ),
          ),
        );
      }
    } finally {
      if (mounted) {
        setState(() {
          _refreshing = false;
        });
      }
    }
  }

  Future<void> _discoverAssets() async {
    setState(() {
      _discovering = true;
    });
    try {
      await Future<void>.sync(widget.onDiscoverAssets);
    } catch (error) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              error is WalletApiException ? error.message : 'Discovery failed',
            ),
          ),
        );
      }
    } finally {
      if (mounted) {
        setState(() {
          _discovering = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Assets'),
        actions: [
          AssetsScreenActions(
            wallet: widget.wallet,
            selectedChain: widget.selectedChain,
            refreshing: _refreshing,
            discovering: _discovering,
            onRefresh: _refreshAssets,
            onDiscoverAssets: _discoverAssets,
            onAddCustomToken: widget.onAddCustomToken,
          ),
        ],
      ),
      body: AssetsScreenBody(
        wallet: widget.wallet,
        assets: widget.assets,
        selectedChain: widget.selectedChain,
        onRemoveCustomToken: widget.onRemoveCustomToken,
      ),
    );
  }
}
