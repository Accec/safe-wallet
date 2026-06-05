import '../../../models.dart';
import '../wallet_api_contract.dart';
import 'demo_wallet_store.dart';

class DemoMultisigClient {
  const DemoMultisigClient(this._store);

  final DemoWalletStore _store;

  Future<MultisigAccountSummary> importAccount(
    ImportMultisigAccountDraft draft,
  ) async {
    final account = MultisigAccountSummary(
      id: 'multisig-${_store.multisigAccounts.length + 1}',
      label: draft.label,
      chain: draft.chain,
      kind: draft.kind,
      address: draft.address,
      threshold: draft.threshold,
      permissionId: draft.permissionId,
    );
    _store.multisigAccounts.add(account);
    return account;
  }

  Future<List<MultisigAccountSummary>> listAccounts() async {
    return _store.multisigAccounts;
  }

  Future<MultisigProposalSummary> createProposal(
    CreateMultisigProposalDraft draft,
  ) async {
    final account = _store.multisigAccounts.firstWhere(
      (account) => account.id == draft.multisigAccountId,
    );
    final proposal = MultisigProposalSummary(
      id: 'proposal-${_store.multisigProposals.length + 1}',
      multisigAccountId: account.id,
      chain: account.chain,
      toAddress: draft.toAddress,
      assetSymbol: draft.assetSymbol,
      amount: draft.amount,
      payloadJson: '{"kind":"${account.kind}"}',
      status: 'pending_signatures',
      threshold: account.threshold,
      signatureWeight: 0,
    );
    _store.multisigProposals.add(proposal);
    return proposal;
  }

  Future<List<MultisigProposalSummary>> listProposals({
    String? multisigAccountId,
  }) async {
    return multisigAccountId == null
        ? _store.multisigProposals
        : _store.multisigProposals
              .where(
                (proposal) => proposal.multisigAccountId == multisigAccountId,
              )
              .toList();
  }

  Future<MultisigProposalSummary> addSignature({
    required String proposalId,
    required String ownerAddress,
    required String signature,
  }) async {
    final index = _store.multisigProposals.indexWhere(
      (proposal) => proposal.id == proposalId,
    );
    if (index == -1) {
      throw const WalletApiException('Multisig account not found');
    }
    final proposal = _store.multisigProposals[index];
    final nextWeight = proposal.signatureWeight + 1;
    final signed = MultisigProposalSummary(
      id: proposal.id,
      multisigAccountId: proposal.multisigAccountId,
      chain: proposal.chain,
      toAddress: proposal.toAddress,
      assetSymbol: proposal.assetSymbol,
      amount: proposal.amount,
      payloadJson: proposal.payloadJson,
      status: nextWeight >= proposal.threshold ? 'ready' : 'pending_signatures',
      threshold: proposal.threshold,
      signatureWeight: nextWeight,
    );
    _store.multisigProposals[index] = signed;
    return signed;
  }
}
