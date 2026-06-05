#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
app_dir="$(cd "$script_dir/.." && pwd)"

cd "$app_dir"

pkill -9 -x flutter_wallet || true
pkill -9 -f '[f]lutter_wallet.app/Contents/MacOS/flutter_wallet' || true
pkill -f '[f]lutter_tools.snapshot run -d macos' || true

exec flutter run -d macos --debug
