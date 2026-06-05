import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/api/wallet_clients.dart';
import '../../models.dart';

final multisigControllerProvider =
    NotifierProvider<MultisigController, MultisigState>(MultisigController.new);

class MultisigState {
  const MultisigState({
    this.loading = true,
    this.busy = false,
    this.accounts = const [],
    this.proposals = const [],
  });

  final bool loading;
  final bool busy;
  final List<MultisigAccountSummary> accounts;
  final List<MultisigProposalSummary> proposals;

  MultisigState copyWith({
    bool? loading,
    bool? busy,
    List<MultisigAccountSummary>? accounts,
    List<MultisigProposalSummary>? proposals,
  }) {
    return MultisigState(
      loading: loading ?? this.loading,
      busy: busy ?? this.busy,
      accounts: accounts ?? this.accounts,
      proposals: proposals ?? this.proposals,
    );
  }
}

class MultisigController extends Notifier<MultisigState> {
  @override
  MultisigState build() => const MultisigState();

  WalletClients get _clients => ref.read(walletClientsProvider);

  Future<void> load() async {
    state = state.copyWith(loading: true);
    try {
      await _loadData(loading: false);
    } catch (_) {
      state = state.copyWith(loading: false);
      rethrow;
    }
  }

  Future<void> importAccount(ImportMultisigAccountDraft draft) {
    return _runBusy(() => _clients.multisig.importAccount(draft));
  }

  Future<void> createProposal(CreateMultisigProposalDraft draft) {
    return _runBusy(() => _clients.multisig.createProposal(draft));
  }

  Future<void> addSignature({
    required String proposalId,
    required String ownerAddress,
    required String signature,
  }) {
    return _runBusy(
      () => _clients.multisig.addSignature(
        proposalId: proposalId,
        ownerAddress: ownerAddress,
        signature: signature,
      ),
    );
  }

  Future<void> _runBusy(Future<Object?> Function() action) async {
    state = state.copyWith(busy: true);
    try {
      await action();
      await _loadData();
    } catch (_) {
      state = state.copyWith(busy: false);
      rethrow;
    }
  }

  Future<void> _loadData({bool? loading}) async {
    final accounts = await _clients.multisig.listAccounts();
    final proposals = await _clients.multisig.listProposals();
    state = state.copyWith(
      loading: loading,
      busy: false,
      accounts: accounts,
      proposals: proposals,
    );
  }
}
