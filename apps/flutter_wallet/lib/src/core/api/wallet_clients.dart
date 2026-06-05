import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../state/wallet_providers.dart';
import 'clients/activity.dart';
import 'clients/assets.dart';
import 'clients/auth.dart';
import 'clients/multisig.dart';
import 'clients/network.dart';
import 'clients/transfers.dart';
import 'clients/wallets.dart';
import 'wallet_api_contract.dart';

final walletClientsProvider = Provider<WalletClients>(
  (ref) => WalletClients(ref.watch(walletApiProvider)),
);

class WalletClients {
  WalletClients(WalletApi api)
    : auth = AuthClient(api),
      wallets = WalletsClient(api),
      assets = AssetsClient(api),
      transfers = TransfersClient(api),
      activity = ActivityClient(api),
      multisig = MultisigClient(api),
      network = NetworkClient(api);

  final AuthClient auth;
  final WalletsClient wallets;
  final AssetsClient assets;
  final TransfersClient transfers;
  final ActivityClient activity;
  final MultisigClient multisig;
  final NetworkClient network;
}
