---
name: document-ios-delivery
description: Produce reviewable simulator evidence for a completed iOS app feature, including an annotated demo, screenshots, optional videos, and relevant test results. Use for Swift-side UI deliveries, not Rust-only or planning changes.
---

# Document an iOS delivery

Use the feature's acceptance criteria to choose scenarios. Keep the project's
Swift testing agreement in `AGENTS.md`; this skill describes how to show the
finished behavior to a reviewer.

Run the integrated app in an iOS Simulator and capture its actual screens. Use
controlled data and a stated simulator device and iOS version so a reviewer can
understand and reproduce the result. When a failure state needs a fixture or
injected response, label that setup and exercise the normal UI and event path.
Do not present a static mockup as a running-app capture.

Deliver a `demo.md` at the location agreed for the feature, with relative links
to committed simulator screenshots. For each scenario, state the starting
condition, action, and visible result. Cover the main flows and each relevant
user-visible edge state from the acceptance criteria, such as empty, loading,
validation, failure/retry, pending, cancellation, and persistence when applicable.
Use short simulator videos when motion or a multi-step interaction is clearer
than still images; videos supplement the screenshots.

Run the relevant Swift Testing, deterministic snapshot, and real-core XCTest UI
checks required by the project. Inspect the captured images and any snapshot
diffs before accepting them. Record the checks and any unavailable evidence in
the demo or delivery report; never imply that an unrun check passed. Verify that
the Markdown opens every committed image or video before presenting the change
for review.
