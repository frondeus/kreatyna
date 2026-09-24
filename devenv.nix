{ lib, config,  pkgs, ... }:

let
  iosSetup = ''
    set -euo pipefail
    cd ${lib.escapeShellArg config.devenv.root}

    profile="''${1:-debug}"
    case "$profile" in
      debug) profile_args=() ;;
      release) profile_args=(--release) ;;
      *) echo "Usage: $0 [debug|release]" >&2; exit 2 ;;
    esac
    if [ "$#" -gt 1 ]; then
      echo "Usage: $0 [debug|release]" >&2
      exit 2
    fi

    # Nix's DEVELOPER_DIR/SDKROOT refer to its macOS SDK, not the iOS SDK.
    # Override per command without changing the system's xcode-select setting.
    export DEVELOPER_DIR="''${IOS_DEVELOPER_DIR:-/Applications/Xcode.app/Contents/Developer}"
    export CARGO_TARGET_DIR="$PWD/target"

    # Host build scripts, proc macros, and bindgen also need Xcode's linker:
    # Nix's older linker cannot read the macOS 27 SDK's arm64e.x1 stubs.
    host_target="$(rustc -vV | sed -n 's/^host: //p')"
    host_target_env="$(echo "$host_target" | tr '[:lower:]-' '[:upper:]_')"
    host_clang="$(/usr/bin/xcrun --sdk macosx --find clang)"
    export "CARGO_TARGET_''${host_target_env}_LINKER=$host_clang"

    # Keep the default aligned with the Xcode app's deployment target.
    export IPHONEOS_DEPLOYMENT_TARGET="''${IPHONEOS_DEPLOYMENT_TARGET:-26.2}"
  '';

  iosBuild = target: sdk: iosSetup + ''
    export SDKROOT="$(/usr/bin/xcrun --sdk ${sdk} --show-sdk-path)"
    ios_clang="$(/usr/bin/xcrun --sdk ${sdk} --find clang)"
    ios_ar="$(/usr/bin/xcrun --sdk ${sdk} --find ar)"

    # Use the selected iOS SDK's SQLite instead of Nix's macOS SQLite.
    # Target-specific tools leave host build scripts and proc macros on macOS.
    env \
      CARGO_TARGET_${lib.toUpper (builtins.replaceStrings [ "-" ] [ "_" ] target)}_LINKER="$ios_clang" \
      CC_${builtins.replaceStrings [ "-" ] [ "_" ] target}="$ios_clang" \
      AR_${builtins.replaceStrings [ "-" ] [ "_" ] target}="$ios_ar" \
      SQLITE3_LIB_DIR="$SDKROOT/usr/lib" \
      cargo build --locked -p kreatyna-ffi --lib \
        --target ${target} "''${profile_args[@]}"

    echo "Built $CARGO_TARGET_DIR/${target}/$profile/libkreatyna_ffi.a"
  '';
in
{
  # https://devenv.sh/languages/rust/
  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [
      "aarch64-apple-ios"
      "aarch64-apple-ios-sim"
    ];
  };

  packages = [
    pkgs.diesel-cli
    pkgs.sqlite
    pkgs.pkg-config
  ];

  env.DATABASE_URL =
    "${config.devenv.root}/crates/kreatyna-ffi/counter-dev.sqlite";

  scripts.ios-build-device = {
    description = "Build the Rust library for iPhone/iPad: ios-build-device [debug|release]";
    exec = iosBuild "aarch64-apple-ios" "iphoneos";
  };

  scripts.ios-build-simulator = {
    description = "Build the Rust library for Apple Silicon Simulator: ios-build-simulator [debug|release]";
    exec = iosBuild "aarch64-apple-ios-sim" "iphonesimulator";
  };

  scripts.ios-bindings = {
    description = "Build both iOS libraries and generate Swift bindings: ios-bindings [debug|release]";
    exec = iosSetup + ''
      ios-build-device "$profile"
      ios-build-simulator "$profile"

      # The generator runs on macOS and reads metadata from each iOS archive.
      export SDKROOT="$(/usr/bin/xcrun --sdk macosx --show-sdk-path)"
      for target in aarch64-apple-ios aarch64-apple-ios-sim; do
        output="$CARGO_TARGET_DIR/generated/ios/$target/$profile"
        cargo run --locked -p kreatyna-ffi --features bindgen --bin uniffi-bindgen -- \
          generate "$CARGO_TARGET_DIR/$target/$profile/libkreatyna_ffi.a" \
          --language swift --out-dir "$output"
        echo "Generated Swift bindings in $output"
      done
    '';
  };
}
