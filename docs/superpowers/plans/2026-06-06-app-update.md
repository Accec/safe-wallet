# App Update Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add an in-app update flow that checks GitHub Releases and performs the best available platform action.

**Architecture:** Add a focused Dart update service for release discovery, version comparison, download, digest verification, and platform actions. Wire it into Settings with an injectable interface so widget tests use a fake service. Add one Android MethodChannel action to hand a downloaded APK to the system installer.

**Tech Stack:** Flutter/Dart, GitHub Releases API, `dart:io`, `package_info_plus`, `url_launcher`, Android Kotlin MethodChannel, Android FileProvider.

---

### Task 1: Update Service

**Files:**
- Create: `apps/flutter_wallet/lib/src/app_update.dart`
- Create: `apps/flutter_wallet/test/app_update_test.dart`

- [ ] Write failing tests for semantic version comparison, release JSON parsing, platform asset selection, and SHA-256 digest verification.
- [ ] Implement `AppUpdateService`, `GitHubAppUpdateService`, `AppUpdateInfo`, `AppUpdateAsset`, `DownloadedUpdate`, and `AppVersion`.
- [ ] Run `flutter test test/app_update_test.dart` and verify it passes.

### Task 2: Settings UI

**Files:**
- Modify: `apps/flutter_wallet/lib/main.dart`
- Modify: `apps/flutter_wallet/lib/src/screens/settings_screen.dart`
- Modify: `apps/flutter_wallet/test/wallet_workflow_test.dart`

- [ ] Inject `AppUpdateService` through `WalletApp` and `SettingsScreen`.
- [ ] Add an `App update` section with current status and platform action buttons.
- [ ] Add widget tests for update available, no update, platform fallback, and update errors.
- [ ] Run the targeted widget tests and verify they pass.

### Task 3: Android Installer

**Files:**
- Modify: `apps/flutter_wallet/android/app/src/main/AndroidManifest.xml`
- Modify: `apps/flutter_wallet/android/app/build.gradle.kts`
- Modify: `apps/flutter_wallet/android/app/src/main/kotlin/app/localwallet/flutter_wallet/MainActivity.kt`
- Create: `apps/flutter_wallet/android/app/src/main/res/xml/update_file_paths.xml`

- [ ] Add `INTERNET` and `REQUEST_INSTALL_PACKAGES`.
- [ ] Add a FileProvider for downloaded APK files.
- [ ] Add `app.localwallet/app_update` MethodChannel method `installApk`.
- [ ] Run Android compile through Flutter analysis/tests where available.

### Task 4: Dependencies And Verification

**Files:**
- Modify: `apps/flutter_wallet/pubspec.yaml`
- Modify: `apps/flutter_wallet/pubspec.lock`

- [ ] Add `package_info_plus`, `url_launcher`, and `crypto`.
- [ ] Run `flutter test`.
- [ ] Run `flutter analyze`.
- [ ] Run Rust tests if native files changed are unrelated to Rust core only if needed.
