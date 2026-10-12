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

  # Xcode needs nothing from the Nix shell. A minimal environment keeps Nix
  # compiler variables (LD, CC, SDKROOT, NIX_LDFLAGS) out of its link step.
  xcodeEnv = ''
    clean_env=(env -i HOME="$HOME" USER="$USER" PATH=/usr/bin:/bin:/usr/sbin:/sbin
      DEVELOPER_DIR="$DEVELOPER_DIR")
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
    pkgs.xcodegen
  ];

  env.DATABASE_URL =
    "${config.devenv.root}/crates/kreatyna-ffi/kreatyna-dev.sqlite";

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

      # The generator runs on macOS and reads metadata from an iOS archive.
      # Bindings are identical for every Apple target and profile, so the
      # Xcode project uses one copy for all of them.
      export SDKROOT="$(/usr/bin/xcrun --sdk macosx --show-sdk-path)"
      output="$CARGO_TARGET_DIR/generated/apple/swift"
      cargo run --locked -p kreatyna-ffi --features bindgen --bin uniffi-bindgen -- \
        generate "$CARGO_TARGET_DIR/aarch64-apple-ios-sim/$profile/libkreatyna_ffi.a" \
        --language swift --out-dir "$output"
      echo "Generated Swift bindings in $output"
    '';
  };

  scripts.apple-generate = {
    description = "Generate the Xcode project from apple/Kreatyna/project.yml";
    exec = ''
      set -euo pipefail
      cd ${lib.escapeShellArg config.devenv.root}
      xcodegen generate --spec apple/Kreatyna/project.yml --quiet
    '';
  };

  scripts.ios-test = {
    description = "Generate the Xcode project and run all iOS tests: ios-test [simulator-udid]";
    exec = ''
      set -euo pipefail
      cd ${lib.escapeShellArg config.devenv.root}
      apple-generate
      ${xcodeEnv}

      udid="''${1:-}"
      if [ -z "$udid" ]; then
        udid="$("''${clean_env[@]}" xcrun simctl list devices booted |
          awk '/^-- /{ios = ($2 == "iOS")} ios && /\(Booted\)/ {
            match($0, /\([0-9A-F-]{36}\)/); print substr($0, RSTART + 1, RLENGTH - 2); exit
          }')"
      fi
      if [ -z "$udid" ]; then
        echo "No booted iOS simulator. Boot one or pass its UDID: ios-test <udid>" >&2
        exit 1
      fi

      mkdir -p target/xcode
      echo "==> Testing on $udid (log: target/xcode/test.log)"
      if ! "''${clean_env[@]}" xcodebuild test \
          -project apple/Kreatyna/Kreatyna.xcodeproj -scheme Kreatyna \
          -destination "id=$udid" -derivedDataPath target/xcode/DerivedData \
          >target/xcode/test.log 2>&1; then
        grep -E "error:|failed|TEST FAILED" target/xcode/test.log >&2 || tail -40 target/xcode/test.log >&2
        exit 1
      fi
      grep -E "Test run with|TEST SUCCEEDED" target/xcode/test.log || true
    '';
  };

  scripts.mac-test = {
    description = "Generate the Xcode project and run all macOS tests: mac-test";
    exec = ''
      set -euo pipefail
      cd ${lib.escapeShellArg config.devenv.root}
      apple-generate
      ${xcodeEnv}

      # The UI tests need Automation Mode without a prompt (MAC-00/Q1):
      # sudo automationmodetool enable-automationmode-without-authentication
      mkdir -p target/xcode
      echo "==> Testing on this Mac (log: target/xcode/mac-test.log)"
      if ! "''${clean_env[@]}" xcodebuild test \
          -project apple/Kreatyna/Kreatyna.xcodeproj -scheme KreatynaMac \
          -destination "platform=macOS,arch=arm64" -derivedDataPath target/xcode/DerivedData \
          >target/xcode/mac-test.log 2>&1; then
        grep -E "error:|failed|TEST FAILED" target/xcode/mac-test.log >&2 || tail -40 target/xcode/mac-test.log >&2
        exit 1
      fi
      grep -E "Test run with|TEST SUCCEEDED" target/xcode/mac-test.log || true
    '';
  };
}
