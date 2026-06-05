import 'dart:convert';

import '../../../models.dart';
import '../native_response_parsers.dart';
import '../native_wallet_gateway.dart';

class NativeTransfersClient {
  const NativeTransfersClient(this._gateway);

  final NativeWalletGateway _gateway;

  Future<ParsedPayment> parsePaymentUri(String payload) async {
    final body = await _gateway.invoke(
      domain: 'transfers',
      action: 'parse_payment_uri',
      payload: <String, Object?>{'payload': payload},
    );
    return parsedPaymentFromNativeResponse(body);
  }

  Future<TransferPreview> preview(TransferDraft draft) async {
    final body = await _gateway.invokeDb(
      domain: 'transfers',
      action: 'preview',
      payload: <String, Object?>{'request_json': jsonEncode(draft.toJson())},
    );
    return transferPreviewFromNativeJson(body);
  }

  Future<TransferResult> send(TransferDraft draft, String password) async {
    final body = await _gateway.invokeDb(
      domain: 'transfers',
      action: 'send',
      payload: <String, Object?>{
        'request_json': jsonEncode(draft.toJson()),
        'password': password,
      },
    );
    return transferResultFromNativeJson(body);
  }
}
