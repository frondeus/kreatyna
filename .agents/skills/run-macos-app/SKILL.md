---
name: run-macos-app
description: Build the Kreatyna Mac app with the real Rust core and open its window for the user, with the real database or an isolated database seeded from a JSON file. Use when asked to run, open, launch, or show the Mac app, not for running the test suites.
---

# Run the Mac app

Run the helper inside the devenv shell. The devenv script `mac-run` calls it
with the same options:

```sh
devenv shell -- mac-run
devenv shell -- mac-run --seed apple/Kreatyna/Fixtures/demo-ingredients.json
```

The script is `.agents/skills/run-macos-app/scripts/run-mac-app.sh`.

The script performs these steps:

1. With `--seed FILE.json`, it checks that the file is a JSON array. Rust
   checks the content when the app starts.
2. It runs `mac-bindings debug` to build the Rust static library and
   generate the Swift bindings in `target/`. Xcode links these prebuilt files
   and has no cargo build phase, so skipping this step runs stale Rust. Pass
   `--skip-rust` only when nothing in `crates/` changed since the last build.
3. It runs `apple-generate`, which generates `Kreatyna.xcodeproj` from
   `apple/Kreatyna/project.yml`. This step is never skipped.
4. It builds the `KreatynaMac` scheme in Debug with `xcodebuild` in a minimal
   environment, using DerivedData under `target/xcode/` (shared with
   `mac-test`).
5. It quits a running Mac copy of Kreatyna, so the new build opens. It
   matches `Kreatyna.app/Contents/MacOS/Kreatyna`, so iOS simulator apps
   keep running.
6. It opens the app with `open`. With `--seed`, it adds
   `KREATYNA_TEST_DATABASE_ID` (a new UUID) and `KREATYNA_TEST_SEED` (the
   file's text). The app then uses a new database in its temporary folder
   (E-18), and Rust saves the ingredients into it before the catalog loads
   (E-17).
7. It waits at most 15 seconds for an on-screen window, then prints the
   process ID, the macOS version, the window size, and the database.

## What you can and cannot see

The agent's shell has no Screen Recording permission, so the script takes no
screenshot (MAC-02/Q1). A printed window size proves the window is on screen,
not what it shows. To see the content, use the UI tests or a capture test
(see [document-apple-delivery](../document-apple-delivery/SKILL.md)). Tell
the user that the window is open and which database it uses.

## Seed files

A seed file is a JSON array. An item is a name, or an object with the
fields of the ingredient form. Numbers are JSON numbers. `basis` is `"100g"`
(default) or `"100ml"`. Unknown fields are an error.

```json
[
  "Cucumber",
  {
    "name": "Garlic",
    "notes": "Peel the cloves.",
    "calories": 149, "protein": 6.4, "carbohydrates": 33, "fat": 0.5,
    "units": [{ "name": "clove", "grams": 5 }]
  }
]
```

`apple/Kreatyna/Fixtures/demo-ingredients.json` has 5 ingredients with full
data. A Rust test seeds it, so it stays valid.

## Troubleshooting

- The window says "Test data could not be added.": Rust refused the seed.
  Read its reason:
  `log show --predicate 'subsystem == "pl.frondeus.Kreatyna" AND category == "seed"' --last 5m`.
- No window after 15 seconds: the script prints the app's log from the last
  minute. Also look for a crash report in `~/Library/Logs/DiagnosticReports`.
- The build fails: the log is `target/xcode/mac-build.log`. Link errors that
  name Nix paths mean `xcodebuild` saw Nix compiler variables; run it the way
  the script does.
- `mac-bindings` or `apple-generate` is missing from PATH outside the devenv
  shell. Run `devenv shell -- mac-run`.
