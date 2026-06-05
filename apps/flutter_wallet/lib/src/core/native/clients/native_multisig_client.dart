import '../../../models.dart';
import '../../../wallet_api.dart';
import '../native_response_parsers.dart';
import '../native_wallet_gateway.dart';

class NativeMultisigClient {
  const NativeMultisigClient(this._gateway);

  final NativeWalletGateway _gateway;

  Future<MultisigAccountSummary> importAccount(
    ImportMultisigAccountDraft draft,
  ) async {
    final body = await _gateway.invokeDb(
      domain: 'multisig',
      action: 'import_account',
      payload: <String, Object?>{
        'label': draft.label,
        'chain': draft.chain,
        'kind': draft.kind,
        'address': draft.address,
        'threshold': draft.threshold,
        'permission_id': draft.permissionId,
        'owners': draft.owners.map((owner) => owner.toJson()).toList(),
      },
    );
    return multisigAccountFromNativeJson(body);
  }

  Future<List<MultisigAccountSummary>> listAccounts() async {
    final body = await _gateway.invokeDb(
      domain: 'multisig',
      action: 'list_accounts',
    );
    if (body is! List<dynamic>) {
      throw const WalletApiException('Invalid native response');
    }
    return body.map(multisigAccountFromNativeJson).toList();
  }

  Future<MultisigProposalSummary> createProposal(
    CreateMultisigProposalDraft draft,
  ) async {
    final body = await _gateway.invokeDb(
      domain: 'multisig',
      action: 'create_proposal',
      payload: <String, Object?>{
        'multisig_account_id': draft.multisigAccountId,
        'to_address': draft.toAddress,
        'asset_symbol': draft.assetSymbol,
        'amount': draft.amount,
      },
    );
    return multisigProposalFromNativeJson(body);
  }

  Future<List<MultisigProposalSummary>> listProposals({
    String? multisigAccountId,
  }) async {
    final body = await _gateway.invokeDb(
      domain: 'multisig',
      action: 'list_proposals',
      payload: <String, Object?>{'multisig_account_id': multisigAccountId},
    );
    if (body is! List<dynamic>) {
      throw const WalletApiException('Invalid native response');
    }
    return body.map(multisigProposalFromNativeJson).toList();
  }

  Future<MultisigProposalSummary> addSignature({
    required String proposalId,
    required String ownerAddress,
    required String signature,
  }) async {
    final body = await _gateway.invokeDb(
      domain: 'multisig',
      action: 'add_signature',
      payload: <String, Object?>{
        'proposal_id': proposalId,
        'owner_address': ownerAddress,
        'signature': signature,
      },
    );
    return multisigProposalFromNativeJson(body);
  }
}
