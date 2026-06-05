import 'package:flutter/material.dart';

import '../features/activity/presentation/activity_screen.dart';
import '../features/assets/presentation/assets_screen.dart';
import '../features/multisig/presentation/multisig_screen.dart';
import '../features/session/session_controller.dart';
import '../features/settings/settings_screen.dart';
import '../features/transfers/presentation/transfer_screen.dart';
import '../features/wallets/presentation/wallets_screen.dart';
import '../features/wallets/wallet_workspace_controller.dart';

List<Widget> buildWalletHomePages({
  required SessionState session,
  required WalletWorkspaceState workspace,
  required WalletWorkspaceController workspaceController,
  required ValueChanged<String> onPasswordRemembered,
}) {
  final selectedWallet = workspace.selectedWallet;
  final selectedChain = workspace.selectedChain;
  final wallets = workspace.wallets;
  final assets = workspace.assets;

  return [
    WalletsScreen(
      wallets: wallets,
      selectedWallet: selectedWallet,
      accounts: workspace.accounts,
      onSelectedWalletChanged: workspaceController.selectWallet,
      selectedChain: selectedChain,
      onSelectedChainChanged: workspaceController.selectChain,
      onRefresh: () {
        workspaceController.loadWallets();
      },
      onGenerateMnemonic: workspaceController.generateMnemonic,
      onCreateWallet: ({required label, required mnemonic}) =>
          workspaceController.createWallet(
            label: label,
            mnemonic: mnemonic,
            password: session.sessionPassword ?? '',
          ),
      onImportWallet: ({required label, required mnemonic}) =>
          workspaceController.importWallet(
            label: label,
            mnemonic: mnemonic,
            password: session.sessionPassword ?? '',
          ),
      onImportPrivateKey: ({required label, required privateKey}) =>
          workspaceController.importPrivateKey(
            label: label,
            privateKey: privateKey,
            password: session.sessionPassword ?? '',
          ),
      onImportKeystore:
          ({
            required label,
            required keystoreJson,
            required keystorePassword,
            required password,
          }) async {
            await workspaceController.importKeystore(
              label: label,
              keystoreJson: keystoreJson,
              keystorePassword: keystorePassword,
              password: password,
            );
            onPasswordRemembered(password);
          },
      onExportKeystore: workspaceController.exportKeystore,
      onDeleteWallet: workspaceController.deleteWallet,
    ),
    AssetsScreen(
      wallet: selectedWallet,
      wallets: wallets,
      assets: assets,
      onSelectedWalletChanged: workspaceController.selectWallet,
      selectedChain: selectedChain,
      onSelectedChainChanged: workspaceController.selectChain,
      onRefresh: workspaceController.refreshAssets,
      onDiscoverAssets: workspaceController.discoverAssets,
      onAddCustomToken: workspaceController.addCustomToken,
      onRemoveCustomToken: workspaceController.removeCustomToken,
    ),
    TransferScreen(
      wallet: selectedWallet,
      assets: selectedChain == null
          ? assets
          : assets.where((asset) => asset.chain == selectedChain).toList(),
      masterPassword: session.sessionPassword,
      onSent: workspaceController.loadWallets,
    ),
    const MultisigScreen(),
    ActivityScreen(
      activity: workspace.activity,
      wallets: wallets,
      selectedWallet: selectedWallet,
      onSync: workspaceController.syncActivity,
    ),
    const SettingsScreen(),
  ];
}
