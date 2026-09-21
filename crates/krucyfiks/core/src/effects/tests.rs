use std::rc::Rc;

use super::*;

// Neither Debug nor Send: navigation must not impose either bound.
struct Payload(Rc<u8>);

#[test]
fn command_navigation_preserves_payloads_without_debug_or_send() {
    let payload = Rc::new(7);
    let value: CommandEffect<Payload, Payload> = CommandEffect::Effect(Payload(payload.clone()));
    let value = match value.try_into_outcome() {
        Err(value) => value,
        Ok(_) => panic!("accepted an effect as an outcome"),
    };
    let value = match value.try_into_done() {
        Err(value) => value,
        Ok(_) => panic!("accepted an effect as completion"),
    };
    assert!(Rc::ptr_eq(&value.expect_effect().0, &payload));

    let value: CommandEffect<Payload, Payload> = CommandEffect::Outcome(Payload(payload.clone()));
    let value = match value.try_into_effect() {
        Err(value) => value,
        Ok(_) => panic!("accepted an outcome as an effect"),
    };
    assert!(Rc::ptr_eq(&value.expect_outcome().0, &payload));

    CommandEffect::<Payload, Payload>::Done.expect_done();
}

#[test]
#[should_panic(expected = "expected CommandEffect::Effect, got CommandEffect::Outcome")]
fn effect_assertion_names_variants_without_formatting_payloads() {
    CommandEffect::<Payload, Payload>::Outcome(Payload(Rc::new(1))).expect_effect();
}

#[test]
#[should_panic(expected = "expected CommandEffect::Outcome, got CommandEffect::Done")]
fn outcome_assertion_names_variants_without_debug_bounds() {
    CommandEffect::<Payload, Payload>::Done.expect_outcome();
}

#[test]
#[should_panic(expected = "expected CommandEffect::Done, got CommandEffect::Effect")]
fn completion_assertion_names_variants_without_formatting_payloads() {
    CommandEffect::<Payload, Payload>::Effect(Payload(Rc::new(1))).expect_done();
}

#[test]
fn leaf_extraction_checks_both_types_and_preserves_the_original_responder() {
    let payload = Rc::new(7);
    let (envelope, response) = Envelope::<Payload, u8>::new(Payload(payload.clone()), 1);

    let envelope = match envelope.extract::<Envelope<Payload, u16>>() {
        Err(envelope) => envelope,
        Ok(_) => panic!("accepted the wrong response type"),
    };
    let envelope = match envelope.extract::<Envelope<(), u8>>() {
        Err(envelope) => envelope,
        Ok(_) => panic!("accepted the wrong request type"),
    };
    let envelope = match envelope
        .any_route()
        .into_inner()
        .extract::<Envelope<Payload, u8>>()
    {
        Ok(envelope) => envelope,
        Err(_) => panic!("rejected the exact envelope type"),
    };
    assert!(Rc::ptr_eq(&envelope.request.0, &payload));
    envelope.answer(42).unwrap();
    assert_eq!(response.try_recv().unwrap(), 42);
}
