import 'package:flutter/material.dart';

ThemeData safeWalletTheme() {
  return ThemeData(
    colorScheme: ColorScheme.fromSeed(
      seedColor: const Color(0xff2563eb),
      primary: const Color(0xff2563eb),
      secondary: const Color(0xff0f766e),
      tertiary: const Color(0xff7c3aed),
      surface: Colors.white,
      surfaceContainerHighest: const Color(0xffeef2f7),
    ),
    useMaterial3: true,
    scaffoldBackgroundColor: const Color(0xfff6f7f9),
    appBarTheme: const AppBarTheme(
      centerTitle: false,
      elevation: 0,
      scrolledUnderElevation: 0,
      backgroundColor: Color(0xfff6f7f9),
      foregroundColor: Color(0xff111827),
      titleTextStyle: TextStyle(
        color: Color(0xff111827),
        fontSize: 22,
        fontWeight: FontWeight.w700,
      ),
    ),
    navigationBarTheme: NavigationBarThemeData(
      backgroundColor: Colors.white,
      indicatorColor: const Color(0xffe0f2fe),
      labelTextStyle: WidgetStateProperty.resolveWith((states) {
        final selected = states.contains(WidgetState.selected);
        return TextStyle(
          fontSize: 11,
          fontWeight: selected ? FontWeight.w700 : FontWeight.w600,
          color: selected ? const Color(0xff1d4ed8) : const Color(0xff4b5563),
        );
      }),
    ),
    filledButtonTheme: FilledButtonThemeData(
      style: FilledButton.styleFrom(
        minimumSize: const Size(44, 44),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
      ),
    ),
    outlinedButtonTheme: OutlinedButtonThemeData(
      style: OutlinedButton.styleFrom(
        minimumSize: const Size(44, 44),
        side: const BorderSide(color: Color(0xffd1d5db)),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
      ),
    ),
    inputDecorationTheme: InputDecorationTheme(
      filled: true,
      fillColor: Colors.white,
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: Color(0xffd1d5db)),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: Color(0xffd1d5db)),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: Color(0xff2563eb)),
      ),
    ),
    listTileTheme: ListTileThemeData(
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
    ),
  );
}
