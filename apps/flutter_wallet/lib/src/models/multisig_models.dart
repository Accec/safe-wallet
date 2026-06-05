class MultisigOwnerDraft {
  const MultisigOwnerDraft({required this.address, required this.weight});

  final String address;
  final int weight;

  Map<String, Object?> toJson() {
    return {'address': address, 'weight': weight};
  }
}

class ImportMultisigAccountDraft {
  const ImportMultisigAccountDraft({
    required this.label,
    required this.chain,
    required this.kind,
    required this.address,
    required this.threshold,
    required this.owners,
    this.permissionId,
  });

  final String label;
  final String chain;
  final String kind;
  final String address;
  final int threshold;
  final int? permissionId;
  final List<MultisigOwnerDraft> owners;
}

class MultisigAccountSummary {
  const MultisigAccountSummary({
    required this.id,
    required this.label,
    required this.chain,
    required this.kind,
    required this.address,
    required this.threshold,
    this.permissionId,
  });

  final String id;
  final String label;
  final String chain;
  final String kind;
  final String address;
  final int threshold;
  final int? permissionId;
}

class CreateMultisigProposalDraft {
  const CreateMultisigProposalDraft({
    required this.multisigAccountId,
    required this.toAddress,
    required this.assetSymbol,
    required this.amount,
  });

  final String multisigAccountId;
  final String toAddress;
  final String assetSymbol;
  final String amount;
}

class MultisigProposalSummary {
  const MultisigProposalSummary({
    required this.id,
    required this.multisigAccountId,
    required this.chain,
    required this.toAddress,
    required this.assetSymbol,
    required this.amount,
    required this.payloadJson,
    required this.status,
    required this.threshold,
    required this.signatureWeight,
  });

  final String id;
  final String multisigAccountId;
  final String chain;
  final String toAddress;
  final String assetSymbol;
  final String amount;
  final String payloadJson;
  final String status;
  final int threshold;
  final int signatureWeight;
}
