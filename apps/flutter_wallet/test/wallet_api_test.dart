import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_wallet/src/wallet_api.dart';

void main() {
  test('wallet api exception string includes the user-facing message', () {
    const error = WalletApiException('Native command failed');

    expect(error.toString(), 'Native command failed');
  });
}
