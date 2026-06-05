import 'package:flutter/material.dart';

import '../../../models.dart';
import 'wallet_address_clipboard.dart';
import 'wallet_floating_actions.dart';
import 'wallets_list.dart';

class WalletsScreen extends StatelessWidget {
  const WalletsScreen({
    super.key,
    required this.wallets,
    required this.selectedWallet,
    required this.accounts,
    required this.onSelectedWalletChanged,
    required this.selectedChain,
    required this.onSelectedChainChanged,
    required this.onRefresh,
    required this.onGenerateMnemonic,
    required this.onCreateWallet,
    required this.onImportWallet,
    required this.onImportPrivateKey,
    required this.onImportKeystore,
    required this.onExportKeystore,
    required this.onDeleteWallet,
  });

  final List<WalletSummary> wallets;
  final WalletSummary? selectedWallet;
  final List<AccountSummary> accounts;
  final ValueChanged<WalletSummary> onSelectedWalletChanged;
  final String? selectedChain;
  final ValueChanged<String> onSelectedChainChanged;
  final VoidCallback onRefresh;
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
  final Future<KeystoreExport> Function({
    required String walletId,
    required String password,
  })
  onExportKeystore;
  final Future<void> Function({
    required String walletId,
    required String password,
  })
  onDeleteWallet;

  @override
  Widget build(BuildContext context) {
    final visibleAccounts = selectedChain == null
        ? const <AccountSummary>[]
        : accounts.where((account) => account.chain == selectedChain).toList();

    return LayoutBuilder(
      builder: (context, constraints) {
        final compact = constraints.maxWidth < 520;
        return Scaffold(
          appBar: AppBar(
            title: const Text('Wallets'),
            actions: [
              IconButton(
                tooltip: 'Refresh',
                onPressed: onRefresh,
                icon: const Icon(Icons.refresh),
              ),
            ],
          ),
          body: WalletsList(
            wallets: wallets,
            selectedWallet: selectedWallet,
            visibleAccounts: visibleAccounts,
            onSelectedWalletChanged: onSelectedWalletChanged,
            onExportKeystore: onExportKeystore,
            onDeleteWallet: onDeleteWallet,
            onCopyAddress: copyWalletAddress,
          ),
          floatingActionButton: WalletFloatingActions(
            compact: compact,
            onGenerateMnemonic: onGenerateMnemonic,
            onCreateWallet: onCreateWallet,
            onImportWallet: onImportWallet,
            onImportPrivateKey: onImportPrivateKey,
            onImportKeystore: onImportKeystore,
          ),
        );
      },
    );
  }
}
