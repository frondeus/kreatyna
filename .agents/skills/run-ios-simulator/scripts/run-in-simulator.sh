#!/usr/bin/env bash
# Build the Rust core and the Kreatyna app, then install and launch it on the
# iOS Simulator the user is looking at (every booted iOS simulator by default).
set -euo pipefail

usage() {
  cat >&2 <<'EOF'
Usage: run-in-simulator.sh [--skip-rust] [--udid UDID]...

  --skip-rust   Reuse the existing Rust library and Swift bindings in target/.
  --udid UDID   Target this simulator (repeatable). Default: all booted iOS
                simulators; if none is booted, open DeviceHub and use the
                device it boots.
EOF
  exit 2
}

skip_rust=0
udids=()
while [ "$#" -gt 0 ]; do
  case "$1" in
    --skip-rust) skip_rust=1 ;;
    --udid) [ "$#" -ge 2 ] || usage; udids+=("$2"); shift ;;
    *) usage ;;
  esac
  shift
done

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$root"

developer_dir="${DEVELOPER_DIR:-/Applications/Xcode.app/Contents/Developer}"
derived_data="$root/target/xcode/DerivedData"
log_dir="$root/target/xcode"
mkdir -p "$log_dir"

# Xcode needs nothing from the Nix shell. A minimal environment keeps Nix or
# stale compiler variables (LD, CC, SDKROOT, NIX_LDFLAGS) out of its link step.
clean_env=(env -i HOME="$HOME" USER="$USER" PATH=/usr/bin:/bin:/usr/sbin:/sbin
  DEVELOPER_DIR="$developer_dir")

booted_ios_udids() {
  "${clean_env[@]}" xcrun simctl list devices booted |
    awk '/^-- /{ios = ($2 == "iOS")} ios && /\(Booted\)/ {
      match($0, /\([0-9A-F-]{36}\)/); print substr($0, RSTART + 1, RLENGTH - 2)
    }'
}

open_simulator_app() {
  # Xcode 27 replaced Simulator.app with DeviceHub.app.
  local hub="$developer_dir/../Applications/DeviceHub.app"
  local legacy="$developer_dir/Applications/Simulator.app"
  if [ -d "$hub" ]; then
    open "$hub"
  elif [ -d "$legacy" ]; then
    open "$legacy"
  else
    echo "Neither DeviceHub.app nor Simulator.app found in $developer_dir/.." >&2
    exit 1
  fi
}

if [ "$skip_rust" -eq 0 ]; then
  if ! command -v ios-bindings >/dev/null; then
    echo "ios-bindings not on PATH: run inside the devenv shell, or pass --skip-rust." >&2
    exit 1
  fi
  echo "==> Building Rust core and Swift bindings"
  ios-bindings debug
fi

open_simulator_app
if [ "${#udids[@]}" -eq 0 ]; then
  echo "==> Waiting for a booted iOS simulator"
  for _ in $(seq 1 90); do
    while read -r udid; do udids+=("$udid"); done < <(booted_ios_udids)
    [ "${#udids[@]}" -gt 0 ] && break
    sleep 1
  done
  if [ "${#udids[@]}" -eq 0 ]; then
    echo "No iOS simulator booted. Pick a device in DeviceHub or pass --udid." >&2
    exit 1
  fi
fi

for udid in "${udids[@]}"; do
  "${clean_env[@]}" xcrun simctl boot "$udid" 2>/dev/null || true
  # Installing before SpringBoard is ready leaves the app in the background.
  "${clean_env[@]}" xcrun simctl bootstatus "$udid" -b >/dev/null
done

echo "==> Building Kreatyna (log: $log_dir/build.log)"
if ! "${clean_env[@]}" xcodebuild \
    -project apple/Kreatyna/Kreatyna.xcodeproj -scheme Kreatyna \
    -configuration Debug -destination "id=${udids[0]}" \
    -derivedDataPath "$derived_data" build >"$log_dir/build.log" 2>&1; then
  grep -E "error:|BUILD FAILED" "$log_dir/build.log" >&2 || tail -40 "$log_dir/build.log" >&2
  exit 1
fi

app="$derived_data/Build/Products/Debug-iphonesimulator/Kreatyna.app"
bundle_id="$(/usr/libexec/PlistBuddy -c "Print CFBundleIdentifier" "$app/Info.plist")"

for udid in "${udids[@]}"; do
  name="$("${clean_env[@]}" xcrun simctl list devices | grep "$udid" | sed -E 's/^ *//; s/ \(.*//')"
  echo "==> Launching $bundle_id on $name ($udid)"
  "${clean_env[@]}" xcrun simctl install "$udid" "$app"
  "${clean_env[@]}" xcrun simctl launch --terminate-running-process "$udid" "$bundle_id"
  sleep 4
  shot="$log_dir/launch-$udid.png"
  "${clean_env[@]}" xcrun simctl io "$udid" screenshot "$shot" >/dev/null 2>&1
  echo "    screenshot: $shot"
done
