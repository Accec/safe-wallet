import '../../../../models.dart';
import '../demo_multisig_client.dart';

mixin DemoMultisigApi {
  DemoMultisigClient get multisigClient;

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
