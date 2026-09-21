# TODO

- [ ] Add explicit expectations for independent effects that may arrive in either order. Start with `CounterPair::init`: accept one left database count request and one right database count request, preserve both envelopes, and allow the caller to choose response order. Check the required requests and their number. Keep ordinary `process_effect()` and `expect_*` strict: they must not skip unexpected effects or search the stream.
- [ ] Before adding a task capability, decide who owns spawned work, how cancellation works, and whether that work can outlive its initiating command. Define completion for that scope. This is separate from effect routing.

The current work is described in [the effect routing implementation plan](docs/effect-routing-plan.md).
