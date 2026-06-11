import 'package:flutter/widgets.dart';

import '../../../models.dart';

class TransferDraftBuilder {
  const TransferDraftBuilder({
    required this.wallet,
    required this.assets,
    required this.selectedAssetId,
    required this.recipientController,
    required this.amountController,
    required this.blockIfEnergyInsufficient,
  });

  final WalletSummary? wallet;
  final List<AssetSummary> assets;
  final String? selectedAssetId;
  final TextEditingController recipientController;
  final TextEditingController amountController;
  final bool blockIfEnergyInsufficient;

  static String? firstAvailableAssetId(List<AssetSummary> assets) {
    return assets.isEmpty ? null : assets.first.id;
  }

  String? get effectiveSelectedAssetId {
    if (assets.isEmpty) {
      return null;
    }
    final selectedAssetId = this.selectedAssetId;
    if (selectedAssetId != null &&
        assets.any((asset) => asset.id == selectedAssetId)) {
      return selectedAssetId;
    }
    return firstAvailableAssetId(assets);
  }

  TransferDraft? buildDraft() {
    final wallet = this.wallet;
    final asset = selectedAsset;
    if (wallet == null || asset == null) {
      return null;
    }
    return TransferDraft(
      walletId: wallet.id,
      chain: asset.chain,
      assetId: asset.id,
      toAddress: recipientController.text.trim(),
      amount: amountController.text.trim(),
      blockIfEnergyInsufficient: blockIfEnergyInsufficient,
    );
  }

  AssetSummary? get selectedAsset {
    final selectedAssetId = effectiveSelectedAssetId;
    if (selectedAssetId == null) {
      return null;
    }
    for (final asset in assets) {
      if (asset.id == selectedAssetId) {
        return asset;
      }
    }
    return null;
  }
}
