---
name: krucyfiks-tag-command-outcomes
description: Tag and check the outcomes of krucyfiks commands with the framework's own wrapper types (Scoped<T> for a child's lifetime, Latest<T> for the newest operation) instead of copying ids into outcome fields. Use when a widget starts a command whose result can arrive late, when you add an outcome variant, or before you write any id, generation, or "is this result still current" pattern in a krucyfiks app.
---

# Tag command outcomes with krucyfiks types

A command's result can arrive after the widget has moved on: the user saved
again, closed the form, or opened another row. The widget must drop such a
late result. krucyfiks already has a wrapper type for each case. Use it, and
never copy an id into your own outcome fields.

## First: search krucyfiks

Before you write an id, a generation counter, a token, or an "accepts" check,
search the framework. The answer is usually already there.

```sh
rg -n 'pub struct|pub fn|pub trait' <krucyfiks>/core/src
```

If the framework has no type for the pattern, do not copy the pattern into
the app. Propose the type for krucyfiks, as `Latest<T>` was added (vibetyna
review of REC-02, 2026-11-01).

## Which wrapper

| The result belongs to | Wrapper | Made by | Checked by |
|---|---|---|---|
| One child's lifetime: a pane, a form, a detail | `Scoped<T> { scope, value }` | `command.scoped(&mut CommandScope)` | `CommandScope::accepts(scoped.scope)`, or a `ScopedChild::accept(scoped) -> Option<T>` |
| The newest run of one operation: a save, a load, a confirmation | `Latest<T> { id, value }` | `command.latest(&mut LatestCommandSlot)` | `LatestCommandSlot::accept(latest) -> Option<T>` |

A new command in a `LatestCommandSlot` cancels the previous one, and `accept`
returns `None` for the old one's results. `stop()` makes the slot reject every
result that is still queued.

## Outcome variants

Keep the wrapper in the outcome. Do not unpack it into fields.

```rust
// Good
pub enum RecipeEditorOutcome {
    ConfirmationDecided(Latest<ConfirmationOutcome>),
    SaveFinished(Latest<Result<Recipe, RecipeSaveError>>),
    Saved { recipe: Recipe },
    Discarded,
}

// Start: the variant's constructor is the map function.
.latest(&mut self.save)
.map(RecipeEditorOutcome::SaveFinished)

// Handle: accept unwraps the value only for the newest command.
Message::Outcome(RecipeEditorOutcome::SaveFinished(finished)) => {
    let Some(result) = self.save.accept(finished) else {
        return Command::empty();
    };
    self.save.stop();
    // ...
}
```

```rust
// Bad: the id is copied into a field, and the check is separate from the
// value, so a handler can forget it.
SaveFinished { id: CommandId, result: Result<Recipe, RecipeSaveError> },
// ...
if !self.save.accepts(id) { return Command::empty(); }
```

`Latest::map` and `Scoped::map` change the value and keep the tag, for
example to wrap a child's outcome in the parent's type.

## Checklist

- Every outcome that a command produces later is wrapped in `Scoped<T>` or
  `Latest<T>`, or the widget can show why a late value is harmless.
- No outcome has an `id: CommandId` or `scope: CommandScopeId` field next to
  its value.
- Every handler unwraps with `accept` before it uses the value.
- Tests dispatch a result of an earlier command and check that it is ignored.
