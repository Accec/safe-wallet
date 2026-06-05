import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../biometric_auth.dart';
import '../../wallet_api.dart';

final walletApiProvider = Provider<WalletApi>((ref) => DemoWalletApi());

final biometricAuthProvider = Provider<BiometricAuth>(
  (ref) => LocalBiometricAuth(),
);
