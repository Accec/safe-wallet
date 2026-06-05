class WalletSummary {
  const WalletSummary({required this.id, required this.label});

  final String id;
  final String label;
}

class AccountSummary {
  const AccountSummary({
    required this.walletId,
    required this.chain,
    required this.address,
    required this.derivationPath,
  });

  final String walletId;
  final String chain;
  final String address;
  final String derivationPath;
}

class KeystoreExport {
  const KeystoreExport({
    required this.walletId,
    required this.label,
    required this.secretKind,
    required this.ciphertextB64,
    required this.nonceB64,
    required this.saltB64,
    required this.kdfName,
    required this.kdfParamsJson,
    required this.cipherName,
    required this.version,
  });

  final String walletId;
  final String label;
  final String secretKind;
  final String ciphertextB64;
  final String nonceB64;
  final String saltB64;
  final String kdfName;
  final String kdfParamsJson;
  final String cipherName;
  final int version;

  Map<String, Object?> toJson() {
    return {
      'wallet_id': walletId,
      'label': label,
      'secret_kind': secretKind,
      'ciphertext_b64': ciphertextB64,
      'nonce_b64': nonceB64,
      'salt_b64': saltB64,
      'kdf_name': kdfName,
      'kdf_params_json': kdfParamsJson,
      'cipher_name': cipherName,
      'version': version,
    };
  }
}
