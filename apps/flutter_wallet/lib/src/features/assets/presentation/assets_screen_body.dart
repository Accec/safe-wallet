import 'package:flutter/material.dart';

import '../../../models.dart';
import 'asset_dialog_callbacks.dart';
import 'assets_list.dart';

class AssetsScreenBody extends StatelessWidget {
  const AssetsScreenBody({
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
    return AssetsList(
      wallet: wallet,
      assets: assets,
      selectedChain: selectedChain,
      onRemoveCustomToken: onRemoveCustomToken,
    );
  }
}
