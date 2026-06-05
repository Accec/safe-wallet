import 'package:flutter/material.dart';

import '../../../core/ui/busy_icon.dart';
import '../../../core/ui/screen_motion.dart';
import '../../../models.dart';
import '../multisig_controller.dart';
import 'multisig_tiles.dart';

class MultisigScreenBody extends StatelessWidget {
  const MultisigScreenBody({
    super.key,
    required this.multisig,
    required this.onCreateProposal,
    required this.onAddSignature,
  });

  final MultisigState multisig;
  final ValueChanged<MultisigAccountSummary> onCreateProposal;
  final ValueChanged<MultisigProposalSummary> onAddSignature;

  @override
  Widget build(BuildContext context) {
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        ..._accountRows(),
        if (multisig.proposals.isNotEmpty) ..._proposalRows(context),
      ],
    );
  }

  List<Widget> _accountRows() {
    if (multisig.loading) {
      return const [
        StaggeredListItem(
          index: 0,
          child: ListTile(
            leading: BusyIcon(),
            title: Text('Loading multisig accounts'),
          ),
        ),
      ];
    }
    if (multisig.accounts.isEmpty) {
      return const [
        StaggeredListItem(
          index: 0,
          child: ListTile(
            leading: Icon(Icons.groups_outlined),
            title: Text('No multisig accounts'),
            subtitle: Text('Import an EVM Safe or TRON permission account.'),
          ),
        ),
      ];
    }
    return [
      for (var index = 0; index < multisig.accounts.length; index++)
        StaggeredListItem(
          index: index,
          child: MultisigAccountTile(
            account: multisig.accounts[index],
            busy: multisig.busy,
            onCreateProposal: () => onCreateProposal(multisig.accounts[index]),
          ),
        ),
    ];
  }

  List<Widget> _proposalRows(BuildContext context) {
    return [
      const Divider(height: 32),
      Padding(
        padding: const EdgeInsets.fromLTRB(16, 0, 16, 8),
        child: Text(
          'Proposals',
          style: Theme.of(context).textTheme.titleMedium,
        ),
      ),
      for (var index = 0; index < multisig.proposals.length; index++)
        StaggeredListItem(
          index: index,
          child: MultisigProposalTile(
            proposal: multisig.proposals[index],
            busy: multisig.busy,
            onAddSignature: () => onAddSignature(multisig.proposals[index]),
          ),
        ),
    ];
  }
}
