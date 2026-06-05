import 'package:flutter/material.dart';

class UnlockBrandHeader extends StatelessWidget {
  const UnlockBrandHeader({super.key, required this.initialized});

  final bool initialized;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        ClipRRect(
          borderRadius: BorderRadius.circular(18),
          child: Image.asset(
            'assets/branding/safe_wallet_icon.png',
            width: 72,
            height: 72,
          ),
        ),
        const SizedBox(height: 16),
        Text(
          initialized ? 'Safe Wallet' : 'Set master password',
          textAlign: TextAlign.center,
          style: Theme.of(context).textTheme.headlineMedium,
        ),
      ],
    );
  }
}
