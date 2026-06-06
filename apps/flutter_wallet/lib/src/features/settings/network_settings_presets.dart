class NetworkUrlPreset {
  const NetworkUrlPreset({required this.name, required this.url});

  final String name;
  final String url;
}

const rpcUrlPresets = [
  NetworkUrlPreset(
    name: 'Ethereum PublicNode',
    url: 'https://ethereum-rpc.publicnode.com',
  ),
  NetworkUrlPreset(
    name: 'BSC PublicNode',
    url: 'https://bsc-rpc.publicnode.com',
  ),
  NetworkUrlPreset(
    name: 'Polygon PublicNode',
    url: 'https://polygon-bor-rpc.publicnode.com',
  ),
  NetworkUrlPreset(
    name: 'Arbitrum PublicNode',
    url: 'https://arbitrum-one-rpc.publicnode.com',
  ),
  NetworkUrlPreset(
    name: 'Optimism PublicNode',
    url: 'https://optimism-rpc.publicnode.com',
  ),
  NetworkUrlPreset(
    name: 'Tron PublicNode',
    url: 'https://tron-rpc.publicnode.com',
  ),
];

const explorerUrlPresets = [
  NetworkUrlPreset(name: 'Etherscan', url: 'https://etherscan.io'),
  NetworkUrlPreset(name: 'BscScan', url: 'https://bscscan.com'),
  NetworkUrlPreset(name: 'PolygonScan', url: 'https://polygonscan.com'),
  NetworkUrlPreset(name: 'Arbiscan', url: 'https://arbiscan.io'),
  NetworkUrlPreset(
    name: 'OP Etherscan',
    url: 'https://optimistic.etherscan.io',
  ),
  NetworkUrlPreset(name: 'Tronscan', url: 'https://tronscan.org'),
];

const scanUrlPresets = [
  NetworkUrlPreset(name: 'Etherscan', url: 'https://etherscan.io'),
  NetworkUrlPreset(name: 'BscScan', url: 'https://bscscan.com'),
  NetworkUrlPreset(name: 'PolygonScan', url: 'https://polygonscan.com'),
  NetworkUrlPreset(name: 'Arbiscan', url: 'https://arbiscan.io'),
  NetworkUrlPreset(
    name: 'OP Etherscan',
    url: 'https://optimistic.etherscan.io',
  ),
  NetworkUrlPreset(name: 'TronScan', url: 'https://apilist.tronscan.org/api'),
];
