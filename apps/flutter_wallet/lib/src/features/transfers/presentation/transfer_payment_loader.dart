import 'package:flutter/widgets.dart';

import '../../../models.dart';

typedef PaymentUriParser = Future<ParsedPayment> Function(String payload);

class TransferPaymentLoader {
  const TransferPaymentLoader({
    required this.assets,
    required this.recipientController,
    required this.amountController,
    required this.parsePaymentUri,
  });

  final List<AssetSummary> assets;
  final TextEditingController recipientController;
  final TextEditingController amountController;
  final PaymentUriParser parsePaymentUri;

  Future<TransferPaymentLoadResult> load(String payload) async {
    final parsed = await parsePaymentUri(payload);
    recipientController.text = parsed.address;
    final amount = parsed.amount;
    if (amount != null) {
      amountController.text = amount;
    }
    return TransferPaymentLoadResult(
      selectedAssetId: _firstAssetIdForChain(parsed.chain),
    );
  }

  String? _firstAssetIdForChain(String? chain) {
    if (chain == null) {
      return null;
    }
    for (final asset in assets) {
      if (asset.chain == chain) {
        return asset.id;
      }
    }
    return null;
  }
}

class TransferPaymentLoadResult {
  const TransferPaymentLoadResult({required this.selectedAssetId});

  final String? selectedAssetId;
}
