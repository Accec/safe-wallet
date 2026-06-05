import 'package:flutter/material.dart';

import '../../../core/ui/screen_motion.dart';
import '../../../models.dart';
import 'wallet_list_items.dart';
import 'wallets_empty_state.dart';

class WalletsList extends StatelessWidget {
  const WalletsList({
    super.key,
    required this.wallets,
    required this.selectedWallet,
    required this.visibleAccounts,
    required this.onSelectedWalletChanged,
    required this.onExportKeystore,
    required this.onDeleteWallet,
    required this.onCopyAddress,
  });

  final List<WalletSummary> wallets;
  final WalletSummary? selectedWallet;
  final List<AccountSummary> visibleAccounts;
  final ValueChanged<WalletSummary> onSelectedWalletChanged;
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
  final void Function(BuildContext context, String address) onCopyAddress;

  @override
  Widget build(BuildContext context) {
    return ListView(padding: const EdgeInsets.all(16), children: _rows());
  }

  List<Widget> _rows() {
    if (wallets.isEmpty) {
      return const [StaggeredListItem(index: 0, child: WalletsEmptyState())];
    }

    var rowIndex = 0;
    final rows = <Widget>[];
    for (final wallet in wallets) {
      rows.add(
        StaggeredListItem(
          index: rowIndex++,
          child: WalletListItem(
            wallet: wallet,
            selected: wallet.id == selectedWallet?.id,
            onSelected: onSelectedWalletChanged,
            onExportKeystore: onExportKeystore,
            onDeleteWallet: onDeleteWallet,
          ),
        ),
      );
      if (wallet.id == selectedWallet?.id) {
        for (final account in visibleAccounts) {
          rows.add(
            StaggeredListItem(
              index: rowIndex++,
              child: WalletAccountItem(
                account: account,
                onCopyAddress: onCopyAddress,
              ),
            ),
          );
        }
      }
    }
    return rows;
  }
}
