import 'package:flutter/material.dart';

import '../../../models.dart';

class TransferAssetSelector extends StatelessWidget {
  const TransferAssetSelector({
    super.key,
    required this.assets,
    required this.selectedAssetId,
    required this.onAssetChanged,
  });

  final List<AssetSummary> assets;
  final String? selectedAssetId;
  final ValueChanged<String> onAssetChanged;

  @override
  Widget build(BuildContext context) {
    return DropdownButtonFormField<String>(
      initialValue: _selectedAssetValue,
      decoration: const InputDecoration(
        labelText: 'Asset',
        prefixIcon: Icon(Icons.account_balance_wallet_outlined),
      ),
      items: assets
          .map(
            (asset) => DropdownMenuItem(
              value: asset.id,
              child: Text(_assetLabel(asset)),
            ),
          )
          .toList(),
      onChanged: (assetId) {
        if (assetId != null) {
          onAssetChanged(assetId);
        }
      },
    );
  }

  String? get _selectedAssetValue {
    if (assets.isEmpty) {
      return null;
    }
    final selected = selectedAssetId;
    if (selected != null && assets.any((asset) => asset.id == selected)) {
      return selected;
    }
    return assets.first.id;
  }
}

String _assetLabel(AssetSummary asset) {
  final chain = asset.chain.toUpperCase();
  final balance = asset.balance.isEmpty ? '0' : asset.balance;
  return '${asset.symbol} · $chain · $balance';
}
