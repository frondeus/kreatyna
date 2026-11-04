---
name: use-plannotator-inbox
description: Talk to the user through the Plannotator Inbox. Use for every question, plan, review request, report, and blocker in this project, instead of chat.
---

# Use the Plannotator Inbox

All communication with the user goes through the Plannotator Inbox. The user
reads and answers there. Do not put questions, plans, or reports in the chat.
A chat message is only for a short line that points to the Inbox message.

## How to reach it

1. If the session has the `plannotator_inbox` tool
   (`mcp__plannotator__plannotator_inbox`, maybe through tool search), use it.
   Its `action` is one of the tool names below. Replies wake the session.
2. If not, run the helper from the repository root:

   ```sh
   python3 -I .agents/skills/use-plannotator-inbox/scripts/inbox.py send_message message.json
   ```

   `message.json` holds the tool arguments as a JSON object (`-` reads
   stdin). Write it in the scratchpad. The helper runs
   `plannotator inbox mcp` and prints the result. A reply may not wake the
   session. After you send, end the turn. When the user returns, read the
   reply with `read_thread`.

`plannotator inbox --help` is the full guide. Read it when a tool refuses a
call.

| Tool | Use |
|---|---|
| `send_message` | `body` (Markdown), `subject`, `thread`, `reply_to`, `attachments` (paths). |
| `read_thread` | Read a thread with the user's answers and annotations. |
| `wait_for_reply` | Wait at most 50 seconds for a reply. Only for short waits. |
| `resolve_message` | Close a thread when you have what you need. |
| `record_decision`, `list_decisions` | Project decisions. Read them before you ask again. |

## Threads

Use one named thread for each unit of work, so work stays apart:
`thread: "MAC-02"` for a slice, `thread: "mac-shell epic"` for epic planning.
Answer a reply with `reply_to` set to its message id.

## Questions

Ask with question blocks. They render as answer cards. Each block needs
enough context to answer without opening anything else: what the choice
changes, what you know, and why you recommend an option.

```markdown
:::question
MAC-02/Q2 — Which database does a run use?

The **+** button stays disabled until MAC-05, so the real database is empty.
A seed shows rows, but not your data.

- [ ] Real database only — empty window until MAC-05
- [ ] Real database, plus `--seed FILE` — your data by default, rows on request

Recommended: Real database, plus `--seed FILE`
:::
```

- Keep the question ID (`MAC-02/Q2`) at the start of the first line.
- Add `Stopped: <why>` when work cannot go on without the answer.
- Ask at most about 8 questions in one message.

## Plans, documents, and mockups

Attach the file (`attachments: ["docs/epics/mac-shell/tasks/MAC-02.md"]`) and
link it from the body. The user annotates it in the Inbox. The annotations
come back in the reply.

## Code review: count the Rust files first

The user reviews the Rust side and the demos. The user does not review Swift.
Before you ask for any review (when a slice is finished, and after each round
of review fixes), count the Rust files that the change edits:

```sh
jj diff -r @ --name-only 'glob:**/*.rs'
```

### No Rust files: no review

Do not ask for a review. Do not open `plannotator review`, do not send a
guided review, and do not ask "Does the app behave as the demo shows?".
Send one short Inbox report: what changed, the checks and their results, and
"No Rust files changed, so there is no review." Include the demo in that
report when there is one, for information only. The change does not wait for a review; continue
with the next work.

### Rust files: a guided review in the Inbox

Send a guided review to the slice's Inbox thread. Do not open the full code
review (`plannotator review`) unless the user asks for it.

1. Send the demo first, if the slice has one, as its own message: the text
   in the body, each screenshot embedded with `![...](path)` and listed in
   `attachments`, no `demo.md` file (see
   [document-apple-delivery](../document-apple-delivery/SKILL.md)). End it
   with the question "Does the app behave as the demo shows?".
2. Complete the checklist in [review-own-diff](../review-own-diff/SKILL.md)
   on the patch, and fix what it finds. This is required before every guided
   review, also after a round of review fixes.
3. Call `get_guide_brief` and follow its method.
4. Make the patch from the Rust side only:

   ```sh
   jj diff --git -r @ 'glob:"**/*.rs" | glob:"**/Cargo.toml" | glob:"**/migrations/**"'
   ```

   Add a Swift file only when it holds an important part that is more than
   presentation: a platform adapter that executes an effect for Rust, the
   event or effect bridge to the core, or business logic that Swift must not
   own. Add it to the fileset by path (`... | "apple/Kreatyna/Shared/X.swift"`,
   inside the same single quotes), and say in its section why it is in the
   review. Never add views, view modifiers, snapshot baselines, UI
   tests, or `project.yml` only for layout.
5. Call `submit_guide` with the guide and that exact patch, in the same
   thread. Every file in the patch goes in exactly one section. The body ends
   with the self-review line from review-own-diff.

The user's comments come back as an Inbox reply. Handle them with
[handle-review-comments](../handle-review-comments/SKILL.md).

## Reports

A report that is not a code review request (a blocker, a check run, a
finding) is an Inbox message. Keep it short: what changed, the checks and
their results, what stays open. Attach the plan when the user
must read it.

## Reply handling

- An answer that takes your recommendation shows `(your recommendation)`.
  Treat it like a **Looks good** label: the user accepts it.
- `Skipped` keeps the question open.
- A note or `Other:` text overrides the offered options.
