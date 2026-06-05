import 'package:flutter/material.dart';

class WalletsEmptyState extends StatelessWidget {
  const WalletsEmptyState({super.key});

  @override
  Widget build(BuildContext context) {
    return const ListTile(
      leading: Icon(Icons.account_balance_wallet_outlined),
      title: Text('No wallets'),
      subtitle: Text('Create or import a recovery phrase.'),
    );
  }
}
