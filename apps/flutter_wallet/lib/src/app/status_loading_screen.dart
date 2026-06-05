import 'package:flutter/material.dart';

import '../core/ui/screen_motion.dart';

class StatusLoadingScreen extends StatelessWidget {
  const StatusLoadingScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SafeArea(
        child: Center(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              MotionEntrance(
                child: Column(
                  mainAxisSize: MainAxisSize.min,
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
                      'Safe Wallet',
                      style: Theme.of(context).textTheme.headlineMedium,
                    ),
                  ],
                ),
              ),
              const SizedBox(height: 20),
              MotionEntrance(
                delay: safeWalletLayerDelay(1),
                child: const SizedBox(
                  width: 160,
                  child: LinearProgressIndicator(),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
