import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:crypto/crypto.dart';
import 'package:flutter/services.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:path_provider/path_provider.dart';
import 'package:url_launcher/url_launcher.dart';

const String defaultUpdateOwner = 'Accec';
const String defaultUpdateRepo = 'safe-wallet';

enum UpdatePlatform { android, ios, macos, unsupported }

class AppUpdateException implements Exception {
  const AppUpdateException(this.message);

  final String message;

  @override
  String toString() => message;
}

class AppVersion implements Comparable<AppVersion> {
  const AppVersion({
    required this.major,
    required this.minor,
    required this.patch,
    this.preRelease,
  });

  factory AppVersion.parse(String value) {
    final normalized = value.trim().replaceFirst(RegExp(r'^v'), '');
    final separatorIndex = normalized.indexOf('-');
    final versionCore = separatorIndex == -1
        ? normalized
        : normalized.substring(0, separatorIndex);
    final preRelease = separatorIndex == -1
        ? null
        : normalized.substring(separatorIndex + 1);
    final core = versionCore.split('.');
    if (core.length < 3) {
      throw const AppUpdateException('Invalid update version');
    }
    return AppVersion(
      major: int.tryParse(core[0]) ?? 0,
      minor: int.tryParse(core[1]) ?? 0,
      patch: int.tryParse(core[2].split('+').first) ?? 0,
      preRelease: preRelease,
    );
  }

  final int major;
  final int minor;
  final int patch;
  final String? preRelease;

  @override
  int compareTo(AppVersion other) {
    final majorCompare = major.compareTo(other.major);
    if (majorCompare != 0) {
      return majorCompare;
    }
    final minorCompare = minor.compareTo(other.minor);
    if (minorCompare != 0) {
      return minorCompare;
    }
    final patchCompare = patch.compareTo(other.patch);
    if (patchCompare != 0) {
      return patchCompare;
    }
    if (preRelease == null && other.preRelease != null) {
      return 1;
    }
    if (preRelease != null && other.preRelease == null) {
      return -1;
    }
    return (preRelease ?? '').compareTo(other.preRelease ?? '');
  }

  bool operator >(AppVersion other) => compareTo(other) > 0;

  @override
  bool operator ==(Object other) =>
      other is AppVersion && compareTo(other) == 0;

  @override
  int get hashCode => Object.hash(major, minor, patch, preRelease);

  @override
  String toString() {
    final suffix = preRelease == null ? '' : '-$preRelease';
    return '$major.$minor.$patch$suffix';
  }
}

class AppUpdateAsset {
  const AppUpdateAsset({
    required this.name,
    required this.downloadUrl,
    required this.size,
    this.digest,
  });

  factory AppUpdateAsset.fromJson(Map<String, dynamic> json) {
    return AppUpdateAsset(
      name: _stringField(json, 'name'),
      downloadUrl: _stringField(json, 'browser_download_url'),
      size: _intField(json, 'size'),
      digest: json['digest'] is String ? json['digest'] as String : null,
    );
  }

  final String name;
  final String downloadUrl;
  final int size;
  final String? digest;
}

class GitHubRelease {
  const GitHubRelease({
    required this.tagName,
    required this.htmlUrl,
    required this.assets,
  });

  factory GitHubRelease.fromJson(Map<String, dynamic> json) {
    final assets = json['assets'];
    if (assets is! List<dynamic>) {
      throw const AppUpdateException('Invalid release response');
    }
    return GitHubRelease(
      tagName: _stringField(json, 'tag_name'),
      htmlUrl: _stringField(json, 'html_url'),
      assets: assets
          .whereType<Map<String, dynamic>>()
          .map(AppUpdateAsset.fromJson)
          .toList(growable: false),
    );
  }

  final String tagName;
  final String htmlUrl;
  final List<AppUpdateAsset> assets;

  AppVersion get version => AppVersion.parse(tagName);

  bool hasNewerVersionThan(String currentVersion) {
    return version.compareTo(AppVersion.parse(currentVersion)) > 0;
  }

  AppUpdateAsset? assetForPlatform(UpdatePlatform platform) {
    bool match(AppUpdateAsset asset, String platformName, String extension) {
      final name = asset.name.toLowerCase();
      return name.contains(platformName) && name.endsWith(extension);
    }

    return switch (platform) {
      UpdatePlatform.android => _firstWhereOrNull(
        assets,
        (asset) => match(asset, 'android', '.apk'),
      ),
      UpdatePlatform.macos => _firstWhereOrNull(
        assets,
        (asset) => match(asset, 'macos', '.zip'),
      ),
      UpdatePlatform.ios || UpdatePlatform.unsupported => null,
    };
  }
}

class AppUpdateInfo {
  const AppUpdateInfo({
    required this.currentVersion,
    required this.latestVersion,
    required this.updateAvailable,
    required this.releasePageUrl,
    required this.platform,
    this.asset,
  });

  final String currentVersion;
  final String latestVersion;
  final bool updateAvailable;
  final String releasePageUrl;
  final UpdatePlatform platform;
  final AppUpdateAsset? asset;

  String get actionLabel {
    if (platform == UpdatePlatform.ios || asset == null) {
      return 'Open download page';
    }
    if (platform == UpdatePlatform.android) {
      return 'Download and install';
    }
    if (platform == UpdatePlatform.macos) {
      return 'Download and install';
    }
    return 'Download update';
  }
}

class DownloadedUpdate {
  const DownloadedUpdate({required this.file, required this.info});

  final File file;
  final AppUpdateInfo info;
}

abstract class AppUpdateService {
  Future<AppUpdateInfo> checkForUpdate();
  Future<DownloadedUpdate> downloadUpdate(
    AppUpdateInfo info, {
    void Function(double progress)? onProgress,
  });
  Future<void> installOrOpenUpdate(DownloadedUpdate update);
  Future<void> openReleasePage(AppUpdateInfo info);
}

class GitHubAppUpdateService implements AppUpdateService {
  GitHubAppUpdateService({
    this.owner = defaultUpdateOwner,
    this.repo = defaultUpdateRepo,
    UpdatePlatform? platform,
    HttpClient? httpClient,
    MethodChannel? platformChannel,
  }) : platform = platform ?? currentUpdatePlatform(),
       _httpClient = httpClient ?? HttpClient(),
       _platformChannel =
           platformChannel ?? const MethodChannel('app.localwallet/app_update');

  final String owner;
  final String repo;
  final UpdatePlatform platform;
  final HttpClient _httpClient;
  final MethodChannel _platformChannel;

  @override
  Future<AppUpdateInfo> checkForUpdate() async {
    final packageInfo = await PackageInfo.fromPlatform();
    final release = await _fetchLatestRelease();
    final updateAvailable = release.hasNewerVersionThan(packageInfo.version);
    return AppUpdateInfo(
      currentVersion: packageInfo.version,
      latestVersion: release.tagName.replaceFirst(RegExp(r'^v'), ''),
      updateAvailable: updateAvailable,
      releasePageUrl: release.htmlUrl,
      platform: platform,
      asset: updateAvailable ? release.assetForPlatform(platform) : null,
    );
  }

  @override
  Future<DownloadedUpdate> downloadUpdate(
    AppUpdateInfo info, {
    void Function(double progress)? onProgress,
  }) async {
    final asset = info.asset;
    if (asset == null) {
      throw const AppUpdateException('No update asset is available');
    }

    final directory = await getApplicationSupportDirectory();
    final updateDirectory = Directory('${directory.path}/updates');
    if (!updateDirectory.existsSync()) {
      updateDirectory.createSync(recursive: true);
    }
    final target = File('${updateDirectory.path}/${asset.name}');
    await _downloadFile(asset.downloadUrl, target, onProgress: onProgress);
    await verifySha256Digest(target, asset.digest);
    return DownloadedUpdate(file: target, info: info);
  }

  @override
  Future<void> installOrOpenUpdate(DownloadedUpdate update) async {
    if (update.info.platform == UpdatePlatform.android) {
      await _platformChannel.invokeMethod<void>('installApk', {
        'path': update.file.path,
      });
      return;
    }
    if (update.info.platform == UpdatePlatform.macos) {
      await _platformChannel.invokeMethod<void>('installMacosUpdate', {
        'path': update.file.path,
      });
      return;
    }
    await openReleasePage(update.info);
  }

  @override
  Future<void> openReleasePage(AppUpdateInfo info) async {
    final uri = Uri.parse(info.releasePageUrl);
    if (!await launchUrl(uri, mode: LaunchMode.externalApplication)) {
      throw const AppUpdateException('Could not open update page');
    }
  }

  Future<GitHubRelease> _fetchLatestRelease() async {
    final uri = Uri.https(
      'api.github.com',
      '/repos/$owner/$repo/releases/latest',
    );
    final request = await _httpClient.getUrl(uri);
    request.headers.set(
      HttpHeaders.acceptHeader,
      'application/vnd.github+json',
    );
    request.headers.set(HttpHeaders.userAgentHeader, 'Safe-Wallet-Updater');
    final response = await request.close();
    final responseBody = await utf8.decodeStream(response);
    if (response.statusCode < 200 || response.statusCode >= 300) {
      throw const AppUpdateException('Update check failed');
    }
    final json = jsonDecode(responseBody);
    if (json is! Map<String, dynamic>) {
      throw const AppUpdateException('Invalid release response');
    }
    return GitHubRelease.fromJson(json);
  }

  Future<void> _downloadFile(
    String url,
    File target, {
    void Function(double progress)? onProgress,
  }) async {
    final request = await _httpClient.getUrl(Uri.parse(url));
    request.headers.set(HttpHeaders.userAgentHeader, 'Safe-Wallet-Updater');
    final response = await request.close();
    if (response.statusCode < 200 || response.statusCode >= 300) {
      throw const AppUpdateException('Update download failed');
    }

    final sink = target.openWrite();
    var received = 0;
    final total = response.contentLength;
    try {
      await for (final chunk in response) {
        received += chunk.length;
        sink.add(chunk);
        if (total > 0) {
          onProgress?.call(received / total);
        }
      }
    } finally {
      await sink.close();
    }
    onProgress?.call(1);
  }
}

UpdatePlatform currentUpdatePlatform() {
  if (Platform.isAndroid) {
    return UpdatePlatform.android;
  }
  if (Platform.isIOS) {
    return UpdatePlatform.ios;
  }
  if (Platform.isMacOS) {
    return UpdatePlatform.macos;
  }
  return UpdatePlatform.unsupported;
}

Future<void> verifySha256Digest(File file, String? digestValue) async {
  if (digestValue == null || digestValue.isEmpty) {
    return;
  }
  final normalized = digestValue.startsWith('sha256:')
      ? digestValue.substring('sha256:'.length)
      : digestValue;
  final actual = sha256.convert(await file.readAsBytes()).toString();
  if (actual.toLowerCase() != normalized.toLowerCase()) {
    throw const AppUpdateException('Downloaded update failed verification');
  }
}

String _stringField(Map<String, dynamic> json, String field) {
  final value = json[field];
  if (value is! String || value.isEmpty) {
    throw const AppUpdateException('Invalid release response');
  }
  return value;
}

int _intField(Map<String, dynamic> json, String field) {
  final value = json[field];
  if (value is int) {
    return value;
  }
  throw const AppUpdateException('Invalid release response');
}

T? _firstWhereOrNull<T>(Iterable<T> items, bool Function(T item) test) {
  for (final item in items) {
    if (test(item)) {
      return item;
    }
  }
  return null;
}
