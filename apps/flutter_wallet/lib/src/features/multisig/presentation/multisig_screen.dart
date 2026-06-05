import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../models.dart';
import '../../../wallet_api.dart';
import '../multisig_controller.dart';
import 'multisig_dialogs.dart';
import 'multisig_screen_actions.dart';
import 'multisig_screen_body.dart';

class MultisigScreen extends ConsumerStatefulWidget {
  const MultisigScreen({super.key});

  @override
  ConsumerState<MultisigScreen> createState() => _MultisigScreenState();
}

class _MultisigScreenState extends ConsumerState<MultisigScreen> {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _reload();
    });
  }

  Future<void> _reload() async {
    try {
      await ref.read(multisigControllerProvider.notifier).load();
    } catch (error) {
      if (mounted) {
        _showError(error);
      }
    }
  }

  Future<void> _importAccount() async {
    final draft = await showImportMultisigDialog(context);
    if (draft == null) {
      return;
    }
    await _runBusy(
      () => ref.read(multisigControllerProvider.notifier).importAccount(draft),
    );
  }

  Future<void> _createProposal(MultisigAccountSummary account) async {
    final draft = await showCreateProposalDialog(context, account);
    if (draft == null) {
      return;
    }
    await _runBusy(
      () => ref.read(multisigControllerProvider.notifier).createProposal(draft),
    );
  }

  Future<void> _addSignature(MultisigProposalSummary proposal) async {
    final signature = await showSignatureDialog(context);
    if (signature == null) {
      return;
    }
    await _runBusy(
      () => ref
          .read(multisigControllerProvider.notifier)
          .addSignature(
            proposalId: proposal.id,
            ownerAddress: signature.ownerAddress,
            signature: signature.signature,
          ),
    );
  }

  Future<void> _runBusy(Future<void> Function() action) async {
    try {
      await action();
    } catch (error) {
      if (mounted) {
        _showError(error);
      }
    }
  }

  void _showError(Object error) {
    final message = error is WalletApiException
        ? error.message
        : 'Wallet action failed';
    ScaffoldMessenger.of(
      context,
    ).showSnackBar(SnackBar(content: Text(message)));
  }

  @override
  Widget build(BuildContext context) {
    final multisig = ref.watch(multisigControllerProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Multisig'),
        actions: [
          MultisigScreenActions(
            loading: multisig.loading,
            busy: multisig.busy,
            onReload: _reload,
            onImportAccount: _importAccount,
          ),
        ],
      ),
      body: MultisigScreenBody(
        multisig: multisig,
        onCreateProposal: _createProposal,
        onAddSignature: _addSignature,
      ),
    );
  }
}
