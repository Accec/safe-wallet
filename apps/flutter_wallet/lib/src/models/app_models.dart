class AppStatus {
  const AppStatus({
    this.initialized = true,
    required this.locked,
    this.biometricEnabled = false,
  });

  final bool initialized;
  final bool locked;
  final bool biometricEnabled;
}
