import '../../models.dart';

const safeWalletSupportedChains = [
  'ethereum',
  'bsc',
  'polygon',
  'arbitrum',
  'optimism',
  'tron',
  'btc',
];

List<String> accountChains(List<AccountSummary> accounts) {
  return orderedChains(accounts.map((account) => account.chain));
}

List<String> assetChains(List<AssetSummary> assets) {
  return orderedChains(assets.map((asset) => asset.chain));
}

List<String> orderedChains(Iterable<String> chains) {
  final ordered = chains.toSet().toList();
  ordered.sort((left, right) {
    final leftIndex = chainSortRank(left);
    final rightIndex = chainSortRank(right);
    if (leftIndex != rightIndex) {
      return leftIndex.compareTo(rightIndex);
    }
    return left.compareTo(right);
  });
  return ordered;
}

String? effectiveChain(List<String> chains, String? selectedChain) {
  if (chains.isEmpty) {
    return null;
  }
  if (selectedChain != null && chains.contains(selectedChain)) {
    return selectedChain;
  }
  if (chains.contains('ethereum')) {
    return 'ethereum';
  }
  return chains.first;
}

String chainLabel(String chain) {
  switch (chain) {
    case 'btc':
      return 'Bitcoin';
    case 'bsc':
      return 'BSC';
    case 'ethereum':
      return 'Ethereum';
    case 'polygon':
      return 'Polygon';
    case 'arbitrum':
      return 'Arbitrum';
    case 'optimism':
      return 'Optimism';
    case 'tron':
      return 'Tron';
    default:
      return chain;
  }
}

int chainSortRank(String chain) {
  final index = safeWalletSupportedChains.indexOf(chain);
  return index == -1 ? safeWalletSupportedChains.length : index;
}
