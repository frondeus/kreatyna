use futures::StreamExt;
use krucyfiks::{Command, Message, Scoped, Widget};

use super::*;

// Green path

#[test]
fn accept_takes_values_for_the_current_scope_only() {
    let child = ScopedChild::new(Counter);
    let foreign = CommandScopeId::from(u64::MAX);

    assert_eq!(child.accept(Scoped::new(child.scope(), 7)), Some(7));
    assert_eq!(child.accept(Scoped::new(foreign, 7)), None);
}

#[test]
fn replace_gives_a_new_scope_and_rejects_the_old_one() {
    let mut child = ScopedChild::new(Counter);
    let old = child.scope();

    child.replace(Counter);

    assert_ne!(child.scope(), old);
    assert_eq!(child.accept(Scoped::new(old, 7)), None);
}

#[test]
fn view_carries_the_current_scope() {
    let child = ScopedChild::new(Counter);

    assert_eq!(
        child.view(Counter::view),
        Scoped::new(child.scope(), "counter")
    );
}

#[tokio::test]
async fn forward_updates_the_child_and_scopes_its_outcome() {
    let mut child = ScopedChild::new(Counter);
    let scope = child.scope();

    let outcomes: Vec<_> = child
        .forward(Scoped::new(scope, 1))
        .into_stream()
        .collect()
        .await;

    assert_eq!(outcomes, [Scoped::new(scope, 2)]);
}

#[tokio::test]
async fn replace_cancels_the_old_childs_commands() {
    let mut child = ScopedChild::new(Counter);
    let pending = child.command(Command::single(std::future::pending::<u32>()));

    child.replace(Counter);

    assert_eq!(pending.into_stream().next().await, None);
}

// Validation

#[tokio::test]
async fn forward_ignores_an_event_for_an_old_scope() {
    let mut child = ScopedChild::new(Counter);
    let old = child.scope();
    child.replace(Counter);

    let mut command = child.forward(Scoped::new(old, 1)).into_stream();

    assert_eq!(command.next().await, None);
}

// Test support

/// Answers each event with the next number.
struct Counter;

impl Widget for Counter {
    type View = &'static str;
    type Event = u32;
    type Outcome = u32;

    fn update(&mut self, message: Message<Self>) -> Command<u32> {
        match message {
            Message::Event(value) => Command::single(async move { value + 1 }),
            Message::Outcome(_) => Command::empty(),
        }
    }

    fn view(&self) -> &'static str {
        "counter"
    }
}
