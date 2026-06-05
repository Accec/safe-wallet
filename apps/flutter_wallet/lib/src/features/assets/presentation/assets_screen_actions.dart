import 'package:flutter/material.dart';

import '../../../core/ui/busy_icon.dart';
import '../../../models.dart';
import 'assets_dialogs.dart';

class AssetsScreenActions extends StatelessWidget {
  const AssetsScreenActions({
    super.key,
    required this.wallet,
    required this.selectedChain,
    required this.refreshing,
    required this.discovering,
    required this.onRefresh,
    required this.onDiscoverAssets,
    required this.onAddCustomToken,
  });

  final WalletSummary? wallet;
  final String? selectedChain;
  final bool refreshing;
  final bool discovering;
  final VoidCallback onRefresh;
  final VoidCallback onDiscoverAssets;
  final AddCustomTokenCallback onAddCustomToken;

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        IconButton(
          tooltip: 'Refresh assets',
          onPressed: wallet == null || refreshing ? null : onRefresh,
          icon: refreshing ? const BusyIcon() : const Icon(Icons.refresh),
        ),
        IconButton(
          tooltip: 'Discover assets',
          onPressed: wallet == null || discovering ? null : onDiscoverAssets,
          icon: discovering
              ? const BusyIcon()
              : const Icon(Icons.manage_search),
        ),
        IconButton(
          tooltip: 'Add token',
          onPressed: wallet == null || selectedChain == null
              ? null
              : () => showAddTokenDialog(
                  context: context,
                  selectedChain: selectedChain!,
                  onAddCustomToken: onAddCustomToken,
                ),
          icon: const Icon(Icons.add_circle_outline),
        ),
      ],
    );
  }
}
