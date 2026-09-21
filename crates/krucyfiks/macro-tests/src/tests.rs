use framework::{
    Effect,
    effects::{Envelope, TransportError},
};
use futures::{executor::block_on, future::join};

struct Payload(String); // deliberately not Debug or Clone

#[derive(Debug)]
struct Error(TransportError);

impl From<TransportError> for Error {
    fn from(error: TransportError) -> Self {
        Self(error)
    }
}

type Reply = Result<Payload, Error>;

#[framework::capability(crate = "::framework")]
trait Service {
    async fn exchange(&self, request: Payload, response: u8) -> Reply;
    #[krucyfiks(panic_on_transport)]
    async fn ping(&self);
    #[krucyfiks(panic_on_transport)]
    async fn explicit_unit(&self) -> ();
    #[krucyfiks(panic_on_transport)]
    async fn r#type(&self, r#type: usize) -> usize;
    #[krucyfiks(panic_on_transport)]
    async fn application_error(&self) -> Result<(), String>;
}

#[derive(framework::Effect)]
#[krucyfiks(crate = "::framework")]
#[allow(clippy::upper_case_acronyms)] // Exercise acronym-to-snake-case helper names.
enum Tree {
    HTTP(Envelope<ServiceRequest, ServiceResponse>),
    Other(Envelope<Payload, Payload>),
}

#[test]
fn renamed_dependency_round_trips_owned_non_debug_payloads_and_result_aliases() {
    block_on(async {
        let (client, inbox) = ServiceClient::new(32, 1);
        let (result, ()) = join(client.exchange(Payload("hello".into()), 7), async {
            Tree::HTTP(inbox.recv_async().await.unwrap())
                .expect_http()
                .resolve_exchange(|request, response| async move {
                    assert_eq!(response, 7);
                    Ok(Payload(format!("{}!", request.0)))
                })
                .await
                .unwrap();
        })
        .await;
        assert_eq!(result.unwrap().0, "hello!");

        let (result, ()) = join(client.exchange(Payload(String::new()), 0), async {
            drop(inbox.recv_async().await.unwrap());
        })
        .await;
        assert!(matches!(
            result,
            Err(Error(TransportError::ResponseChannelClosed))
        ));
    });
}

#[test]
fn both_unit_signatures_use_unit_variants_and_raw_identifiers_work() {
    block_on(async {
        let (client, inbox) = ServiceClient::new(32, 1);
        join(client.ping(), async {
            inbox
                .recv_async()
                .await
                .unwrap()
                .answer(ServiceResponse::Ping)
                .unwrap();
        })
        .await;
        join(client.explicit_unit(), async {
            inbox
                .recv_async()
                .await
                .unwrap()
                .answer(ServiceResponse::ExplicitUnit)
                .unwrap();
        })
        .await;
        let (value, ()) = join(client.r#type(21), async {
            Tree::HTTP(inbox.recv_async().await.unwrap())
                .any_route()
                .resolve_type(|value| async move { value * 2 })
                .await
                .unwrap();
        })
        .await;
        assert_eq!(value, 42);
    });
}

#[test]
fn panic_policy_preserves_application_errors_without_from_bound() {
    block_on(async {
        let (client, inbox) = ServiceClient::new(32, 1);
        let (result, ()) = join(client.application_error(), async {
            inbox
                .recv_async()
                .await
                .unwrap()
                .resolve_application_error(|| async { Err("business failure".into()) })
                .await
                .unwrap();
        })
        .await;
        assert_eq!(result, Err("business failure".into()));
    });
}

#[test]
fn derive_preserves_non_debug_children_on_a_wrong_branch() {
    let (envelope, response) = Envelope::<Payload, Payload>::new(Payload("request".into()), 1);
    let original = match Tree::Other(envelope).try_into_http() {
        Err(original) => original,
        Ok(_) => panic!("wrong branch accepted"),
    };
    assert!(
        original
            .expect_other()
            .answer(Payload("response".into()))
            .is_ok()
    );
    assert_eq!(response.try_recv().unwrap().0, "response");
}

#[test]
#[should_panic(expected = "Service::application_error transport failed: request channel closed")]
fn panic_policy_on_result_methods_panics_on_transport_failures() {
    let (client, inbox) = ServiceClient::new(32, 1);
    drop(inbox);
    let _ = block_on(client.application_error());
}

#[test]
#[should_panic(expected = "Service::ping transport failed: unexpected response variant")]
fn panic_policy_rejects_wrong_response_variants() {
    let (client, inbox) = ServiceClient::new(32, 1);
    block_on(join(client.ping(), async {
        inbox
            .recv_async()
            .await
            .unwrap()
            .answer(ServiceResponse::ExplicitUnit)
            .unwrap();
    }));
}

mod public_api {
    pub mod facade {
        pub use framework::*;
    }

    #[facade::capability(crate = "crate::tests::public_api::facade")]
    pub trait Notifications {
        #[krucyfiks(panic_on_transport)]
        async fn notify(&self, envelope: String, __krucyfiks_arg_1: usize);
    }

    #[derive(facade::Effect)]
    #[krucyfiks(crate = "crate::tests::public_api::facade")]
    pub(crate) enum Events {
        Notification(facade::effects::Envelope<NotificationsRequest, NotificationsResponse>),
    }
}

#[test]
fn public_api_and_reexported_macros_work_across_modules() {
    use public_api::{Events, Notifications, NotificationsClient, ResolveNotifications};
    block_on(async {
        let (client, inbox) = NotificationsClient::new(32, 1);
        let client: Box<dyn Notifications> = Box::new(client);
        join(client.notify("hello".into(), 12), async {
            Events::Notification(inbox.recv_async().await.unwrap())
                .expect_notification()
                .resolve_notify(|text, count| async move {
                    assert_eq!(text, "hello");
                    assert_eq!(count, 12);
                })
                .await
                .unwrap();
        })
        .await;
    });
}

#[test]
fn configured_request_capacity_applies_backpressure() {
    block_on(async {
        for capacity in [1, 3] {
            let (client, inbox) = ServiceClient::new(capacity, 1);
            let mut calls: Vec<_> = (0..=capacity).map(|_| client.ping()).collect();
            for call in &mut calls {
                assert!(futures::poll!(call.as_mut()).is_pending());
            }
            assert_eq!(inbox.len(), capacity);

            let request = inbox.try_recv().unwrap();
            request.answer(ServiceResponse::Ping).unwrap();
            assert!(futures::poll!(calls[0].as_mut()).is_ready());
            assert!(futures::poll!(calls[capacity].as_mut()).is_pending());
            assert_eq!(inbox.len(), capacity);

            for call in &mut calls[1..] {
                inbox
                    .try_recv()
                    .unwrap()
                    .answer(ServiceResponse::Ping)
                    .unwrap();
                assert!(futures::poll!(call.as_mut()).is_ready());
            }
            assert!(inbox.is_empty());
        }
    });
}
