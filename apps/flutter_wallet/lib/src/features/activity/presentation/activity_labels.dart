String activityKindLabel(String kind) {
  return switch (kind) {
    'native_transfer' => 'Transfer',
    'token_transfer' => 'Token transfer',
    'approval' => 'Approval',
    'swap' => 'Swap',
    'contract_call' => 'Contract call',
    _ => activityTitleCase(kind.replaceAll('_', ' ')),
  };
}

String shortActivityHash(String txHash) {
  if (txHash.length <= 18) {
    return txHash;
  }
  return '${txHash.substring(0, 8)}...${txHash.substring(txHash.length - 8)}';
}

String activityTitleCase(String value) {
  final words = value
      .trim()
      .replaceAll('_', ' ')
      .split(RegExp(r'\s+'))
      .where((word) => word.isNotEmpty)
      .toList();
  if (words.isEmpty) {
    return value;
  }
  return words
      .map(
        (word) => '${word[0].toUpperCase()}${word.substring(1).toLowerCase()}',
      )
      .join(' ');
}
