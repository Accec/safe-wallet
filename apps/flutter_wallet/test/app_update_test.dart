import 'dart:convert';
import 'dart:io';

import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_wallet/src/app_update.dart';

void main() {
  group('AppVersion', () {
    test('compares v-prefixed semantic versions', () {
      expect(AppVersion.parse('v1.2.3') > AppVersion.parse('1.2.2'), isTrue);
      expect(AppVersion.parse('1.2.3') == AppVersion.parse('v1.2.3'), isTrue);
      expect(
        AppVersion.parse('1.2.3') > AppVersion.parse('1.2.3-rc.1'),
        isTrue,
      );
      expect(AppVersion.parse('1.10.0') > AppVersion.parse('1.2.9'), isTrue);
    });
  });

  group('GitHubRelease', () {
    test('selects android apk and macos zip assets', () {
      final release = GitHubRelease.fromJson(_releaseJson());

      expect(
        release.assetForPlatform(UpdatePlatform.android)?.name,
        'safe-wallet-android-v1.2.3.apk',
      );
      expect(
        release.assetForPlatform(UpdatePlatform.macos)?.name,
        'safe-wallet-macos-v1.2.3.zip',
      );
      expect(release.assetForPlatform(UpdatePlatform.ios), isNull);
    });

    test('detects update availability from current app version', () {
      final release = GitHubRelease.fromJson(_releaseJson());

      expect(release.hasNewerVersionThan('1.2.2'), isTrue);
      expect(release.hasNewerVersionThan('1.2.3'), isFalse);
      expect(release.hasNewerVersionThan('1.3.0'), isFalse);
    });
  });

  test('verifySha256Digest rejects corrupted downloads', () async {
    final file = File('${Directory.systemTemp.path}/safe-wallet-bad.bin');
    await file.writeAsString('bad update payload');
    addTearDown(() {
      if (file.existsSync()) {
        file.deleteSync();
      }
    });

    expect(
      () => verifySha256Digest(file, 'sha256:${'0' * 64}'),
      throwsA(isA<AppUpdateException>()),
    );
  });

  test('macos install action asks native layer to replace the app', () async {
    TestWidgetsFlutterBinding.ensureInitialized();
    const channel = MethodChannel('test.safe_wallet/app_update');
    MethodCall? capturedCall;
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(channel, (call) async {
          capturedCall = call;
          return null;
        });
    addTearDown(() {
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(channel, null);
    });

    final file = File(
      '${Directory.systemTemp.path}/safe-wallet-macos-v1.2.3.zip',
    );
    final service = GitHubAppUpdateService(
      platform: UpdatePlatform.macos,
      platformChannel: channel,
    );

    await service.installOrOpenUpdate(
      DownloadedUpdate(
        file: file,
        info: AppUpdateInfo(
          currentVersion: '1.2.2',
          latestVersion: '1.2.3',
          updateAvailable: true,
          releasePageUrl:
              'https://github.com/Accec/safe-wallet/releases/tag/v1.2.3',
          platform: UpdatePlatform.macos,
          asset: AppUpdateAsset(
            name: 'safe-wallet-macos-v1.2.3.zip',
            downloadUrl: 'https://example.invalid/macos.zip',
            size: 200,
            digest:
                'sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb',
          ),
        ),
      ),
    );

    expect(capturedCall?.method, 'installMacosUpdate');
    expect(capturedCall?.arguments, {'path': file.path});
  });
}

Map<String, dynamic> _releaseJson() {
  return jsonDecode('''
{
  "tag_name": "v1.2.3",
  "name": "Safe Wallet v1.2.3",
  "html_url": "https://github.com/Accec/safe-wallet/releases/tag/v1.2.3",
  "assets": [
    {
      "name": "safe-wallet-android-v1.2.3.apk",
      "browser_download_url": "https://example.invalid/android.apk",
      "size": 100,
      "digest": "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    },
    {
      "name": "safe-wallet-macos-v1.2.3.zip",
      "browser_download_url": "https://example.invalid/macos.zip",
      "size": 200,
      "digest": "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    }
  ]
}
''')
      as Map<String, dynamic>;
}
