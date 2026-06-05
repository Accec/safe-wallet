# Safe Wallet

Privacy-focused Flutter shell for the Rust-backed local wallet.

Run checks from this directory:

```sh
flutter test
flutter analyze
```

Run the macOS debug app:

```sh
tool/run_macos_debug.sh
```

On macOS 26, directly launching the ad-hoc signed Flutter debug bundle from
`build/macos/...` can stall while the system evaluates `App.framework`. The
debug run script builds the app, copies it to `/tmp/flutter_wallet_debug`, then
launches that copy.

The macOS target builds and bundles `libwallet_ffi.dylib` through the Xcode
build phase in `macos/Runner.xcodeproj`. The iOS target builds and links the
Rust `libwallet_ffi.a` static library through `ios/Runner.xcodeproj`. Android
builds require `cargo-ndk`; `tool/build_wallet_ffi.sh android` prepares
`armeabi-v7a`, `arm64-v8a`, and `x86_64` native libraries by default.

Android Rust targets:

```sh
cargo install cargo-ndk
rustup target add armv7-linux-androideabi aarch64-linux-android x86_64-linux-android
```

iOS Rust targets:

```sh
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
```
