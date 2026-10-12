#!/usr/bin/env bash
# Build the Rust core and the Kreatyna Mac app, then open its window.
set -euo pipefail

usage() {
  cat >&2 <<'USAGE'
Usage: run-mac-app.sh [--skip-rust] [--seed FILE.json]

  --skip-rust       Reuse the existing Rust library and Swift bindings in target/.
  --seed FILE.json  Open a new isolated database, and let Rust save the
                    ingredients in FILE.json into it first (E-17). Without it,
                    the app opens your real database.
USAGE
  exit 2
}

skip_rust=0
seed_file=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    --skip-rust) skip_rust=1 ;;
    --seed) [ "$#" -ge 2 ] || usage; seed_file="$2"; shift ;;
    *) usage ;;
  esac
  shift
done

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$root"
scripts="$root/.agents/skills/run-macos-app/scripts"

developer_dir="${DEVELOPER_DIR:-/Applications/Xcode.app/Contents/Developer}"
derived_data="$root/target/xcode/DerivedData"
log_dir="$root/target/xcode"
mkdir -p "$log_dir"

# Xcode needs nothing from the Nix shell. A minimal environment keeps Nix or
# stale compiler variables (LD, CC, SDKROOT, NIX_LDFLAGS) out of its link step.
clean_env=(env -i HOME="$HOME" USER="$USER" PATH=/usr/bin:/bin:/usr/sbin:/sbin
  DEVELOPER_DIR="$developer_dir")

# Rust checks the content when the app starts. Check the form here, before
# the build, so a broken file fails fast.
if [ -n "$seed_file" ]; then
  if [ ! -f "$seed_file" ]; then
    echo "Seed file not found: $seed_file" >&2
    exit 1
  fi
  if ! /usr/bin/jq -e 'type == "array"' "$seed_file" >/dev/null 2>&1; then
    echo "Seed file is not a JSON array: $seed_file" >&2
    exit 1
  fi
fi

if [ "$skip_rust" -eq 0 ]; then
  if ! command -v mac-bindings >/dev/null; then
    echo "mac-bindings not on PATH: run inside the devenv shell, or pass --skip-rust." >&2
    exit 1
  fi
  echo "==> Building Rust core and Swift bindings"
  mac-bindings debug
fi

# The Xcode project is generated from apple/Kreatyna/project.yml (MAC-10).
if ! command -v apple-generate >/dev/null; then
  echo "apple-generate not on PATH: run inside the devenv shell." >&2
  exit 1
fi
echo "==> Generating the Xcode project"
apple-generate

echo "==> Building KreatynaMac (log: $log_dir/mac-build.log)"
if ! "${clean_env[@]}" xcodebuild \
    -project apple/Kreatyna/Kreatyna.xcodeproj -scheme KreatynaMac \
    -configuration Debug -destination "platform=macOS,arch=arm64" \
    -derivedDataPath "$derived_data" build >"$log_dir/mac-build.log" 2>&1; then
  grep -E "error:|BUILD FAILED" "$log_dir/mac-build.log" >&2 || tail -40 "$log_dir/mac-build.log" >&2
  exit 1
fi

app="$derived_data/Build/Products/Debug/Kreatyna.app"
# iOS simulator apps have no Contents/MacOS in their path, so this never
# matches them.
process_pattern="Kreatyna\.app/Contents/MacOS/Kreatyna$"

if pgrep -f "$process_pattern" >/dev/null; then
  echo "==> Quitting the running Kreatyna"
  pkill -f "$process_pattern" || true
  for _ in $(seq 1 50); do
    pgrep -f "$process_pattern" >/dev/null || break
    sleep 0.1
  done
fi

open_args=()
if [ -n "$seed_file" ]; then
  open_args+=(--env "KREATYNA_TEST_DATABASE_ID=$(uuidgen)"
              --env "KREATYNA_TEST_SEED=$(cat "$seed_file")")
  database="isolated, seeded from $(basename "$seed_file") ($(/usr/bin/jq length "$seed_file") items)"
else
  database="real (~/Library/Containers/pl.frondeus.Kreatyna/Data/Library/Application Support/Kreatyna/kreatyna.sqlite)"
fi
echo "==> Opening $app"
open "${open_args[@]}" "$app"

pid=""
window=""
for _ in $(seq 1 30); do
  pid="$(pgrep -f "$process_pattern" | head -1 || true)"
  if [ -n "$pid" ]; then
    window="$("${clean_env[@]}" swift "$scripts/window-info.swift" "$pid" 2>/dev/null | head -1 || true)"
    [ -n "$window" ] && break
  fi
  sleep 0.5
done

if [ -z "$window" ]; then
  echo "Kreatyna has no window after 15 seconds (pid: ${pid:-none}). Its log:" >&2
  /usr/bin/log show --predicate 'subsystem == "pl.frondeus.Kreatyna"' --last 1m --style compact >&2 || true
  exit 1
fi

echo "==> Kreatyna is running (pid $pid) on macOS $(sw_vers -productVersion)"
echo "    window: $window"
echo "    database: $database"
