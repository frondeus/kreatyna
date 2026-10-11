---
name: run-ios-simulator
description: Build the Kreatyna iOS app with the real Rust core and open it in the iOS Simulator for the user to see. Use when asked to run, open, launch, or show the app in a simulator, not for running the test suites.
---

# Run the iOS app in a simulator

Run the helper from the repository root inside the devenv shell:

```sh
.agents/skills/run-ios-simulator/scripts/run-in-simulator.sh
```

The script performs these steps:

1. It runs `ios-bindings debug` to build the Rust static library and generate the Swift bindings
   in `target/`. Xcode links these prebuilt files and has no cargo build
   phase, so skipping this step runs stale Rust. Pass `--skip-rust` only when
   nothing in `crates/` changed since the last build.
2. It runs `apple-generate`, which generates `Kreatyna.xcodeproj` from
   `apple/Kreatyna/project.yml`. The generated project is not in version
   control, so this step is never skipped.
3. It opens DeviceHub. Xcode 27 has no `Simulator.app`; its replacement is
   `/Applications/Xcode.app/Contents/Applications/DeviceHub.app`.
4. It targets the device the user sees: every booted iOS simulator. If none is booted,
   it waits for the device that DeviceHub boots. Do not boot a different device
   yourself. The user would not see it, and DeviceHub keeps its own default.
   Use `--udid <UDID>` only when the user names a device.
5. It builds with `xcodebuild` in a minimal environment, using DerivedData under
   `target/xcode/`. The user's Xcode build folder stays untouched.
6. It installs the app and launches it with `--terminate-running-process`, so the app
   comes to the foreground. Then it saves a screenshot for each device to
   `target/xcode/launch-<UDID>.png`.

Read each screenshot before reporting success. "Starting…" is the brief state
while the Rust core loads; take another screenshot with
`xcrun simctl io <UDID> screenshot <path>` before you treat it as stuck. If it shows the home screen
instead of the app, check that the process is alive with
`pgrep -lf "<UDID>.*Kreatyna.app"`. If it is alive, relaunch with
`xcrun simctl launch --terminate-running-process <UDID> pl.frondeus.Kreatyna`.
If it is not, look for a crash log in `~/Library/Logs/DiagnosticReports`.

## Troubleshooting

- A link error `-objc_abi_version '-Xlinker' not supported` or the warning
  `-rpath missing <path>` means `xcodebuild` saw Nix compiler variables such as `LD=ld`,
  `NIX_LDFLAGS`, or a Nix `SDKROOT`. The script's minimal environment prevents
  this. If you run `xcodebuild` by hand, run it the same way. A shell that exports these
  variables comes from an older `devenv.nix`. The current file does not set them,
  so tell the user to run `direnv reload`. Do not edit `devenv.nix` to work around it.
- `ios-bindings` or `apple-generate` is missing from PATH when you are not in the devenv shell.
  Run the command through `devenv shell -- <command>`.
- Build output is kept in `target/xcode/build.log`.

Report to the user which device and iOS version the app is running on.
