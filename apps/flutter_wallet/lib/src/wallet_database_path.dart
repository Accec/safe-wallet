import 'dart:io';

import 'package:path_provider/path_provider.dart';

Future<String> defaultWalletDatabasePath() async {
  if (Platform.isMacOS) {
    final home = Platform.environment['HOME'];
    if (home == null || home.isEmpty) {
      throw StateError('Unable to resolve macOS home directory.');
    }
    final walletDirectory = Directory(
      '$home/Library/Application Support/Local Wallet/local_wallet',
    );
    if (!walletDirectory.existsSync()) {
      walletDirectory.createSync(recursive: true);
    }
    return '${walletDirectory.path}/wallet.sqlite';
  }

  final directory = await getApplicationSupportDirectory();
  final walletDirectory = Directory('${directory.path}/local_wallet');
  if (!walletDirectory.existsSync()) {
    walletDirectory.createSync(recursive: true);
  }
  return '${walletDirectory.path}/wallet.sqlite';
}
