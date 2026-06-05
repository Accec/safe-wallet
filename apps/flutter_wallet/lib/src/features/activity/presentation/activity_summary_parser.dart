enum ActivityTransferDirection { sent, received, transfer }

class ParsedActivityTransferSummary {
  const ParsedActivityTransferSummary({
    required this.direction,
    required this.amount,
    required this.symbol,
  });

  final ActivityTransferDirection direction;
  final String amount;
  final String symbol;
}

ParsedActivityTransferSummary? parseActivityTransferSummary(String summary) {
  final normalized = summary.trim();
  final directedMatch = RegExp(
    r'^(Sent|Received)\s+([0-9]+(?:\.[0-9]+)?)\s+([A-Za-z0-9._-]+)$',
    caseSensitive: false,
  ).firstMatch(normalized);
  if (directedMatch != null) {
    final direction = directedMatch.group(1)!.toLowerCase() == 'sent'
        ? ActivityTransferDirection.sent
        : ActivityTransferDirection.received;
    return ParsedActivityTransferSummary(
      direction: direction,
      amount: directedMatch.group(2)!,
      symbol: directedMatch.group(3)!,
    );
  }

  final transferMatch = RegExp(
    r'^Transfer\s+([0-9]+(?:\.[0-9]+)?)\s+([A-Za-z0-9._-]+)(?:\s+to\s+\S+)?$',
    caseSensitive: false,
  ).firstMatch(normalized);
  if (transferMatch == null) {
    return null;
  }
  return ParsedActivityTransferSummary(
    direction: ActivityTransferDirection.transfer,
    amount: transferMatch.group(1)!,
    symbol: transferMatch.group(2)!,
  );
}
