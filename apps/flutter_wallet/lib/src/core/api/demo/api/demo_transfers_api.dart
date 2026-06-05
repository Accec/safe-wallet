import '../../../../models.dart';
import '../demo_transfers_client.dart';

mixin DemoTransfersApi {
  DemoTransfersClient get transfersClient;

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
