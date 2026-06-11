import '../../../models.dart';
import '../../../wallet_api.dart';
import 'fields.dart';

TransferPreview transferPreviewFromNativeJson(Object? value) {
  if (value is! Map<String, dynamic>) {
    throw const WalletApiException('Invalid native response');
  }
  return TransferPreview(
    chain: nativeStringField(value, 'chain'),
    fromAddress: nativeStringField(value, 'from_address'),
    toAddress: nativeStringField(value, 'to_address'),
    assetSymbol: nativeStringField(value, 'asset_symbol'),
    amount: nativeStringField(value, 'amount'),
    feeEstimate: nativeStringField(value, 'fee_estimate'),
    rpcUrl: nativeStringField(value, 'rpc_url'),
    resourceStatus: transferResourceStatusFromNativeJson(
      value['resource_status'],
    ),
  );
}

TransferResourceStatus? transferResourceStatusFromNativeJson(Object? value) {
  if (value == null) {
    return null;
  }
  if (value is! Map<String, dynamic>) {
    throw const WalletApiException('Invalid native response');
  }
  return TransferResourceStatus(
    energyAvailable: nativeIntField(value, 'energy_available'),
    energyRequired: nativeIntField(value, 'energy_required'),
    bandwidthAvailable: nativeIntField(value, 'bandwidth_available'),
    bandwidthRequired: nativeIntField(value, 'bandwidth_required'),
    trxBalanceSun: nativeIntField(value, 'trx_balance_sun'),
    trxFeeReserveRequiredSun: nativeIntField(
      value,
      'trx_fee_reserve_required_sun',
    ),
    canSendWithoutBurningTrx: nativeBoolField(
      value,
      'can_send_without_burning_trx',
    ),
  );
}

TransferResult transferResultFromNativeJson(Object? value) {
  if (value is! Map<String, dynamic>) {
    throw const WalletApiException('Invalid native response');
  }
  return TransferResult(
    chain: nativeStringField(value, 'chain'),
    txHash: nativeStringField(value, 'tx_hash'),
    status: nativeStringField(value, 'status'),
  );
}

ParsedPayment parsedPaymentFromNativeResponse(Object? body) {
  if (body is! Map<String, dynamic>) {
    throw const WalletApiException('Invalid native response');
  }
  final address = body['address'];
  if (address is! String) {
    throw const WalletApiException('Invalid native response');
  }
  return ParsedPayment(
    address: address,
    chain: optionalNativeString(body, 'chain'),
    amount: optionalNativeString(body, 'amount'),
    note: optionalNativeString(body, 'note'),
  );
}
