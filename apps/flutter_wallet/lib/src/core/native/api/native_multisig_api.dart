import '../../../models.dart';
import '../clients/native_multisig_client.dart';

mixin NativeMultisigApi {
  NativeMultisigClient get multisigClient;

  Future<MultisigAccountSummary> importMultisigAccount(
    ImportMultisigAccountDraft draft,
  ) {
    return multisigClient.importAccount(draft);
  }

  Future<List<MultisigAccountSummary>> listMultisigAccounts() {
    return multisigClient.listAccounts();
  }

  Future<MultisigProposalSummary> createMultisigProposal(
    CreateMultisigProposalDraft draft,
  ) {
    return multisigClient.createProposal(draft);
  }

  Future<List<MultisigProposalSummary>> listMultisigProposals({
    String? multisigAccountId,
  }) {
    return multisigClient.listProposals(multisigAccountId: multisigAccountId);
  }

  Future<MultisigProposalSummary> addMultisigSignature({
    required String proposalId,
    required String ownerAddress,
    required String signature,
  }) {
    return multisigClient.addSignature(
      proposalId: proposalId,
      ownerAddress: ownerAddress,
      signature: signature,
    );
  }
}
