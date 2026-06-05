import 'package:flutter/material.dart';

import '../../../core/ui/chain_selector.dart';
import '../../../models.dart';
import 'wallet_dialogs.dart';

class WalletListItem extends StatelessWidget {
  const WalletListItem({
    super.key,
    required this.wallet,
    required this.selected,
    required this.onSelected,
    required this.onExportKeystore,
    required this.onDeleteWallet,
  });

  final WalletSummary wallet;
  final bool selected;
  final ValueChanged<WalletSummary> onSelected;
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
    return ListTile(
      leading: const Icon(Icons.account_balance_wallet_outlined),
      title: Text(wallet.label),
      subtitle: Text(wallet.id),
      selected: selected,
      onTap: () => onSelected(wallet),
      trailing: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          IconButton(
            tooltip: 'Export keystore',
            onPressed: () => showExportKeystoreDialog(
              context: context,
              wallet: wallet,
              onExportKeystore: onExportKeystore,
            ),
            icon: const Icon(Icons.ios_share_outlined),
          ),
          IconButton(
            tooltip: 'Delete wallet',
            onPressed: () => showDeleteWalletDialog(
              context: context,
              wallet: wallet,
              onDeleteWallet: onDeleteWallet,
            ),
            icon: const Icon(Icons.delete_outline),
          ),
        ],
      ),
    );
  }
}

class WalletAccountItem extends StatelessWidget {
  const WalletAccountItem({
    super.key,
    required this.account,
    required this.onCopyAddress,
  });

  final AccountSummary account;
  final void Function(BuildContext context, String address) onCopyAddress;

  @override
  Widget build(BuildContext context) {
    return ListTile(
      dense: true,
      leading: const SizedBox(width: 24),
      title: Text(chainLabel(account.chain)),
      subtitle: Text(account.address),
      trailing: IconButton(
        tooltip: 'Copy address',
        onPressed: () => onCopyAddress(context, account.address),
        icon: const Icon(Icons.copy_outlined),
      ),
    );
  }
}
