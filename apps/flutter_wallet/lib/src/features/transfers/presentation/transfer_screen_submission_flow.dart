part of 'transfer_screen_route.dart';

mixin TransferScreenSubmissionFlow on ConsumerState<TransferScreen> {
  TransferDraftBuilder get _draftBuilder;

  Future<void> _previewTransfer() async {
    final draft = _draftBuilder.buildDraft();
    if (draft == null) {
      showCreateWalletBeforePreview(context);
      return;
    }
    try {
      await _submissionActions.preview(draft);
      if (!mounted) {
        return;
      }
    } catch (error) {
      if (!mounted) {
        return;
      }
      showTransferError(context, error);
    }
  }

  Future<void> _sendTransfer() async {
    final password = await promptMasterPassword(
      context: context,
      title: 'Confirm transfer',
      actionLabel: 'Send',
    );
    if (!mounted || password == null) {
      return;
    }
    try {
      final result = await _submissionActions.send(password);
      if (!mounted) {
        return;
      }
      showTransferBroadcasted(context, result.txHash);
    } catch (error) {
      if (!mounted) {
        return;
      }
      showTransferError(context, error);
    }
  }

  TransferSubmissionActions get _submissionActions => TransferSubmissionActions(
    controller: ref.read(transferControllerProvider.notifier),
    onSent: widget.onSent,
  );
}
