import '../../../models.dart';
import '../../../wallet_api.dart';

class TransfersClient {
  const TransfersClient(this._api);

  final WalletApi _api;

  Future<ParsedPayment> parsePaymentUri(String payload) =>
      _api.parsePaymentUri(payload);
  Future<TransferPreview> preview(TransferDraft draft) =>
      _api.previewTransfer(draft);
  Future<TransferResult> send(TransferDraft draft, String password) =>
      _api.sendTransfer(draft, password);
}
