---
name: handle-review-comments
description: Decide what to do with each comment the user leaves on a plan, a guided review, a demo, or a message in kreatyna. Use whenever a Plannotator reply carries comments or notes, before you change any code for them.
---

# Handle review comments

Introspection 1 (2026-11-04) found four habits that the user had to correct
more than once. This skill replaces them. Read every comment in the reply first,
then handle each one by its kind.

## 1. A question, "discuss", or "think about it": answer first

The user wrote "don't jump immediately to code, discuss that with me", "discuss
it with me especially if it's good idea or not", and "think very hard and
discuss it with me without immediately folding to my call".

- Answer the question, or give your analysis, in the Inbox. Change no code
  until the user decides.
- Give an honest verdict, with evidence from the code. If you disagree,
  say so and give the reason. Do not agree only because the user suggested
  it.
- If there is a real choice, ask it as a `:::question` block, with your
  recommendation on its own line.

A comment that is clearly an instruction ("rename it", "use a newtype here")
needs no discussion. Do it.

## 2. A comment about a pattern: fix every place

A comment on one line is often about a pattern, for example "can we use a
newtype here?" or "don't unwrap in a test". The user asked three times for "a
bead for the other places".

- Search the workspace for the same pattern (`rg`).
- Fix every place in the same change if the change stays under 1,000 lines.
- If it does not, fix this change, and in the same reply file a `tech-debt`
  bead that lists the other places by file and line.
- Say which you did in the reply.

## 3. A rule from the user: write it down in the same change

The user had to ask "make this a skill" six times, once with "it should be
already a skill but somehow you ignored it".

- When a comment states a rule that holds beyond this line, add it to the
  skill that is active when the mistake happens. Do this in the same change
  as the fix. If no skill fits, use `bd remember`.
- Quote the comment and give its date, so that later readers know where the
  rule came from.
- Tell the user in the reply where the rule now lives.

## 4. A tooling problem you cannot explain: search first

The user wrote "Have you tried searching in the internet? This must be known
issue", and "I won't be looking at the clock to see when I should move my
mouse".

- Search the web before you run experiments.
- Never ask the user to do a timed action. Automate the experiment, or run it
  while the user does their own work, and log what you need.
