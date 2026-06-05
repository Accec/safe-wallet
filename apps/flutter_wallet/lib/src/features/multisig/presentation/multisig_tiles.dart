import 'package:flutter/material.dart';

import '../../../models.dart';

class MultisigAccountTile extends StatelessWidget {
  const MultisigAccountTile({
    super.key,
    required this.account,
    required this.busy,
    required this.onCreateProposal,
  });

  final MultisigAccountSummary account;
  final bool busy;
  final VoidCallback onCreateProposal;

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: const Icon(Icons.groups_outlined),
      title: Text(account.label),
      subtitle: Text(
        '${account.kind} · ${account.chain} · ${account.threshold} approvals\n${account.address}',
      ),
      isThreeLine: true,
      trailing: IconButton(
        tooltip: 'Create multisig proposal',
        onPressed: busy ? null : onCreateProposal,
        icon: const Icon(Icons.note_add_outlined),
      ),
    );
  }
}

class MultisigProposalTile extends StatelessWidget {
  const MultisigProposalTile({
    super.key,
    required this.proposal,
    required this.busy,
    required this.onAddSignature,
  });

  final MultisigProposalSummary proposal;
  final bool busy;
  final VoidCallback onAddSignature;

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: Icon(
        proposal.status == 'ready'
            ? Icons.verified_outlined
            : Icons.pending_outlined,
      ),
      title: Text('${proposal.amount} ${proposal.assetSymbol}'),
      subtitle: Text(
        '${proposal.status} · ${proposal.signatureWeight}/${proposal.threshold}\n${proposal.toAddress}',
      ),
      isThreeLine: true,
      trailing: IconButton(
        tooltip: 'Add multisig signature',
        onPressed: busy ? null : onAddSignature,
        icon: const Icon(Icons.edit_outlined),
      ),
    );
  }
}
