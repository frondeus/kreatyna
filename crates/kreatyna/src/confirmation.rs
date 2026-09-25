//! A single decision. The parent owns presentation and the confirmed action.
use krucyfiks::{Command, CommandExt, Message, Widget};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationEvent {
    Confirm,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationOutcome {
    Confirmed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationView {
    AwaitingDecision,
    Resolved { decision: ConfirmationOutcome },
}

/// Create a new instance for each confirmation; resolved instances cannot reopen.
pub struct Confirmation {
    state: ConfirmationView,
}

impl Confirmation {
    pub fn new() -> Self {
        Self {
            state: ConfirmationView::AwaitingDecision,
        }
    }
}

impl Default for Confirmation {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Confirmation {
    type View = ConfirmationView;
    type Event = ConfirmationEvent;
    type Outcome = ConfirmationOutcome;

    fn init(&mut self) -> Command<Self::Outcome> {
        Command::empty()
    }

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        let Message::Event(event) = message else {
            return Command::empty();
        };
        if !matches!(self.state, ConfirmationView::AwaitingDecision) {
            return Command::empty();
        }
        let decision = match event {
            ConfirmationEvent::Confirm => ConfirmationOutcome::Confirmed,
            ConfirmationEvent::Cancel => ConfirmationOutcome::Cancelled,
        };
        self.state = ConfirmationView::Resolved { decision };
        Command::single(async move { decision })
    }

    fn view(&self) -> Self::View {
        self.state
    }
}
