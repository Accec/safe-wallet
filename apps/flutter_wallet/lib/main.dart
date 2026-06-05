import 'package:flutter/material.dart';

import 'src/app/wallet_app.dart';
import 'src/native_wallet_api.dart';
import 'src/wallet_database_path.dart';

export 'src/app/wallet_app.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final dbPath = await defaultWalletDatabasePath();
  runApp(WalletApp(api: NativeWalletApi(dbPath: dbPath)));
}
