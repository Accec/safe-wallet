import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../models.dart';
import '../../../qr_scanner.dart';
import '../../settings/master_password_dialog.dart';
import '../transfer_controller.dart';
import 'transfer_draft_builder.dart';
import 'transfer_error_presenter.dart';
import 'transfer_feedback.dart';
import 'transfer_qr_actions.dart';
import 'transfer_screen_body.dart';
import 'transfer_submission_actions.dart';

part 'transfer_screen_qr_flow.dart';
part 'transfer_screen_submission_flow.dart';

class TransferScreen extends ConsumerStatefulWidget {
  const TransferScreen({
    super.key,
    this.wallet,
    this.wallets = const [],
    this.onSelectedWalletChanged,
    this.assets = const [],
    this.onSent,
    this.scanPayload,
    this.importImagePayload,
    this.qrScanner = const QrScanner(),
  });

  final WalletSummary? wallet;
  final List<WalletSummary> wallets;
  final ValueChanged<WalletSummary>? onSelectedWalletChanged;
  final List<AssetSummary> assets;
  final Future<void> Function()? onSent;
  final Future<String> Function()? scanPayload;
  final Future<String?> Function()? importImagePayload;
  final QrScanner qrScanner;

  @override
  ConsumerState<TransferScreen> createState() => _TransferScreenState();
}

class _TransferScreenState extends ConsumerState<TransferScreen>
    with TransferScreenQrFlow, TransferScreenSubmissionFlow {
  @override
  final TextEditingController _recipientController = TextEditingController();
  @override
  final TextEditingController _amountController = TextEditingController();
  bool _scanning = false;
  bool _importing = false;
  String? _selectedAssetId;

  @override
  void didUpdateWidget(covariant TransferScreen oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (_selectedAssetId != null &&
        !widget.assets.any((asset) => asset.id == _selectedAssetId)) {
      _selectedAssetId = TransferDraftBuilder.firstAvailableAssetId(
        widget.assets,
      );
      ref.read(transferControllerProvider.notifier).clearPreview();
    }
  }

  @override
  void dispose() {
    _recipientController.dispose();
    _amountController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final transfer = ref.watch(transferControllerProvider);
    return TransferScreenBody(
      assets: widget.assets,
      selectedAssetId: _draftBuilder.selectedAssetId,
      onAssetChanged: (assetId) {
        setState(() {
          _selectedAssetId = assetId;
        });
        ref.read(transferControllerProvider.notifier).clearPreview();
      },
      recipientController: _recipientController,
      amountController: _amountController,
      scanning: _scanning,
      importing: _importing,
      transfer: transfer,
      onScanQr: _startCameraScan,
      onImportQrImage: _startImageImport,
      onPreviewTransfer: _previewTransfer,
      onSendTransfer: _sendTransfer,
    );
  }

  @override
  TransferDraftBuilder get _draftBuilder => TransferDraftBuilder(
    wallet: widget.wallet,
    assets: widget.assets,
    selectedAssetId: _selectedAssetId,
    recipientController: _recipientController,
    amountController: _amountController,
  );
}
