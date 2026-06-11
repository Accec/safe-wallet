import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/api/wallet_clients.dart';
import '../../models.dart';

final transferControllerProvider =
    NotifierProvider<TransferController, TransferState>(TransferController.new);

class TransferState {
  const TransferState({
    this.previewing = false,
    this.sending = false,
    this.blockIfEnergyInsufficient = true,
    this.preview,
    this.draft,
  });

  final bool previewing;
  final bool sending;
  final bool blockIfEnergyInsufficient;
  final TransferPreview? preview;
  final TransferDraft? draft;

  bool get energyBlocked {
    final resourceStatus = preview?.resourceStatus;
    return blockIfEnergyInsufficient &&
        resourceStatus != null &&
        !resourceStatus.hasEnoughEnergy;
  }

  bool get resourceFeeBlocked {
    final resourceStatus = preview?.resourceStatus;
    return resourceStatus != null &&
        !resourceStatus.hasEnoughTrxForResourceFees;
  }

  bool get canSend => preview != null && !energyBlocked && !resourceFeeBlocked;

  TransferState copyWith({
    bool? previewing,
    bool? sending,
    bool? blockIfEnergyInsufficient,
    TransferPreview? preview,
    TransferDraft? draft,
    bool clearPreview = false,
    bool clearDraft = false,
  }) {
    return TransferState(
      previewing: previewing ?? this.previewing,
      sending: sending ?? this.sending,
      blockIfEnergyInsufficient:
          blockIfEnergyInsufficient ?? this.blockIfEnergyInsufficient,
      preview: clearPreview ? null : preview ?? this.preview,
      draft: clearDraft ? null : draft ?? this.draft,
    );
  }
}

class TransferController extends Notifier<TransferState> {
  @override
  TransferState build() => const TransferState();

  WalletClients get _clients => ref.read(walletClientsProvider);

  Future<ParsedPayment> parsePaymentUri(String payload) {
    return _clients.transfers.parsePaymentUri(payload);
  }

  Future<void> preview(TransferDraft draft) async {
    state = state.copyWith(
      previewing: true,
      clearPreview: true,
      clearDraft: true,
    );
    try {
      final preview = await _clients.transfers.preview(draft);
      state = state.copyWith(previewing: false, preview: preview, draft: draft);
    } catch (_) {
      state = state.copyWith(previewing: false);
      rethrow;
    }
  }

  Future<TransferResult> send(String password) async {
    final draft = state.draft;
    if (draft == null) {
      throw const TransferStateException('Unlock again before sending.');
    }
    if (state.energyBlocked) {
      throw const TransferStateException('Insufficient energy.');
    }
    if (state.resourceFeeBlocked) {
      throw const TransferStateException('Insufficient TRX for network fees.');
    }
    state = state.copyWith(sending: true);
    try {
      final result = await _clients.transfers.send(draft, password);
      state = state.copyWith(sending: false);
      return result;
    } catch (_) {
      state = state.copyWith(sending: false);
      rethrow;
    }
  }

  void clearPreview() {
    state = state.copyWith(clearPreview: true, clearDraft: true);
  }

  void setBlockIfEnergyInsufficient(bool value) {
    state = state.copyWith(
      blockIfEnergyInsufficient: value,
      draft: state.draft?.copyWith(blockIfEnergyInsufficient: value),
    );
  }
}

class TransferStateException implements Exception {
  const TransferStateException(this.message);

  final String message;
}
