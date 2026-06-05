import 'package:flutter/material.dart';

const walletHomeDestinations = [
  NavigationDestination(
    icon: Icon(Icons.account_balance_wallet_outlined),
    selectedIcon: Icon(Icons.account_balance_wallet),
    label: 'Wallets',
  ),
  NavigationDestination(
    icon: Icon(Icons.token_outlined),
    selectedIcon: Icon(Icons.token),
    label: 'Assets',
  ),
  NavigationDestination(icon: Icon(Icons.swap_horiz), label: 'Transfer'),
  NavigationDestination(
    icon: Icon(Icons.groups_outlined),
    selectedIcon: Icon(Icons.groups),
    label: 'Multisig',
  ),
  NavigationDestination(icon: Icon(Icons.history), label: 'Activity'),
  NavigationDestination(
    icon: Icon(Icons.settings_outlined),
    selectedIcon: Icon(Icons.settings),
    label: 'Settings',
  ),
];
