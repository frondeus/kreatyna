{ lib, config, pkgs, inputs, ... }:

let
  rustBin = inputs.rust-overlay.lib.mkRustBin { } pkgs.buildPackages;

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

    export CARGO_TARGET_DIR="$PWD/target"

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
  # Use Xcode's toolchain instead of Nix's on macOS. Nix's apple-sdk exports
  # DEVELOPER_DIR/SDKROOT and puts xcbuild's xcrun on PATH, which hides Xcode
  # from xcodebuild. Nix's cc wrapper and cctools ld cannot read the macOS 27
  # SDK's arm64e.x1 stubs, so cc/ld must resolve to Xcode's /usr/bin shims.
  apple.sdk = null;
  stdenv = pkgs.stdenvNoCC;
  languages.c.enable = false;
  # Select Xcode for this shell without changing the system's xcode-select.
  env.DEVELOPER_DIR = "/Applications/Xcode.app/Contents/Developer";

  # https://devenv.sh/languages/rust/
  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [
      "aarch64-apple-ios"
      "aarch64-apple-ios-sim"
    ];
    # rust-overlay propagates Nix's cc wrapper, which would shadow Xcode's
    # cc/ld on PATH. Drop it so cargo links with the Xcode toolchain.
    toolchainPackage =
      (rustBin.stable.latest.minimal.override {
        extensions = [ "clippy" "rustfmt" ];
        inherit (config.languages.rust) targets;
      }).overrideAttrs {
        depsHostHostPropagated = [ ];
        propagatedBuildInputs = [ ];
      };
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
        output="$CARGO_TARGET_DIR/generated/apple/$target/$profile"
        cargo run --locked -p kreatyna-ffi --features bindgen --bin uniffi-bindgen -- \
          generate "$CARGO_TARGET_DIR/$target/$profile/libkreatyna_ffi.a" \
          --language swift --out-dir "$output"
        echo "Generated Swift bindings in $output"
      done
    '';
  };
}
