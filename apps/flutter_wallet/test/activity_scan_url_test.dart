import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_wallet/src/features/activity/presentation/activity_scan_url.dart';
import 'package:flutter_wallet/src/models.dart';

void main() {
  test(
    'activity scan url maps each supported chain to its transaction page',
    () {
      final cases = <String, String>{
        'ethereum': 'https://etherscan.io/tx/0xabc',
        'bsc': 'https://bscscan.com/tx/0xabc',
        'polygon': 'https://polygonscan.com/tx/0xabc',
        'arbitrum': 'https://arbiscan.io/tx/0xabc',
        'optimism': 'https://optimistic.etherscan.io/tx/0xabc',
        'tron': 'https://tronscan.org/#/transaction/0xabc',
        'btc': 'https://blockstream.info/tx/0xabc',
      };

      for (final entry in cases.entries) {
        expect(
          activityScanUrl(_activity(chain: entry.key)).toString(),
          entry.value,
        );
      }
    },
  );

  test('activity scan url ignores unsupported chains and blank hashes', () {
    expect(activityScanUrl(_activity(chain: 'unknown')), isNull);
    expect(activityScanUrl(_activity(chain: 'ethereum', txHash: ' ')), isNull);
  });
}

ActivitySummary _activity({required String chain, String txHash = '0xabc'}) {
  return ActivitySummary(
    chain: chain,
    txHash: txHash,
    kind: 'native_transfer',
    status: 'confirmed',
    summary: 'Received 1',
  );
}
