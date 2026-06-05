import '../../../models.dart';

Uri? activityScanUrl(ActivitySummary record) {
  final txHash = record.txHash.trim();
  if (txHash.isEmpty) {
    return null;
  }

  final encodedHash = Uri.encodeComponent(txHash);
  return switch (record.chain) {
    'ethereum' => Uri.parse('https://etherscan.io/tx/$encodedHash'),
    'bsc' => Uri.parse('https://bscscan.com/tx/$encodedHash'),
    'polygon' => Uri.parse('https://polygonscan.com/tx/$encodedHash'),
    'arbitrum' => Uri.parse('https://arbiscan.io/tx/$encodedHash'),
    'optimism' => Uri.parse('https://optimistic.etherscan.io/tx/$encodedHash'),
    'tron' => Uri.parse('https://tronscan.org/#/transaction/$encodedHash'),
    'btc' => Uri.parse('https://blockstream.info/tx/$encodedHash'),
    _ => null,
  };
}
