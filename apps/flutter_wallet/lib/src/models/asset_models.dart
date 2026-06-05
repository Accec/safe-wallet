class AssetSummary {
  const AssetSummary({
    required this.id,
    required this.chain,
    required this.symbol,
    required this.name,
    required this.decimals,
    required this.kind,
    this.balance = '0',
    this.contractAddress,
  });

  final String id;
  final String chain;
  final String symbol;
  final String name;
  final int decimals;
  final String kind;
  final String balance;
  final String? contractAddress;
}
