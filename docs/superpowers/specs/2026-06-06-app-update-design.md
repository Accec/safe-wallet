# App Update Design

## Goal

Safe Wallet should let users check for a newer release from inside the app. When
the current platform can support an app-managed update path, the app should
download the release asset itself instead of sending the user to a browser first.

## Platform Behavior

- Android checks the latest GitHub Release, downloads the matching APK asset,
  verifies the asset digest when GitHub provides one, and starts the Android
  package installer with the downloaded APK. Android still shows the system
  install confirmation UI.
- macOS checks the latest GitHub Release, downloads the matching macOS zip asset,
  verifies the asset digest when available, asks the native Runner to extract the
  zip, replaces the current `.app` bundle, and relaunches the app. A later
  Sparkle integration can replace this with a signed appcast and a more robust
  updater.
- iOS checks the latest GitHub Release and opens the release page when a newer
  version exists. iOS does not download and install executable app updates from
  inside the app.

## Update Source

The app uses the GitHub Releases API for `Accec/safe-wallet` and reads the latest
published release. Tags use the existing release workflow format, such as
`v1.2.3`.

The app chooses platform assets by filename:

- Android: `safe-wallet-android-*.apk`
- macOS: `safe-wallet-macos-*.zip`
- iOS: release page URL

## UI

The Settings screen gets an `App update` section with the current version,
latest checked version, update status, and a `Check for updates` button. If a
new version exists, the user can start the platform action:

- Android: `Download and install`
- macOS: `Download and install`
- iOS: `Open download page`

The UI shows download progress for Android and macOS, and reports clear errors
for network failures, missing assets, digest mismatches, and unsupported
platforms.

## Boundaries

This design does not add background auto-update checks, forced updates, Sparkle
appcasts, code signing automation, or store-specific update mechanisms. Those
can be added later without changing the Settings screen contract.

## Testing

- Version comparison handles equal, older, newer, and `v`-prefixed tags.
- GitHub release parsing selects the correct platform asset.
- Digest verification rejects corrupted downloads when a SHA-256 digest is
  available.
- Settings widget tests cover no-update, update-available, download/install, and
  error states using a fake update service.
