import 'package:flutter/material.dart';

import '../models.dart';

class WalletSidebarWallets extends StatelessWidget {
  const WalletSidebarWallets({
    super.key,
    required this.wallets,
    required this.selectedWallet,
    required this.onSelectedWalletChanged,
  });

  final List<WalletSummary> wallets;
  final WalletSummary? selectedWallet;
  final ValueChanged<WalletSummary> onSelectedWalletChanged;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Wallets',
          style: TextStyle(
            color: Color(0xff6b7280),
            fontSize: 12,
            fontWeight: FontWeight.w700,
          ),
        ),
        const SizedBox(height: 8),
        Expanded(
          child: wallets.isEmpty
              ? const Align(
                  alignment: Alignment.topLeft,
                  child: Text(
                    'No wallets yet',
                    style: TextStyle(color: Color(0xff6b7280)),
                  ),
                )
              : ListView.separated(
                  itemCount: wallets.length,
                  separatorBuilder: (_, _) => const SizedBox(height: 6),
                  itemBuilder: (context, index) {
                    final wallet = wallets[index];
                    return _SidebarWalletItem(
                      wallet: wallet,
                      selected: wallet.id == selectedWallet?.id,
                      onTap: () => onSelectedWalletChanged(wallet),
                    );
                  },
                ),
        ),
      ],
    );
  }
}

class _SidebarWalletItem extends StatelessWidget {
  const _SidebarWalletItem({
    required this.wallet,
    required this.selected,
    required this.onTap,
  });

  final WalletSummary wallet;
  final bool selected;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return Tooltip(
      message: 'Switch to ${wallet.label} wallet',
      child: InkWell(
        borderRadius: BorderRadius.circular(8),
        onTap: onTap,
        child: Container(
          constraints: const BoxConstraints(minHeight: 48),
          padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
          decoration: BoxDecoration(
            color: selected ? const Color(0xfff0fdfa) : Colors.transparent,
            borderRadius: BorderRadius.circular(8),
            border: Border.all(
              color: selected ? const Color(0xff99f6e4) : Colors.transparent,
            ),
          ),
          child: Row(
            children: [
              CircleAvatar(
                radius: 16,
                backgroundColor: selected
                    ? const Color(0xff0f766e)
                    : const Color(0xffe5e7eb),
                foregroundColor: selected
                    ? Colors.white
                    : const Color(0xff374151),
                child: Text(wallet.label.characters.first.toUpperCase()),
              ),
              const SizedBox(width: 10),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Text(
                      wallet.label,
                      overflow: TextOverflow.ellipsis,
                      style: const TextStyle(
                        fontWeight: FontWeight.w700,
                        color: Color(0xff111827),
                      ),
                    ),
                    Text(
                      wallet.id,
                      overflow: TextOverflow.ellipsis,
                      style: const TextStyle(
                        fontSize: 11,
                        color: Color(0xff6b7280),
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
