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
    this.blockIfEnergyInsufficient = false,
  });

  final String walletId;
  final String chain;
  final String assetId;
  final String toAddress;
  final String amount;
  final bool blockIfEnergyInsufficient;

  TransferDraft copyWith({bool? blockIfEnergyInsufficient}) {
    return TransferDraft(
      walletId: walletId,
      chain: chain,
      assetId: assetId,
      toAddress: toAddress,
      amount: amount,
      blockIfEnergyInsufficient:
          blockIfEnergyInsufficient ?? this.blockIfEnergyInsufficient,
    );
  }

  Map<String, Object?> toJson() {
    return {
      'wallet_id': walletId,
      'chain': chain,
      'asset_id': assetId,
      'to_address': toAddress,
      'amount': amount,
      'block_if_energy_insufficient': blockIfEnergyInsufficient,
    };
  }
}

class TransferResourceStatus {
  const TransferResourceStatus({
    required this.energyAvailable,
    required this.energyRequired,
    required this.bandwidthAvailable,
    required this.bandwidthRequired,
    required this.trxBalanceSun,
    required this.trxFeeReserveRequiredSun,
    required this.canSendWithoutBurningTrx,
  });

  final int energyAvailable;
  final int energyRequired;
  final int bandwidthAvailable;
  final int bandwidthRequired;
  final int trxBalanceSun;
  final int trxFeeReserveRequiredSun;
  final bool canSendWithoutBurningTrx;

  bool get hasEnoughEnergy => energyAvailable >= energyRequired;

  bool get hasEnoughBandwidth => bandwidthAvailable >= bandwidthRequired;

  bool get hasEnoughTrxForResourceFees =>
      trxBalanceSun >= trxFeeReserveRequiredSun;
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
    this.resourceStatus,
  });

  final String chain;
  final String fromAddress;
  final String toAddress;
  final String assetSymbol;
  final String amount;
  final String feeEstimate;
  final String rpcUrl;
  final TransferResourceStatus? resourceStatus;
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
