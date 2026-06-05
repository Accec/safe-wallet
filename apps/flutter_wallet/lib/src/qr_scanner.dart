import 'package:flutter/services.dart';

class QrScanner {
  const QrScanner();

  static const MethodChannel _channel = MethodChannel(
    'app.localwallet/qr_scanner',
  );

  Future<String?> scanQr() {
    return _channel.invokeMethod<String>('scanQr');
  }

  Future<String?> pickQrImagePayload() {
    return _channel.invokeMethod<String>('pickQrImagePayload');
  }
}
