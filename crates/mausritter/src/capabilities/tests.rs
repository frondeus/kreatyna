use std::cell::Cell;

use futures::{FutureExt, executor::block_on, future::join};

use super::*;
use crate::composability::CounterPairEffect;

#[test]
fn wrong_exact_branch_preserves_the_effect_and_responder() {
    block_on(async {
        let (envelope, response) = Envelope::new(RenderRequest::Render, 1);
        let effect = CounterPairEffect::Right(CounterEffect::Render(envelope));
        let effect = effect.try_into_left().unwrap_err();
        let effect = effect.expect_right().try_into_database().unwrap_err();
        effect
            .expect_render()
            .resolve_render(|| async {})
            .await
            .unwrap();
        assert!(matches!(response.try_recv(), Ok(RenderResponse::Render)));
    });
}

#[test]
#[should_panic(expected = "expected CounterPairEffect::Left, got CounterPairEffect::Right")]
fn exact_branch_assertion_does_not_search_for_a_matching_capability() {
    let (envelope, _response) = Envelope::new(RenderRequest::Render, 1);
    CounterPairEffect::Right(CounterEffect::Render(envelope)).expect_left();
}

#[test]
fn recursive_extraction_preserves_the_route_on_mismatch_and_only_returns_leaves() {
    let (envelope, response) = Envelope::new(DatabaseRequest::SetCount { count: 19 }, 1);
    let effect = CounterPairEffect::Left(CounterEffect::Database(envelope));
    let effect = effect.extract::<CounterEffect>().unwrap_err();
    let effect = effect
        .extract::<Envelope<RenderRequest, RenderResponse>>()
        .unwrap_err();
    let envelope = effect.expect_left().expect_database();
    assert!(matches!(
        envelope.request,
        DatabaseRequest::SetCount { count: 19 }
    ));
    envelope.answer(DatabaseResponse::SetCount(Ok(()))).unwrap();
    assert!(matches!(
        response.try_recv(),
        Ok(DatabaseResponse::SetCount(Ok(())))
    ));
}

#[test]
fn recursive_resolution_accepts_both_origins_and_an_explicit_subtree() {
    block_on(async {
        for left in [true, false] {
            let (envelope, response) = Envelope::new(RenderRequest::Render, 1);
            let effect = if left {
                CounterPairEffect::Left(CounterEffect::Render(envelope))
            } else {
                CounterPairEffect::Right(CounterEffect::Render(envelope))
            };
            effect
                .any_route()
                .resolve_render(|| async {})
                .await
                .unwrap();
            assert!(matches!(response.try_recv(), Ok(RenderResponse::Render)));
        }

        let (envelope, response) = Envelope::new(DatabaseRequest::Count, 1);
        CounterPairEffect::Left(CounterEffect::Database(envelope))
            .expect_left()
            .any_route()
            .resolve_count(|| async { Ok(Some(23)) })
            .await
            .unwrap();
        assert!(matches!(
            response.try_recv(),
            Ok(DatabaseResponse::Count(Ok(Some(23))))
        ));
    });
}

#[test]
fn capability_mismatches_skip_callbacks_and_close_the_original_response_channel() {
    block_on(async {
        let (envelope, response) = Envelope::new(DatabaseRequest::Count, 1);
        let result = CounterEffect::Database(envelope)
            .any_route()
            .resolve_render(|| {
                panic!("render callback must not run");
                #[allow(unreachable_code)]
                async {}
            })
            .await;
        assert!(matches!(result, Err(EnvelopeError::Mismatch)));
        assert!(response.is_disconnected());

        let (envelope, response) = Envelope::new(RenderRequest::Render, 1);
        let result = CounterEffect::Render(envelope)
            .any_route()
            .resolve_count(|| {
                panic!("database callback must not run");
                #[allow(unreachable_code)]
                async {
                    Ok(None)
                }
            })
            .await;
        assert!(matches!(result, Err(EnvelopeError::Mismatch)));
        assert!(response.is_disconnected());

        let (envelope, response) = Envelope::new(RenderRequest::Render, 1);
        let result = CounterEffect::Render(envelope)
            .any_route()
            .resolve_sleep_ms(|_| {
                panic!("clock callback must not run");
                #[allow(unreachable_code)]
                async {}
            })
            .await;
        assert!(matches!(result, Err(EnvelopeError::Mismatch)));
        assert!(response.is_disconnected());
    });
}

#[test]
fn method_mismatches_skip_callbacks_for_exact_and_recursive_resolvers() {
    block_on(async {
        let (envelope, response) = Envelope::new(DatabaseRequest::SetCount { count: 7 }, 1);
        let result = envelope
            .resolve_count(|| {
                panic!("count callback must not run for set_count");
                #[allow(unreachable_code)]
                async {
                    Ok(None)
                }
            })
            .await;
        assert!(matches!(result, Err(EnvelopeError::Mismatch)));
        assert!(response.is_disconnected());

        let (envelope, response) = Envelope::new(DatabaseRequest::Count, 1);
        let result = CounterPairEffect::Right(CounterEffect::Database(envelope))
            .any_route()
            .resolve_set_count(|_| {
                panic!("set_count callback must not run for count");
                #[allow(unreachable_code)]
                async {
                    Ok(())
                }
            })
            .await;
        assert!(matches!(result, Err(EnvelopeError::Mismatch)));
        assert!(response.is_disconnected());
    });
}

// Cell makes both the tree and backend Send but not Sync. The wrapper deliberately
// has no Debug implementation, either.
struct LocalTree<E> {
    effect: E,
    marker: Cell<()>,
}

impl<E: EffectTree> EffectTree for LocalTree<E> {
    fn extract<T: 'static>(self) -> Result<T, Self> {
        self.effect.extract().map_err(|effect| Self {
            effect,
            marker: self.marker,
        })
    }
}

fn local<E>(effect: E) -> LocalTree<E> {
    LocalTree {
        effect,
        marker: Cell::new(()),
    }
}

#[derive(Default)]
struct Backend {
    renders: Cell<usize>,
    slept_ms: Cell<u64>,
    count: Cell<Option<usize>>,
}

#[async_trait::async_trait]
impl RenderMut for Backend {
    async fn render(&mut self) {
        self.renders.set(self.renders.get() + 1);
    }
}

#[async_trait::async_trait]
impl ClockMut for Backend {
    async fn sleep_ms(&mut self, ms: u64) {
        self.slept_ms.set(self.slept_ms.get() + ms);
    }
}

#[async_trait::async_trait]
impl DatabaseMut for Backend {
    async fn count(&mut self) -> Result<Option<usize>, DatabaseError> {
        Ok(self.count.get())
    }

    async fn set_count(&mut self, count: usize) -> Result<(), DatabaseError> {
        self.count.set(Some(count));
        Ok(())
    }
}

#[test]
fn mutable_dispatch_accepts_non_sync_trees_and_trait_object_backends() {
    block_on(async {
        let mut backend = Backend::default();
        let (envelope, response) = Envelope::new(RenderRequest::Render, 1);
        ResolveRender::resolve_mut(
            local(envelope).any_route(),
            &mut backend as &mut dyn RenderMut,
        )
        .await
        .unwrap();
        assert_eq!(backend.renders.get(), 1);
        assert!(matches!(response.try_recv(), Ok(RenderResponse::Render)));

        let (envelope, response) = Envelope::new(ClockRequest::SleepMs { ms: 123 }, 1);
        ResolveClock::resolve_mut(
            local(envelope).any_route(),
            &mut backend as &mut dyn ClockMut,
        )
        .await
        .unwrap();
        assert_eq!(backend.slept_ms.get(), 123);
        assert!(matches!(response.try_recv(), Ok(ClockResponse::SleepMs)));

        let (envelope, response) = Envelope::new(DatabaseRequest::SetCount { count: 37 }, 1);
        ResolveDatabase::resolve_mut(
            local(envelope).any_route(),
            &mut backend as &mut dyn DatabaseMut,
        )
        .await
        .unwrap();
        assert_eq!(backend.count.get(), Some(37));
        assert!(matches!(
            response.try_recv(),
            Ok(DatabaseResponse::SetCount(Ok(())))
        ));

        let (envelope, response) = Envelope::new(DatabaseRequest::Count, 1);
        ResolveDatabase::resolve_mut(
            local(envelope).any_route(),
            &mut backend as &mut dyn DatabaseMut,
        )
        .await
        .unwrap();
        assert!(matches!(
            response.try_recv(),
            Ok(DatabaseResponse::Count(Ok(Some(37))))
        ));
    });
}

#[test]
fn mismatched_mutable_resolvers_do_not_run_backends() {
    block_on(async {
        let mut backend = Backend::default();
        let (envelope, response) = Envelope::new(DatabaseRequest::Count, 1);
        assert!(matches!(
            ResolveRender::resolve_mut(CounterEffect::Database(envelope).any_route(), &mut backend)
                .await,
            Err(EnvelopeError::Mismatch)
        ));
        assert_eq!(backend.renders.get(), 0);
        assert!(response.is_disconnected());

        let (envelope, response) = Envelope::new(RenderRequest::Render, 1);
        assert!(matches!(
            ResolveClock::resolve_mut(CounterEffect::Render(envelope).any_route(), &mut backend)
                .await,
            Err(EnvelopeError::Mismatch)
        ));
        assert_eq!(backend.slept_ms.get(), 0);
        assert!(response.is_disconnected());
    });
}

#[test]
fn clock_client_round_trips_arguments_through_exact_and_recursive_resolvers() {
    block_on(async {
        for recursive in [false, true] {
            let (client, inbox) = ClockClient::new(1);
            let (_, ()) = join(client.sleep_ms(456), async {
                let envelope = inbox.recv_async().await.unwrap();
                let handle = |ms| async move { assert_eq!(ms, 456) };
                if recursive {
                    CounterPairEffect::Right(CounterEffect::Clock(envelope))
                        .any_route()
                        .resolve_sleep_ms(handle)
                        .await
                        .unwrap();
                } else {
                    envelope.resolve_sleep_ms(handle).await.unwrap();
                }
            })
            .await;
        }
    });
}

#[test]
fn database_clients_propagate_transport_failures() {
    block_on(async {
        let (client, inbox) = DatabaseClient::new(1);
        drop(inbox);
        assert!(matches!(
            client.count().await,
            Err(DatabaseError::Transport)
        ));
        assert!(matches!(
            client.set_count(7).await,
            Err(DatabaseError::Transport)
        ));

        let (client, inbox) = DatabaseClient::new(1);
        let (result, ()) = join(client.count(), async {
            drop(inbox.recv_async().await.unwrap());
        })
        .await;
        assert!(matches!(result, Err(DatabaseError::Transport)));
        let (result, ()) = join(client.set_count(7), async {
            drop(inbox.recv_async().await.unwrap());
        })
        .await;
        assert!(matches!(result, Err(DatabaseError::Transport)));
    });
}

#[test]
fn database_clients_reject_wrong_response_variants() {
    block_on(async {
        let (client, inbox) = DatabaseClient::new(1);
        let (result, ()) = join(client.count(), async {
            inbox
                .recv_async()
                .await
                .unwrap()
                .answer(DatabaseResponse::SetCount(Ok(())))
                .unwrap();
        })
        .await;
        assert!(matches!(result, Err(DatabaseError::Transport)));
        let (result, ()) = join(client.set_count(7), async {
            inbox
                .recv_async()
                .await
                .unwrap()
                .answer(DatabaseResponse::Count(Ok(Some(7))))
                .unwrap();
        })
        .await;
        assert!(matches!(result, Err(DatabaseError::Transport)));
    });
}

#[test]
fn database_clients_preserve_successes_and_application_errors() {
    block_on(async {
        let (client, inbox) = DatabaseClient::new(1);
        let (result, ()) = join(client.count(), async {
            inbox
                .recv_async()
                .await
                .unwrap()
                .resolve_count(|| async { Ok(Some(11)) })
                .await
                .unwrap();
        })
        .await;
        assert!(matches!(result, Ok(Some(11))));
        let (result, ()) = join(client.count(), async {
            inbox
                .recv_async()
                .await
                .unwrap()
                .resolve_count(|| async { Err(DatabaseError::Storage) })
                .await
                .unwrap();
        })
        .await;
        assert!(matches!(result, Err(DatabaseError::Storage)));
        let (result, ()) = join(client.set_count(usize::MAX), async {
            inbox
                .recv_async()
                .await
                .unwrap()
                .resolve_set_count(|count| async move {
                    assert_eq!(count, usize::MAX);
                    Err(DatabaseError::OutOfRange)
                })
                .await
                .unwrap();
        })
        .await;
        assert!(matches!(result, Err(DatabaseError::OutOfRange)));
    });
}

#[test]
#[should_panic(expected = "Render::render transport failed: request channel closed")]
fn render_explicitly_panics_on_request_transport_failure() {
    let (client, inbox) = RenderClient::new(1);
    drop(inbox);
    block_on(client.render());
}

#[test]
#[should_panic(expected = "Render::render transport failed: response channel closed")]
fn render_explicitly_panics_when_its_responder_is_dropped() {
    let (client, inbox) = RenderClient::new(1);
    block_on(join(client.render(), async {
        drop(inbox.recv_async().await.unwrap());
    }));
}

#[test]
#[should_panic(expected = "Clock::sleep_ms transport failed: request channel closed")]
fn clock_explicitly_panics_on_request_transport_failure() {
    let (client, inbox) = ClockClient::new(1);
    drop(inbox);
    block_on(client.sleep_ms(7));
}

#[test]
#[should_panic(expected = "Clock::sleep_ms transport failed: response channel closed")]
fn clock_explicitly_panics_when_its_responder_is_dropped() {
    let (client, inbox) = ClockClient::new(1);
    block_on(join(client.sleep_ms(7), async {
        drop(inbox.recv_async().await.unwrap());
    }));
}

#[test]
fn cancelled_request_returns_the_unsent_response_to_the_resolver() {
    block_on(async {
        let (client, inbox) = ClockClient::new(1);
        let mut request = Box::pin(client.sleep_ms(8));
        assert!(request.as_mut().now_or_never().is_none());
        let envelope = inbox.try_recv().unwrap();
        drop(request);
        let result = envelope
            .resolve_sleep_ms(|ms| async move { assert_eq!(ms, 8) })
            .await;
        assert!(matches!(result, Err(EnvelopeError::Send(_))));
    });
}
