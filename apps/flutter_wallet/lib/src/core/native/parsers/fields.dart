import '../../../wallet_api.dart';

String stringFieldFromNativeJson(Map<String, dynamic> body, String key) {
  return nativeStringField(body, key);
}

String nativeStringField(Map<String, dynamic> body, String key) {
  final value = body[key];
  if (value is! String) {
    throw const WalletApiException('Invalid native response');
  }
  return value;
}

int nativeIntField(Map<String, dynamic> body, String key) {
  final value = body[key];
  if (value is! int) {
    throw const WalletApiException('Invalid native response');
  }
  return value;
}

bool nativeBoolField(Map<String, dynamic> body, String key) {
  final value = body[key];
  if (value is! bool) {
    throw const WalletApiException('Invalid native response');
  }
  return value;
}

String? optionalNativeString(Map<String, dynamic> body, String key) {
  final value = body[key];
  if (value == null) {
    return null;
  }
  if (value is String) {
    return value;
  }
  throw const WalletApiException('Invalid native response');
}
