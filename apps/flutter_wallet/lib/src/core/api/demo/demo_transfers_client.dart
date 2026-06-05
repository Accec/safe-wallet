import '../../../models.dart';
import 'demo_wallet_store.dart';

class DemoTransfersClient {
  const DemoTransfersClient(this._store);

  final DemoWalletStore _store;

  Future<ParsedPayment> parsePaymentUri(String payload) async {
    return ParsedPayment(
      address: payload,
      chain: payload.startsWith('T') ? 'tron' : null,
    );
  }

  Future<TransferPreview> preview(TransferDraft draft) async {
    return TransferPreview(
      chain: draft.chain,
      fromAddress: '0x9858effd232b4033e47d90003d41ec34ecaeda94',
      toAddress: draft.toAddress,
      assetSymbol: _store.assets.first.symbol,
      amount: draft.amount,
      feeEstimate: '0.00042',
      rpcUrl: 'https://ethereum-rpc.publicnode.com',
    );
  }

  Future<TransferResult> send(TransferDraft draft, String password) async {
    const result = TransferResult(
      chain: 'ethereum',
      txHash: '0xbroadcasted',
      status: 'broadcasted',
    );
    _store.activity.add(
      ActivitySummary(
        chain: draft.chain,
        txHash: result.txHash,
        kind: 'native_transfer',
        status: 'pending',
        summary: 'Sent ${draft.amount} ${_store.assets.first.symbol}',
      ),
    );
    return result;
  }
}
