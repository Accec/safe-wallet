import '../../../models.dart';
import '../../../wallet_api.dart';
import 'fields.dart';

WalletSummary walletFromNativeJson(Object? value) {
  if (value is! Map<String, dynamic>) {
    throw const WalletApiException('Invalid native response');
  }
  return WalletSummary(
    id: nativeStringField(value, 'id'),
    label: nativeStringField(value, 'label'),
  );
}

AccountSummary accountFromNativeJson(Object? value) {
  if (value is! Map<String, dynamic>) {
    throw const WalletApiException('Invalid native response');
  }
  return AccountSummary(
    walletId: nativeStringField(value, 'wallet_id'),
    chain: nativeStringField(value, 'chain'),
    address: nativeStringField(value, 'address'),
    derivationPath: nativeStringField(value, 'derivation_path'),
  );
}

KeystoreExport keystoreExportFromNativeJson(Object? value) {
  if (value is! Map<String, dynamic>) {
    throw const WalletApiException('Invalid native response');
  }
  return KeystoreExport(
    walletId: nativeStringField(value, 'wallet_id'),
    label: nativeStringField(value, 'label'),
    secretKind: nativeStringField(value, 'secret_kind'),
    ciphertextB64: nativeStringField(value, 'ciphertext_b64'),
    nonceB64: nativeStringField(value, 'nonce_b64'),
    saltB64: nativeStringField(value, 'salt_b64'),
    kdfName: nativeStringField(value, 'kdf_name'),
    kdfParamsJson: nativeStringField(value, 'kdf_params_json'),
    cipherName: nativeStringField(value, 'cipher_name'),
    version: nativeIntField(value, 'version'),
  );
}
