import 'package:flutter/widgets.dart';

import '../../models.dart';

const defaultTorProxyUrl = 'socks5h://127.0.0.1:9050';

class NetworkPrivacyDraftController {
  NetworkPrivacyDraftController({
    required this.proxyEnabled,
    required this.proxyMode,
    required String? proxyUrl,
  }) : proxyUrl = TextEditingController(text: proxyUrl) {
    _ensureTorDefaultUrl();
  }

  factory NetworkPrivacyDraftController.fromSettings(
    NetworkPrivacySettings settings,
  ) {
    return NetworkPrivacyDraftController(
      proxyEnabled: settings.proxyEnabled,
      proxyMode: settings.proxyMode,
      proxyUrl: settings.proxyUrl,
    );
  }

  bool proxyEnabled;
  String proxyMode;
  final TextEditingController proxyUrl;

  void setProxyEnabled(bool value) {
    proxyEnabled = value;
    _ensureTorDefaultUrl();
  }

  void setProxyMode(String mode) {
    proxyMode = mode;
    _ensureTorDefaultUrl();
  }

  NetworkPrivacySettingsDraft toDraft() {
    final rawUrl = proxyUrl.text.trim();
    final normalizedProxyUrl =
        proxyEnabled && proxyMode == 'tor' && rawUrl.isEmpty
        ? defaultTorProxyUrl
        : rawUrl.isEmpty
        ? null
        : rawUrl;
    return NetworkPrivacySettingsDraft(
      proxyEnabled: proxyEnabled,
      proxyMode: proxyMode,
      proxyUrl: normalizedProxyUrl,
    );
  }

  void dispose() {
    proxyUrl.dispose();
  }

  void _ensureTorDefaultUrl() {
    if (proxyEnabled && proxyMode == 'tor' && proxyUrl.text.isEmpty) {
      proxyUrl.text = defaultTorProxyUrl;
    }
  }
}
