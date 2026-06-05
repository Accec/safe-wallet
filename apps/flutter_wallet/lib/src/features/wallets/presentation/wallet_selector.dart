import 'package:flutter/material.dart';

import '../../../models.dart';

class WalletSelector extends StatelessWidget {
  const WalletSelector({
    super.key,
    required this.wallets,
    required this.selectedWallet,
    required this.onChanged,
  });

  final List<WalletSummary> wallets;
  final WalletSummary? selectedWallet;
  final ValueChanged<WalletSummary> onChanged;

  @override
  Widget build(BuildContext context) {
    final selected = selectedWallet;
    if (wallets.isEmpty || selected == null) {
      return const SizedBox.shrink();
    }
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 180),
      child: Tooltip(
        message: 'Select wallet',
        child: DropdownButtonHideUnderline(
          child: DropdownButton<String>(
            value: selected.id,
            icon: const Icon(Icons.expand_more),
            isExpanded: true,
            items: [
              for (final wallet in wallets)
                DropdownMenuItem(
                  value: wallet.id,
                  child: Text(wallet.label, overflow: TextOverflow.ellipsis),
                ),
            ],
            onChanged: (walletId) {
              if (walletId == null) {
                return;
              }
              final wallet = wallets.firstWhere(
                (wallet) => wallet.id == walletId,
                orElse: () => selected,
              );
              onChanged(wallet);
            },
          ),
        ),
      ),
    );
  }
}
