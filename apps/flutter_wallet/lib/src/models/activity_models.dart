class ActivitySummary {
  const ActivitySummary({
    required this.chain,
    required this.txHash,
    required this.kind,
    required this.status,
    required this.summary,
  });

  final String chain;
  final String txHash;
  final String kind;
  final String status;
  final String summary;
}
