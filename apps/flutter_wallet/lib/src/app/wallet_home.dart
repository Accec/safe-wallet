import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../core/ui/screen_motion.dart';
import '../features/session/session_controller.dart';
import '../features/wallets/wallet_workspace_controller.dart';
import 'wallet_home_pages.dart';
import 'wallet_mobile_navigation.dart';
import 'wallet_sidebar.dart';

class WalletHome extends ConsumerStatefulWidget {
  const WalletHome({super.key});

  @override
  ConsumerState<WalletHome> createState() => _WalletHomeState();
}

class _WalletHomeState extends ConsumerState<WalletHome> {
  int _index = 0;

  void _selectIndex(int index) {
    setState(() => _index = index);
  }

  @override
  Widget build(BuildContext context) {
    final session = ref.watch(sessionControllerProvider);
    final workspace = ref.watch(walletWorkspaceControllerProvider);
    final workspaceController = ref.read(
      walletWorkspaceControllerProvider.notifier,
    );

    final wallets = workspace.wallets;
    final selectedWallet = workspace.selectedWallet;
    final selectedChain = workspace.selectedChain;
    final pages = buildWalletHomePages(
      session: session,
      workspace: workspace,
      workspaceController: workspaceController,
      onPasswordRemembered: ref
          .read(sessionControllerProvider.notifier)
          .rememberPassword,
    );

    return LayoutBuilder(
      builder: (context, constraints) {
        final compact = constraints.maxWidth < 720;
        if (compact) {
          return Scaffold(
            body: SafeArea(
              bottom: false,
              child: AnimatedPageStack(index: _index, children: pages),
            ),
            bottomNavigationBar: NavigationBar(
              selectedIndex: _index,
              onDestinationSelected: _selectIndex,
              labelBehavior: NavigationDestinationLabelBehavior.alwaysShow,
              destinations: walletHomeDestinations,
            ),
          );
        }
        return Scaffold(
          body: Row(
            children: [
              WalletSidebar(
                index: _index,
                wallets: wallets,
                selectedWallet: selectedWallet,
                selectedChain: selectedChain,
                onSelectIndex: _selectIndex,
                onSelectedWalletChanged: workspaceController.selectWallet,
                onSelectedChainChanged: workspaceController.selectChain,
              ),
              Expanded(
                child: AnimatedPageStack(index: _index, children: pages),
              ),
            ],
          ),
        );
      },
    );
  }
}
