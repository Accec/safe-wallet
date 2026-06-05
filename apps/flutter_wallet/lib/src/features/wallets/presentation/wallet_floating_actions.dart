import 'package:flutter/material.dart';

import 'wallet_dialogs.dart';

class WalletFloatingActions extends StatelessWidget {
  const WalletFloatingActions({
    super.key,
    required this.compact,
    required this.onGenerateMnemonic,
    required this.onCreateWallet,
    required this.onImportWallet,
    required this.onImportPrivateKey,
    required this.onImportKeystore,
  });

  final bool compact;
  final Future<String> Function() onGenerateMnemonic;
  final Future<void> Function({required String label, required String mnemonic})
  onCreateWallet;
  final Future<void> Function({required String label, required String mnemonic})
  onImportWallet;
  final Future<void> Function({
    required String label,
    required String privateKey,
  })
  onImportPrivateKey;
  final Future<void> Function({
    required String label,
    required String keystoreJson,
    required String keystorePassword,
    required String password,
  })
  onImportKeystore;

  @override
  Widget build(BuildContext context) {
    if (compact) {
      return _CreateWalletButton(
        onGenerateMnemonic: onGenerateMnemonic,
        onCreateWallet: onCreateWallet,
      );
    }

    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        FloatingActionButton.small(
          heroTag: 'import_wallet',
          tooltip: 'Import wallet',
          onPressed: () => showWalletDialog(
            context: context,
            title: 'Import wallet',
            actionLabel: 'Import wallet',
            onSubmit: onImportWallet,
          ),
          child: const Icon(Icons.file_upload_outlined),
        ),
        const SizedBox(width: 12),
        FloatingActionButton.small(
          heroTag: 'import_keystore',
          tooltip: 'Import keystore',
          onPressed: () => showKeystoreImportDialog(
            context: context,
            onSubmit: onImportKeystore,
          ),
          child: const Icon(Icons.inventory_2_outlined),
        ),
        const SizedBox(width: 12),
        FloatingActionButton.small(
          heroTag: 'import_private_key',
          tooltip: 'Import private key',
          onPressed: () => showPrivateKeyDialog(
            context: context,
            onSubmit: onImportPrivateKey,
          ),
          child: const Icon(Icons.key_outlined),
        ),
        const SizedBox(width: 12),
        _CreateWalletButton(
          onGenerateMnemonic: onGenerateMnemonic,
          onCreateWallet: onCreateWallet,
        ),
      ],
    );
  }
}

class _CreateWalletButton extends StatelessWidget {
  const _CreateWalletButton({
    required this.onGenerateMnemonic,
    required this.onCreateWallet,
  });

  final Future<String> Function() onGenerateMnemonic;
  final Future<void> Function({required String label, required String mnemonic})
  onCreateWallet;

  @override
  Widget build(BuildContext context) {
    return FloatingActionButton(
      heroTag: 'create_wallet',
      tooltip: 'Create wallet',
      onPressed: () => showWalletDialog(
        context: context,
        title: 'Create wallet',
        actionLabel: 'Save wallet',
        loadMnemonic: onGenerateMnemonic,
        onSubmit: onCreateWallet,
      ),
      child: const Icon(Icons.add),
    );
  }
}
