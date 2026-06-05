typedef AddCustomTokenCallback =
    Future<void> Function({
      required String chain,
      required String contractAddress,
      required String tokenName,
    });

typedef RemoveCustomTokenCallback = Future<void> Function(String assetId);
