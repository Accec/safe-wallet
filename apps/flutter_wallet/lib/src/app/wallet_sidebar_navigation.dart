import 'package:flutter/material.dart';

class WalletSidebarNavigation extends StatelessWidget {
  const WalletSidebarNavigation({
    super.key,
    required this.selectedIndex,
    required this.onSelectIndex,
  });

  final int selectedIndex;
  final ValueChanged<int> onSelectIndex;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        _SidebarNavItem(
          icon: Icons.account_balance_wallet_outlined,
          selectedIcon: Icons.account_balance_wallet,
          label: 'Wallets',
          selected: selectedIndex == 0,
          onTap: () => onSelectIndex(0),
        ),
        _SidebarNavItem(
          icon: Icons.token_outlined,
          selectedIcon: Icons.token,
          label: 'Assets',
          selected: selectedIndex == 1,
          onTap: () => onSelectIndex(1),
        ),
        _SidebarNavItem(
          icon: Icons.swap_horiz,
          selectedIcon: Icons.swap_horiz,
          label: 'Transfer',
          selected: selectedIndex == 2,
          onTap: () => onSelectIndex(2),
        ),
        _SidebarNavItem(
          icon: Icons.groups_outlined,
          selectedIcon: Icons.groups,
          label: 'Multisig',
          selected: selectedIndex == 3,
          onTap: () => onSelectIndex(3),
        ),
        _SidebarNavItem(
          icon: Icons.history,
          selectedIcon: Icons.history,
          label: 'Activity',
          selected: selectedIndex == 4,
          onTap: () => onSelectIndex(4),
        ),
        _SidebarNavItem(
          icon: Icons.settings_outlined,
          selectedIcon: Icons.settings,
          label: 'Settings',
          selected: selectedIndex == 5,
          onTap: () => onSelectIndex(5),
        ),
      ],
    );
  }
}

class _SidebarNavItem extends StatelessWidget {
  const _SidebarNavItem({
    required this.icon,
    required this.selectedIcon,
    required this.label,
    required this.selected,
    required this.onTap,
  });

  final IconData icon;
  final IconData selectedIcon;
  final String label;
  final bool selected;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 4),
      child: InkWell(
        borderRadius: BorderRadius.circular(8),
        onTap: onTap,
        child: Container(
          height: 42,
          padding: const EdgeInsets.symmetric(horizontal: 12),
          decoration: BoxDecoration(
            color: selected ? const Color(0xffeff6ff) : Colors.transparent,
            borderRadius: BorderRadius.circular(8),
          ),
          child: Row(
            children: [
              Icon(
                selected ? selectedIcon : icon,
                size: 20,
                color: selected
                    ? const Color(0xff2563eb)
                    : const Color(0xff4b5563),
              ),
              const SizedBox(width: 10),
              Text(
                label,
                style: TextStyle(
                  color: selected
                      ? const Color(0xff1d4ed8)
                      : const Color(0xff374151),
                  fontWeight: selected ? FontWeight.w700 : FontWeight.w600,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
