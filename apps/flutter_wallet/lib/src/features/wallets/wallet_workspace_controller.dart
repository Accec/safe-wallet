import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/api/wallet_clients.dart';
import '../../core/chains/supported_chains.dart';
import '../../models.dart';

part 'wallet_workspace_asset_actions.dart';
part 'wallet_workspace_state.dart';
part 'wallet_workspace_wallet_mutations.dart';
part 'wallet_workspace_wallet_selection.dart';

final walletWorkspaceControllerProvider =
    NotifierProvider<WalletWorkspaceController, WalletWorkspaceState>(
      WalletWorkspaceController.new,
    );

class WalletWorkspaceController extends Notifier<WalletWorkspaceState>
    with
        WalletWorkspaceWalletSelection,
        WalletWorkspaceWalletMutations,
        WalletWorkspaceAssetActions {
  @override
  WalletWorkspaceState build() => const WalletWorkspaceState();

  @override
  WalletClients get walletClients => ref.read(walletClientsProvider);

  void reset() {
    state = const WalletWorkspaceState();
  }
}
