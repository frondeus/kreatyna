//! A child widget together with the scope that owns its commands.
use krucyfiks::{Command, CommandExt, CommandScope, CommandScopeId, Message, Scoped, Widget};

/// A child and its [`CommandScope`]. Replacing the child replaces the scope,
/// which cancels the old child's commands. Events and outcomes reach the child
/// only through [`ScopedChild::accept`] or [`ScopedChild::forward`], so a
/// value addressed to an old scope cannot reach the new child.
pub(crate) struct ScopedChild<W> {
    pub(crate) widget: W,
    scope: CommandScope,
}

impl<W> ScopedChild<W> {
    pub(crate) fn new(widget: W) -> Self {
        Self {
            widget,
            scope: CommandScope::new(),
        }
    }

    pub(crate) fn replace(&mut self, widget: W) {
        *self = Self::new(widget);
    }

    /// The value, if it was addressed to this child's current scope.
    pub(crate) fn accept<T>(&self, scoped: Scoped<T>) -> Option<T> {
        self.scope.accepts(scoped.scope).then_some(scoped.value)
    }

    /// Ties a command to this child's scope.
    pub(crate) fn command<O: Send + 'static>(&mut self, command: Command<O>) -> Command<Scoped<O>> {
        command.scoped(&mut self.scope)
    }

    /// A view of the child, with the scope that its events must carry.
    pub(crate) fn view<V>(&self, view: impl FnOnce(&W) -> V) -> Scoped<V> {
        Scoped::new(self.scope.id(), view(&self.widget))
    }

    pub(crate) fn scope(&self) -> CommandScopeId {
        self.scope.id()
    }
}

impl<W: Widget> ScopedChild<W> {
    /// Checks the scope, updates the child, and scopes its command.
    pub(crate) fn forward(&mut self, event: Scoped<W::Event>) -> Command<Scoped<W::Outcome>> {
        match self.accept(event) {
            Some(event) => {
                let command = self.widget.update(Message::Event(event));
                self.command(command)
            }
            None => Command::empty(),
        }
    }
}

#[cfg(test)]
mod tests;
