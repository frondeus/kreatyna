use super::*;
use framework::effects::Effect;
use futures::{executor::block_on, future::join};

#[test]
fn three_arguments_round_trip_through_exact_and_recursive_routes() {
    block_on(async {
        for recursive in [false, true] {
            let (client, inbox) = StoreClient::new(32, 1);
            let (result, ()) = join(
                client.put("item-key".into(), "item-value".into(), 42),
                async {
                    let effect = StoreEffect::Store(inbox.recv_async().await.unwrap());
                    let handle = |key, value, revision| async move {
                        // Distinct values of the same type also catch argument swaps.
                        assert_eq!(key, "item-key");
                        assert_eq!(value, "item-value");
                        assert_eq!(revision, 42);
                        Ok("stored".into())
                    };
                    if recursive {
                        AppEffect::Right(effect)
                            .any_route()
                            .resolve_put(handle)
                            .await
                            .unwrap();
                    } else {
                        AppEffect::Left(effect)
                            .expect_left()
                            .expect_store()
                            .resolve_put(handle)
                            .await
                            .unwrap();
                    }
                },
            )
            .await;
            assert_eq!(result.unwrap(), "stored");
        }
    });
}

#[derive(Default)]
struct Backend {
    writes: Vec<(String, String, u64)>,
    notifications: Vec<(String, u64)>,
}

#[framework::__private::async_trait]
impl StoreMut for Backend {
    async fn put(
        &mut self,
        key: String,
        value: String,
        revision: u64,
    ) -> Result<String, TransportError> {
        self.writes.push((key, value, revision));
        Ok("saved by backend".into())
    }

    async fn notify(&mut self, key: String, revision: u64) {
        self.notifications.push((key, revision));
    }
}

#[test]
fn multiple_arguments_reach_mutable_backend_dispatch_in_order() {
    block_on(async {
        let (client, inbox) = StoreClient::new(32, 1);
        let mut backend = Backend::default();
        let (result, ()) = join(
            client.put("backend-key".into(), "backend-value".into(), 73),
            async {
                let effect = AppEffect::Left(StoreEffect::Store(inbox.recv_async().await.unwrap()));
                ResolveStore::resolve_mut(effect.any_route(), &mut backend)
                    .await
                    .unwrap();
            },
        )
        .await;
        assert_eq!(result.unwrap(), "saved by backend");
        assert_eq!(
            backend.writes,
            [("backend-key".into(), "backend-value".into(), 73)]
        );

        // Explicitly match a unit response variant after two-argument dispatch.
        let response = StoreRequest::Notify {
            key: "backend-key".into(),
            revision: 73,
        }
        .dispatch_mut(&mut backend)
        .await;
        assert!(matches!(response, StoreResponse::Notify));
        assert_eq!(backend.notifications, [("backend-key".into(), 73)]);
    });
}

#[test]
fn two_argument_unit_method_completes_through_the_tree() {
    block_on(async {
        let (client, inbox) = StoreClient::new(32, 1);
        join(client.notify("notify-key".into(), 9), async {
            AppEffect::Right(StoreEffect::Store(inbox.recv_async().await.unwrap()))
                .expect_right()
                .expect_store()
                .resolve_notify(|key, revision| async move {
                    assert_eq!(key, "notify-key");
                    assert_eq!(revision, 9);
                })
                .await
                .unwrap();
        })
        .await;
    });
}
