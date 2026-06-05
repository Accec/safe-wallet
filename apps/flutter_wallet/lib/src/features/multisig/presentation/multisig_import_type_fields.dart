import 'package:flutter/material.dart';

class MultisigImportTypeFields extends StatelessWidget {
  const MultisigImportTypeFields({
    super.key,
    required this.kind,
    required this.chain,
    required this.onKindChanged,
    required this.onChainChanged,
  });

  final String kind;
  final String chain;
  final ValueChanged<String> onKindChanged;
  final ValueChanged<String>? onChainChanged;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        DropdownButtonFormField<String>(
          initialValue: kind,
          decoration: const InputDecoration(labelText: 'Type'),
          items: const [
            DropdownMenuItem(value: 'evm_safe', child: Text('EVM Safe')),
            DropdownMenuItem(
              value: 'tron_permission',
              child: Text('TRON permission'),
            ),
          ],
          onChanged: (value) {
            if (value != null) {
              onKindChanged(value);
            }
          },
        ),
        const SizedBox(height: 12),
        DropdownButtonFormField<String>(
          initialValue: chain,
          decoration: const InputDecoration(labelText: 'Chain'),
          items: const [
            DropdownMenuItem(value: 'ethereum', child: Text('Ethereum')),
            DropdownMenuItem(value: 'bsc', child: Text('BSC')),
            DropdownMenuItem(value: 'polygon', child: Text('Polygon')),
            DropdownMenuItem(value: 'arbitrum', child: Text('Arbitrum')),
            DropdownMenuItem(value: 'optimism', child: Text('Optimism')),
            DropdownMenuItem(value: 'tron', child: Text('TRON')),
          ],
          onChanged: onChainChanged == null
              ? null
              : (value) {
                  if (value != null) {
                    onChainChanged!(value);
                  }
                },
        ),
      ],
    );
  }
}
