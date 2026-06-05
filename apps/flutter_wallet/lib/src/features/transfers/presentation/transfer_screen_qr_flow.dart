part of 'transfer_screen_route.dart';

mixin TransferScreenQrFlow on ConsumerState<TransferScreen> {
  TextEditingController get _recipientController;
  TextEditingController get _amountController;
  set _selectedAssetId(String? value);
  set _scanning(bool value);
  set _importing(bool value);

  Future<void> _startCameraScan() async {
    setState(() {
      _scanning = true;
    });

    try {
      await _loadPaymentRequest(_qrActions.loadFromCamera);
    } catch (error) {
      if (!mounted) {
        return;
      }
      showTransferError(context, error);
    } finally {
      if (mounted) {
        setState(() {
          _scanning = false;
        });
      }
    }
  }

  Future<void> _startImageImport() async {
    setState(() {
      _importing = true;
    });

    try {
      await _loadPaymentRequest(_qrActions.loadFromImage);
    } catch (error) {
      if (!mounted) {
        return;
      }
      showTransferError(context, error);
    } finally {
      if (mounted) {
        setState(() {
          _importing = false;
        });
      }
    }
  }

  Future<void> _loadPaymentRequest(
    Future<TransferQrActionResult?> Function() load,
  ) async {
    final result = await load();
    if (!mounted || result == null) {
      return;
    }
    setState(() {
      final selectedAssetId = result.selectedAssetId;
      if (selectedAssetId != null) {
        _selectedAssetId = selectedAssetId;
      }
    });
    ref.read(transferControllerProvider.notifier).clearPreview();
    showPaymentRequestLoaded(context);
  }

  TransferQrActions get _qrActions => TransferQrActions(
    assets: widget.assets,
    recipientController: _recipientController,
    amountController: _amountController,
    parsePaymentUri: ref
        .read(transferControllerProvider.notifier)
        .parsePaymentUri,
    qrScanner: widget.qrScanner,
    scanPayload: widget.scanPayload,
    importImagePayload: widget.importImagePayload,
  );
}
