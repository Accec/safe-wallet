import '../../../models.dart';
import '../../../wallet_api.dart';
import 'fields.dart';

MultisigAccountSummary multisigAccountFromNativeJson(Object? value) {
  if (value is! Map<String, dynamic>) {
    throw const WalletApiException('Invalid native response');
  }
  final permissionId = value['permission_id'];
  if (permissionId != null && permissionId is! int) {
    throw const WalletApiException('Invalid native response');
  }
  return MultisigAccountSummary(
    id: nativeStringField(value, 'id'),
    label: nativeStringField(value, 'label'),
    chain: nativeStringField(value, 'chain'),
    kind: nativeStringField(value, 'kind'),
    address: nativeStringField(value, 'address'),
    threshold: nativeIntField(value, 'threshold'),
    permissionId: permissionId,
  );
}

MultisigProposalSummary multisigProposalFromNativeJson(Object? value) {
  if (value is! Map<String, dynamic>) {
    throw const WalletApiException('Invalid native response');
  }
  return MultisigProposalSummary(
    id: nativeStringField(value, 'id'),
    multisigAccountId: nativeStringField(value, 'multisig_account_id'),
    chain: nativeStringField(value, 'chain'),
    toAddress: nativeStringField(value, 'to_address'),
    assetSymbol: nativeStringField(value, 'asset_symbol'),
    amount: nativeStringField(value, 'amount'),
    payloadJson: nativeStringField(value, 'payload_json'),
    status: nativeStringField(value, 'status'),
    threshold: nativeIntField(value, 'threshold'),
    signatureWeight: nativeIntField(value, 'signature_weight'),
  );
}
