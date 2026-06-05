class ParsedPayment {
  const ParsedPayment({
    required this.address,
    this.chain,
    this.amount,
    this.note,
  });

  final String address;
  final String? chain;
  final String? amount;
  final String? note;
}

class TransferDraft {
  const TransferDraft({
    required this.walletId,
    required this.chain,
    required this.assetId,
    required this.toAddress,
    required this.amount,
  });

  final String walletId;
  final String chain;
  final String assetId;
  final String toAddress;
  final String amount;

  Map<String, Object?> toJson() {
    return {
      'wallet_id': walletId,
      'chain': chain,
      'asset_id': assetId,
      'to_address': toAddress,
      'amount': amount,
    };
  }
}

class TransferPreview {
  const TransferPreview({
    required this.chain,
    required this.fromAddress,
    required this.toAddress,
    required this.assetSymbol,
    required this.amount,
    required this.feeEstimate,
    required this.rpcUrl,
  });

  final String chain;
  final String fromAddress;
  final String toAddress;
  final String assetSymbol;
  final String amount;
  final String feeEstimate;
  final String rpcUrl;
}

class TransferResult {
  const TransferResult({
    required this.chain,
    required this.txHash,
    required this.status,
  });

  final String chain;
  final String txHash;
  final String status;
}
