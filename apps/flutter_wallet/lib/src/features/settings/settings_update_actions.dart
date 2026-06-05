import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../app_update.dart';
import '../../core/state/wallet_providers.dart';

class SettingsUpdateActions {
  const SettingsUpdateActions({
    required this.context,
    required this.ref,
    required this.isMounted,
    required this.setCheckingUpdate,
    required this.setUpdating,
    required this.setUpdateProgress,
    required this.setUpdateInfo,
  });

  final BuildContext context;
  final WidgetRef ref;
  final bool Function() isMounted;
  final ValueChanged<bool> setCheckingUpdate;
  final ValueChanged<bool> setUpdating;
  final ValueChanged<double?> setUpdateProgress;
  final ValueChanged<AppUpdateInfo?> setUpdateInfo;

  Future<void> checkForUpdates() async {
    setCheckingUpdate(true);
    setUpdateProgress(null);
    try {
      final info = await ref.read(appUpdateServiceProvider).checkForUpdate();
      if (!context.mounted || !isMounted()) {
        return;
      }
      setUpdateInfo(info);
      _showSnackBar(
        info.updateAvailable
            ? 'Update available.'
            : 'You are running the latest version.',
      );
    } catch (error) {
      if (context.mounted && isMounted()) {
        _showUpdateError(error);
      }
    } finally {
      if (context.mounted && isMounted()) {
        setCheckingUpdate(false);
      }
    }
  }

  Future<void> runUpdateAction(AppUpdateInfo? info) async {
    if (info == null) {
      return;
    }
    final updateService = ref.read(appUpdateServiceProvider);
    setUpdating(true);
    setUpdateProgress(info.asset == null ? null : 0);
    try {
      if (info.asset == null) {
        await updateService.openReleasePage(info);
        if (!context.mounted || !isMounted()) {
          return;
        }
        _showSnackBar('Update page opened.');
        return;
      }
      final downloaded = await updateService.downloadUpdate(
        info,
        onProgress: (progress) {
          if (!context.mounted || !isMounted()) {
            return;
          }
          setUpdateProgress(progress.clamp(0, 1));
        },
      );
      await updateService.installOrOpenUpdate(downloaded);
      if (!context.mounted || !isMounted()) {
        return;
      }
      _showSnackBar('Update installer opened.');
    } catch (error) {
      if (context.mounted && isMounted()) {
        _showUpdateError(error);
      }
    } finally {
      if (context.mounted && isMounted()) {
        setUpdating(false);
        setUpdateProgress(null);
      }
    }
  }

  void _showUpdateError(Object error) {
    final message = error is AppUpdateException
        ? error.message
        : 'Update failed';
    _showSnackBar(message);
  }

  void _showSnackBar(String message) {
    final messenger = ScaffoldMessenger.of(context);
    messenger.hideCurrentSnackBar();
    messenger.showSnackBar(SnackBar(content: Text(message)));
  }
}
