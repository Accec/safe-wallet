import '../../models.dart';

class WalletApiException implements Exception {
  const WalletApiException(this.message);

  final String message;

  @override
  String toString() => message;
}

abstract class WalletApi {
  Future<AppStatus> appStatus();
  Future<void> setMasterPassword(String password);
  Future<void> unlockApp(String password);
  Future<void> setDuressPassword({
    required String masterPassword,
    required String duressPassword,
  });
  Future<void> updateBiometricUnlock({
    required String password,
    required bool enabled,
  });
  Future<String> generateMnemonic();
  Future<WalletSummary> createWallet({
    required String label,
    required String mnemonic,
    required String password,
  });
  Future<WalletSummary> importWallet({
    required String label,
    required String mnemonic,
    required String password,
  });
  Future<WalletSummary> importPrivateKey({
    required String label,
    required String privateKey,
    required String password,
  });
  Future<WalletSummary> importKeystore({
    required String label,
    required String keystoreJson,
    required String keystorePassword,
    required String password,
  });
  Future<KeystoreExport> exportKeystore({
    required String walletId,
    required String password,
  });
  Future<void> deleteWallet({
    required String walletId,
    required String password,
  });
  Future<List<WalletSummary>> listWallets();
  Future<List<AccountSummary>> listAccounts(String walletId);
  Future<List<AssetSummary>> listAssets(String walletId);
  Future<void> refreshAssets(String walletId, {String? chain});
  Future<void> discoverAssets(String walletId, {String? chain});
  Future<AssetSummary> addCustomToken({
    required String chain,
    required String contractAddress,
    required String tokenName,
  });
  Future<void> removeCustomToken(String assetId);
  Future<ParsedPayment> parsePaymentUri(String payload);
  Future<TransferPreview> previewTransfer(TransferDraft draft);
  Future<TransferResult> sendTransfer(TransferDraft draft, String password);
  Future<List<ActivitySummary>> listActivity(String walletId);
  Future<void> syncActivity(String walletId, {String? chain});
  Future<MultisigAccountSummary> importMultisigAccount(
    ImportMultisigAccountDraft draft,
  );
  Future<List<MultisigAccountSummary>> listMultisigAccounts();
  Future<MultisigProposalSummary> createMultisigProposal(
    CreateMultisigProposalDraft draft,
  );
  Future<List<MultisigProposalSummary>> listMultisigProposals({
    String? multisigAccountId,
  });
  Future<MultisigProposalSummary> addMultisigSignature({
    required String proposalId,
    required String ownerAddress,
    required String signature,
  });
  Future<NetworkPrivacySettings> networkPrivacySettings();
  Future<void> saveNetworkPrivacySettings(NetworkPrivacySettingsDraft draft);
  Future<void> testProxyConnection(NetworkPrivacySettingsDraft draft);
  Future<List<NetworkSettings>> listNetworkSettings();
  Future<void> updateChainRpc({required String chain, required String rpcUrl});
  Future<void> saveNetworkSettings(NetworkSettingsDraft draft);
  Future<void> updateIndexerSettings({
    required String chain,
    required String endpoint,
    String? apiKey,
  });
}
