import '../../../models.dart';
import '../clients/native_transfers_client.dart';

mixin NativeTransfersApi {
  NativeTransfersClient get transfersClient;

  Future<ParsedPayment> parsePaymentUri(String payload) {
    return transfersClient.parsePaymentUri(payload);
  }

  Future<TransferPreview> previewTransfer(TransferDraft draft) {
    return transfersClient.preview(draft);
  }

  Future<TransferResult> sendTransfer(TransferDraft draft, String password) {
    return transfersClient.send(draft, password);
  }
}
