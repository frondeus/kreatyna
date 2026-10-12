#!/usr/bin/env bash
# Run a demo capture UI test and save its screenshots as PNG files.
set -euo pipefail

usage() {
  cat >&2 <<'USAGE'
Usage: capture-demo.sh <mac|ios> <test identifier> <output folder> [--udid UDID]

  mac   Run on this Mac with the KreatynaMac scheme.
  ios   Run on the booted iOS simulator (or --udid) with the Kreatyna scheme.

Example:
  capture-demo.sh mac KreatynaMacUITests/MacDemoCaptureUITests docs/epics/mac-shell/tasks/MAC-01-demo

The script does not build Rust. Run mac-bindings or ios-bindings first.
USAGE
  exit 2
}

[ "$#" -ge 3 ] || usage
platform="$1"
test_id="$2"
output="$3"
shift 3
udid=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    --udid) [ "$#" -ge 2 ] || usage; udid="$2"; shift ;;
    *) usage ;;
  esac
  shift
done

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$root"
mkdir -p "$output"
output="$(cd "$output" && pwd)"

developer_dir="${DEVELOPER_DIR:-/Applications/Xcode.app/Contents/Developer}"
log_dir="$root/target/xcode"
result="$log_dir/demo.xcresult"
attachments="$log_dir/demo-attachments"
mkdir -p "$log_dir"

# Xcode needs nothing from the Nix shell. A minimal environment keeps Nix or
# stale compiler variables (LD, CC, SDKROOT, NIX_LDFLAGS) out of its link step.
clean_env=(env -i HOME="$HOME" USER="$USER" PATH=/usr/bin:/bin:/usr/sbin:/sbin
  DEVELOPER_DIR="$developer_dir")

case "$platform" in
  mac)
    scheme=KreatynaMac
    destination="platform=macOS,arch=arm64"
    ;;
  ios)
    scheme=Kreatyna
    if [ -z "$udid" ]; then
      udid="$("${clean_env[@]}" xcrun simctl list devices booted |
        awk '/^-- /{ios = ($2 == "iOS")} ios && /\(Booted\)/ {
          match($0, /\([0-9A-F-]{36}\)/); print substr($0, RSTART + 1, RLENGTH - 2); exit
        }')"
    fi
    if [ -z "$udid" ]; then
      echo "No booted iOS simulator. Boot one or pass --udid." >&2
      exit 1
    fi
    destination="id=$udid"
    ;;
  *) usage ;;
esac

if ! command -v apple-generate >/dev/null; then
  echo "apple-generate not on PATH: run inside the devenv shell." >&2
  exit 1
fi
apple-generate

rm -rf "$result" "$attachments"
echo "==> Capturing $test_id on $destination (log: $log_dir/demo.log)"
if ! "${clean_env[@]}" TEST_RUNNER_KREATYNA_CAPTURE_DEMO=1 xcodebuild test \
    -project apple/Kreatyna/Kreatyna.xcodeproj -scheme "$scheme" \
    -destination "$destination" -derivedDataPath "$log_dir/DerivedData" \
    -resultBundlePath "$result" -only-testing:"$test_id" \
    >"$log_dir/demo.log" 2>&1; then
  grep -E "error:|failed|TEST FAILED" "$log_dir/demo.log" >&2 || tail -40 "$log_dir/demo.log" >&2
  exit 1
fi
if grep -q "skipped" "$log_dir/demo.log" && ! grep -q "passed" "$log_dir/demo.log"; then
  echo "The capture test was skipped. Is it a capture test?" >&2
  exit 1
fi

"${clean_env[@]}" xcrun xcresulttool export attachments \
  --path "$result" --output-path "$attachments" >/dev/null

# manifest.json lists each test's attachments: the exported file and the
# name the test gave it.
count=0
while IFS=$'\t' read -r file name; do
  # "01-catalog_0_<UUID>.png" → "01-catalog"
  name="${name%.png}"
  name="${name%_*}"
  name="${name%_*}"
  cp "$attachments/$file" "$output/$name.png"
  echo "    $output/$name.png"
  count=$((count + 1))
done < <(/usr/bin/jq -r '.[].attachments[] | [.exportedFileName, .suggestedHumanReadableName] | @tsv' \
  "$attachments/manifest.json")

if [ "$count" -eq 0 ]; then
  echo "The test attached no screenshots." >&2
  exit 1
fi
echo "==> Saved $count screenshots in $output"
