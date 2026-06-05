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
  NetworkUrlPreset(name: 'TronGrid', url: 'https://api.trongrid.io'),
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

const indexerUrlPresets = [
  NetworkUrlPreset(
    name: 'Etherscan V2',
    url: 'https://api.etherscan.io/v2/api',
  ),
  NetworkUrlPreset(name: 'BscScan API', url: 'https://api.bscscan.com/api'),
  NetworkUrlPreset(
    name: 'PolygonScan API',
    url: 'https://api.polygonscan.com/api',
  ),
  NetworkUrlPreset(name: 'Arbiscan API', url: 'https://api.arbiscan.io/api'),
  NetworkUrlPreset(
    name: 'OP Etherscan API',
    url: 'https://api-optimistic.etherscan.io/api',
  ),
  NetworkUrlPreset(
    name: 'Tronscan API',
    url: 'https://apilist.tronscanapi.com/api',
  ),
];
