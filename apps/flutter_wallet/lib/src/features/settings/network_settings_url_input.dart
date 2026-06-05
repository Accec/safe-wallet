import 'package:flutter/material.dart';

import 'network_settings_presets.dart';

class NetworkSettingsUrlInput extends StatelessWidget {
  const NetworkSettingsUrlInput({
    super.key,
    required this.controller,
    required this.enabled,
    required this.labelText,
    required this.presets,
  });

  final TextEditingController controller;
  final bool enabled;
  final String labelText;
  final List<NetworkUrlPreset> presets;

  @override
  Widget build(BuildContext context) {
    return TextField(
      controller: controller,
      enabled: enabled,
      decoration: InputDecoration(
        labelText: labelText,
        hintText: 'Add URL',
        suffixIcon: PopupMenuButton<String>(
          tooltip: 'Select URL',
          icon: const Icon(Icons.expand_more),
          enabled: enabled,
          itemBuilder: (context) => [
            for (final preset in presets)
              PopupMenuItem(
                value: preset.url,
                child: Text('${preset.name} · ${preset.url}'),
              ),
          ],
          onSelected: (url) {
            controller.text = url;
          },
        ),
      ),
    );
  }
}
