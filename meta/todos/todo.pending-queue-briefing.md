---
node: cairn.kernel.query
status: done
created: 2026-07-31
---

# The Approval Queue Lists Names, Not Questions

## What the maintainer asked for

Stated 2026-07-31, in his words, and binding on this todo's shape:

- The interface is the chat session, not the file tree. Being sent to a file
  path to make a decision is the failure, whatever that file contains.
- Present decisions in "clear, real, concise, basic vocabulary". Insider
  vocabulary in a question aimed at a human is a defect, not a style choice.
- One interface, no continuity break. A new session should open with where
  things were left, what needs a decision, and the recommended next action,
  without anyone pasting a command or finding a file.

Those three are acceptance criteria, not preferences.

## Problem

`dec.north-star-continuous-loop` goal 5 says the maintainer not knowing what
is waiting on them is a defect state. The same decision requires every queued
decision to carry a rubric: its class, what it unblocks, how it aligns, and
the options with a recommendation. That rubric is a briefing, and it is
already written in every pending decision.

The queue does not read it. Measured 2026-07-31, `cairn pending` prints one
line per decision:

```
dec.bootstrap-fixture-corpus-split (age 1d, local) nodes: cairn.tests
```

and the data behind it carries five values: id, age, nodes, class, and a
content fingerprint. To rule on that row a maintainer must open the file, read
the ruling, work out what changed since they last saw it, and find the
options. The briefing was written and then buried.

Measured cost this session: a decision was approved, review found a false
claim in it, the text was corrected, and the maintainer was sent back to the
file to rule again. Nothing in the queue showed the ruling, the correction, or
the review verdicts that forced it.

## Scope

Make the queue answer "what am I being asked, and what happens either way"
without leaving it.

- Extend the pending data with the rubric sections parsed from the decision
  body, a plain summary of the ruling, and for a self-approvable row, the
  state of its evidence: which reviews exist, who reviewed, and their
  verdicts.
- Add a detail read (`cairn pending <id>`, or a flag matching the command
  grammar) rendering that briefing in full. Keep the one-line list as index.
- Say plainly when a decision's text changed after it was last approved. The
  fingerprint already proves it; nothing new needs computing.
- Provide one command an agent runs at session start that returns the same
  briefing set plus the current state of work, so a fresh session opens with
  the answer instead of a search. This is the continuity half, and it is what
  makes the surface one interface rather than two.
- Web surface (`cairn.ui`): the Pending channel becomes an inbox with a
  detail pane carrying the same briefing, and stays read-only per
  `dec.user-surfaces`, offering the exact commands rather than hidden writes.

## Out of scope

A write surface for approving decisions from the UI. Approval is a committed
change under the evidence protocol; a button that hides that is worse than a
command that shows it.

## Acceptance

- `cairn pending <id>` prints the ruling, the rubric, the evidence state, and
  the next action, with no file read needed to rule.
- A row whose text changed after its last approval says so in the list, not
  only in the detail view.
- One session-start command returns where work was left, what awaits a
  decision, and the recommended next action.
- Every string a human reads passes the plain-language bar in
  `docs/agent/voice.md`: no insider vocabulary in a question aimed at a human.
- The web Pending channel shows the same briefing and stays read-only.

## Origin

Maintainer, 2026-07-31: "why do I have to dig up decisions?", then "my UI
interface right now is this chat", after being sent to a decision file to rule
on a correction the queue could not show. Feeds the over-harness UI thread
(`res.overharness-design-threads`).

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. It is campaign step 5 for briefing the pending signature queue.
