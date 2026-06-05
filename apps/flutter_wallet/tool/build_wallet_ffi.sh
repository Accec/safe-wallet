#!/usr/bin/env bash
set -euo pipefail

platform="${1:-macos}"
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
app_dir="$(cd "$script_dir/.." && pwd)"
repo_root="$(cd "$app_dir/../.." && pwd)"

case "$platform" in
  macos)
    configuration="${CONFIGURATION:-Debug}"
    profile_args=()
    profile_dir="debug"
    if [[ "$configuration" == "Release" || "$configuration" == "Profile" ]]; then
      profile_args=(--release)
      profile_dir="release"
    fi

    cargo build --manifest-path "$repo_root/Cargo.toml" -p wallet_ffi "${profile_args[@]}"
    source_lib="$repo_root/target/$profile_dir/libwallet_ffi.dylib"

    if [[ -n "${BUILT_PRODUCTS_DIR:-}" && -n "${FRAMEWORKS_FOLDER_PATH:-}" ]]; then
      destination_dir="$BUILT_PRODUCTS_DIR/$FRAMEWORKS_FOLDER_PATH"
    else
      destination_dir="$app_dir/build/native/macos"
    fi

	    mkdir -p "$destination_dir"
	    cp "$source_lib" "$destination_dir/libwallet_ffi.dylib"
	    if command -v codesign >/dev/null 2>&1; then
	      sign_identity="${EXPANDED_CODE_SIGN_IDENTITY:-${CODE_SIGN_IDENTITY:--}}"
	      for sign_target in \
	        "$destination_dir/App.framework" \
	        "$destination_dir/FlutterMacOS.framework" \
	        "$destination_dir/objective_c.framework" \
	        "$destination_dir/libwallet_ffi.dylib"; do
	        if [[ -e "$sign_target" ]]; then
	          codesign --force --sign "$sign_identity" "$sign_target"
	        fi
	      done
	    fi
	    ;;
  android)
    if ! command -v cargo-ndk >/dev/null 2>&1; then
      echo "cargo-ndk is required to build Android wallet_ffi native libraries." >&2
      echo "Install it with: cargo install cargo-ndk" >&2
      exit 66
    fi
    android_abis="${ANDROID_ABIS:-armeabi-v7a arm64-v8a x86_64}"
    target_args=()
    for abi in $android_abis; do
      target_args+=(-t "$abi")
    done
    cargo ndk \
      "${target_args[@]}" \
      -o "$app_dir/android/app/src/main/jniLibs" \
      build --manifest-path "$repo_root/Cargo.toml" -p wallet_ffi --release
    ;;
  ios)
    configuration="${CONFIGURATION:-Debug}"
    platform_name="${PLATFORM_NAME:-iphonesimulator}"
    archs="${ARCHS:-arm64}"
    profile_args=()
    profile_dir="debug"
    if [[ "$configuration" == "Release" || "$configuration" == "Profile" ]]; then
      profile_args=(--release)
      profile_dir="release"
    fi

    destination_dir="$app_dir/build/native/ios/$platform_name"
    mkdir -p "$destination_dir"
    built_libraries=()

    for arch in $archs; do
      case "$platform_name:$arch" in
        iphoneos:arm64)
          rust_target="aarch64-apple-ios"
          ;;
        iphonesimulator:arm64)
          rust_target="aarch64-apple-ios-sim"
          ;;
        iphonesimulator:x86_64)
          rust_target="x86_64-apple-ios"
          ;;
        *)
          echo "Unsupported iOS Rust target for platform=$platform_name arch=$arch" >&2
          exit 65
          ;;
      esac

      cargo build \
        --manifest-path "$repo_root/Cargo.toml" \
        -p wallet_ffi \
        --target "$rust_target" \
        "${profile_args[@]}"
      built_libraries+=("$repo_root/target/$rust_target/$profile_dir/libwallet_ffi.a")
    done

    if [[ "${#built_libraries[@]}" -eq 1 ]]; then
      cp "${built_libraries[0]}" "$destination_dir/libwallet_ffi.a"
    else
      lipo -create "${built_libraries[@]}" -output "$destination_dir/libwallet_ffi.a"
    fi
    ;;
  *)
    echo "Unknown platform: $platform" >&2
    exit 64
    ;;
esac
