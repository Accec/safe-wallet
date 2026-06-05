import '../../../models.dart';

class DemoWalletStore {
  bool locked = true;
  bool initialized = false;
  bool biometricEnabled = false;
  NetworkPrivacySettings networkPrivacySettings = const NetworkPrivacySettings(
    proxyEnabled: false,
    proxyMode: 'custom',
  );
  final wallets = <WalletSummary>[];
  final assets = <AssetSummary>[
    const AssetSummary(
      id: 'demo-eth',
      chain: 'ethereum',
      symbol: 'ETH',
      name: 'ETH',
      decimals: 18,
      kind: 'native',
    ),
  ];
  final activity = <ActivitySummary>[];
  final multisigAccounts = <MultisigAccountSummary>[];
  final multisigProposals = <MultisigProposalSummary>[];
  final networks = <NetworkSettings>[
    const NetworkSettings(
      chain: 'ethereum',
      networkName: 'Ethereum',
      chainId: '1',
      enabled: true,
      defaultRpcUrl: 'https://ethereum-rpc.publicnode.com',
      nativeSymbol: 'ETH',
      nativeDecimals: 18,
      explorerUrl: 'https://etherscan.io',
    ),
    const NetworkSettings(
      chain: 'bsc',
      networkName: 'BNB Smart Chain',
      chainId: '56',
      enabled: true,
      defaultRpcUrl: 'https://bsc-rpc.publicnode.com',
      nativeSymbol: 'BNB',
      nativeDecimals: 18,
      explorerUrl: 'https://bscscan.com',
    ),
  ];
}
