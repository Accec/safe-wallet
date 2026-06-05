import 'package:flutter/material.dart';

import '../chains/supported_chains.dart';

export '../chains/supported_chains.dart';

class ChainSelector extends StatelessWidget {
  const ChainSelector({
    super.key,
    required this.chains,
    required this.selectedChain,
    required this.onChanged,
  });

  final List<String> chains;
  final String? selectedChain;
  final ValueChanged<String> onChanged;

  @override
  Widget build(BuildContext context) {
    if (chains.isEmpty || selectedChain == null) {
      return const SizedBox.shrink();
    }
    return Tooltip(
      message: 'Select chain',
      child: Container(
        height: 44,
        padding: const EdgeInsets.symmetric(horizontal: 12),
        decoration: BoxDecoration(
          color: const Color(0xfff9fafb),
          borderRadius: BorderRadius.circular(8),
          border: Border.all(color: const Color(0xffd1d5db)),
        ),
        child: Row(
          children: [
            const Icon(Icons.hub_outlined, size: 18, color: Color(0xff4b5563)),
            const SizedBox(width: 8),
            Expanded(
              child: DropdownButtonHideUnderline(
                child: DropdownButton<String>(
                  value: selectedChain,
                  icon: const Icon(Icons.expand_more),
                  isExpanded: true,
                  borderRadius: BorderRadius.circular(8),
                  items: [
                    for (final chain in chains)
                      DropdownMenuItem(
                        value: chain,
                        child: Text(
                          chainLabel(chain),
                          overflow: TextOverflow.ellipsis,
                        ),
                      ),
                  ],
                  onChanged: (chain) {
                    if (chain != null) {
                      onChanged(chain);
                    }
                  },
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class CompactChainSelector extends StatelessWidget {
  const CompactChainSelector({
    super.key,
    required this.chains,
    required this.selectedChain,
    required this.onChanged,
  });

  final List<String> chains;
  final String? selectedChain;
  final ValueChanged<String> onChanged;

  @override
  Widget build(BuildContext context) {
    if (chains.isEmpty || selectedChain == null) {
      return const SizedBox.shrink();
    }
    return SizedBox(
      width: 180,
      child: ChainSelector(
        chains: chains,
        selectedChain: selectedChain,
        onChanged: onChanged,
      ),
    );
  }
}
