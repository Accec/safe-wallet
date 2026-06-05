import '../../../models.dart';
import '../../../wallet_api.dart';

class MultisigClient {
  const MultisigClient(this._api);

  final WalletApi _api;

  Future<MultisigAccountSummary> importAccount(
    ImportMultisigAccountDraft draft,
  ) => _api.importMultisigAccount(draft);
  Future<List<MultisigAccountSummary>> listAccounts() =>
      _api.listMultisigAccounts();
  Future<MultisigProposalSummary> createProposal(
    CreateMultisigProposalDraft draft,
  ) => _api.createMultisigProposal(draft);
  Future<List<MultisigProposalSummary>> listProposals({
    String? multisigAccountId,
  }) => _api.listMultisigProposals(multisigAccountId: multisigAccountId);
  Future<MultisigProposalSummary> addSignature({
    required String proposalId,
    required String ownerAddress,
    required String signature,
  }) => _api.addMultisigSignature(
    proposalId: proposalId,
    ownerAddress: ownerAddress,
    signature: signature,
  );
}
