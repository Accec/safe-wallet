import 'package:flutter/material.dart';

import '../core/ui/chain_selector.dart';
import '../models.dart';
import 'wallet_sidebar_navigation.dart';
import 'wallet_sidebar_wallets.dart';

class WalletSidebar extends StatelessWidget {
  const WalletSidebar({
    super.key,
    required this.index,
    required this.wallets,
    required this.selectedWallet,
    required this.selectedChain,
    required this.onSelectIndex,
    required this.onSelectedWalletChanged,
    required this.onSelectedChainChanged,
  });

  final int index;
  final List<WalletSummary> wallets;
  final WalletSummary? selectedWallet;
  final String? selectedChain;
  final ValueChanged<int> onSelectIndex;
  final ValueChanged<WalletSummary> onSelectedWalletChanged;
  final ValueChanged<String> onSelectedChainChanged;

  @override
  Widget build(BuildContext context) {
    const borderColor = Color(0xffe5e7eb);
    return Container(
      width: 280,
      decoration: const BoxDecoration(
        color: Colors.white,
        border: Border(right: BorderSide(color: borderColor)),
      ),
      child: SafeArea(
        child: Padding(
          padding: const EdgeInsets.fromLTRB(16, 16, 16, 14),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Row(
                children: [
                  ClipRRect(
                    borderRadius: BorderRadius.circular(8),
                    child: Image.asset(
                      'assets/branding/safe_wallet_icon.png',
                      width: 36,
                      height: 36,
                    ),
                  ),
                  const SizedBox(width: 10),
                  const Expanded(
                    child: Text(
                      'Safe Wallet',
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        fontSize: 18,
                        fontWeight: FontWeight.w800,
                        color: Color(0xff111827),
                      ),
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 18),
              ChainSelector(
                chains: safeWalletSupportedChains,
                selectedChain: selectedChain,
                onChanged: onSelectedChainChanged,
              ),
              const SizedBox(height: 22),
              WalletSidebarNavigation(
                selectedIndex: index,
                onSelectIndex: onSelectIndex,
              ),
              const SizedBox(height: 18),
              const Divider(height: 1),
              const SizedBox(height: 14),
              Expanded(
                child: WalletSidebarWallets(
                  wallets: wallets,
                  selectedWallet: selectedWallet,
                  onSelectedWalletChanged: onSelectedWalletChanged,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
