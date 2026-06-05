import '../../../models.dart';
import '../transfer_controller.dart';

class TransferSubmissionActions {
  const TransferSubmissionActions({required this.controller, this.onSent});

  final TransferController controller;
  final Future<void> Function()? onSent;

  Future<void> preview(TransferDraft draft) {
    controller.clearPreview();
    return controller.preview(draft);
  }

  Future<TransferResult> send(String password) async {
    final result = await controller.send(password);
    await onSent?.call();
    return result;
  }
}
