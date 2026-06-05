import '../../../models.dart';
import '../../../wallet_api.dart';
import 'fields.dart';

ActivitySummary activityFromNativeJson(Object? value) {
  if (value is! Map<String, dynamic>) {
    throw const WalletApiException('Invalid native response');
  }
  return ActivitySummary(
    chain: nativeStringField(value, 'chain'),
    txHash: nativeStringField(value, 'tx_hash'),
    kind: nativeStringField(value, 'kind'),
    status: nativeStringField(value, 'status'),
    summary: nativeStringField(value, 'summary'),
  );
}
