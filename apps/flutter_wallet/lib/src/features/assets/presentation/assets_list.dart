import 'package:flutter/material.dart';

import '../../../core/ui/chain_selector.dart';
import '../../../core/ui/screen_motion.dart';
import '../../../models.dart';
import 'assets_dialogs.dart';

class AssetsList extends StatelessWidget {
  const AssetsList({
    super.key,
    required this.wallet,
    required this.assets,
    required this.selectedChain,
    required this.onRemoveCustomToken,
  });

  final WalletSummary? wallet;
  final List<AssetSummary> assets;
  final String? selectedChain;
  final RemoveCustomTokenCallback onRemoveCustomToken;

  @override
  Widget build(BuildContext context) {
    final visibleAssets = selectedChain == null
        ? assets
        : assets.where((asset) => asset.chain == selectedChain).toList();

    return ListView(
      padding: const EdgeInsets.all(16),
      children: _buildRows(context, visibleAssets),
    );
  }

  List<Widget> _buildRows(
    BuildContext context,
    List<AssetSummary> visibleAssets,
  ) {
    if (wallet == null) {
      return const [
        StaggeredListItem(
          index: 0,
          child: ListTile(
            leading: Icon(Icons.token_outlined),
            title: Text('No wallet selected'),
            subtitle: Text('Create or import a wallet first.'),
          ),
        ),
      ];
    }
    if (visibleAssets.isEmpty) {
      return const [
        StaggeredListItem(
          index: 0,
          child: ListTile(
            leading: Icon(Icons.token_outlined),
            title: Text('No assets'),
            subtitle: Text('Assets for the selected chain appear here.'),
          ),
        ),
      ];
    }

    return [
      for (var index = 0; index < visibleAssets.length; index++)
        StaggeredListItem(
          index: index,
          child: AssetTile(
            asset: visibleAssets[index],
            onRemoveCustomToken: onRemoveCustomToken,
          ),
        ),
    ];
  }
}

class AssetTile extends StatelessWidget {
  const AssetTile({
    super.key,
    required this.asset,
    required this.onRemoveCustomToken,
  });

  final AssetSummary asset;
  final RemoveCustomTokenCallback onRemoveCustomToken;

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: CircleAvatar(child: Text(asset.symbol.characters.first)),
      title: Text(asset.symbol),
      subtitle: Text(
        '${chainLabel(asset.chain)} · ${asset.name} · ${asset.kind}',
      ),
      trailing: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text('${asset.balance} ${asset.symbol}'),
          if (asset.contractAddress != null) ...[
            const SizedBox(width: 8),
            IconButton(
              tooltip: 'Remove token',
              onPressed: () => showRemoveTokenDialog(
                context: context,
                asset: asset,
                onRemoveCustomToken: onRemoveCustomToken,
              ),
              icon: const Icon(Icons.delete_outline),
            ),
          ],
        ],
      ),
    );
  }
}
