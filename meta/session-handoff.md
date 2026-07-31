# Where things stand: 2026-07-31

Read this top to bottom and you can carry on. Nothing else needs opening.

## To start the next session, paste this

> Read meta/session-handoff.md and carry on. Explain things in plain words,
> and never send me to a file to make a decision: bring me the question,
> the options, and what happens either way.

## Two things are waiting on George

### 1. A yes or no on the test-fixture rule

**What it is.** Cairn ships a small fake project used by its own tests. Its
files sit in two piles. One pile (sources and research notes) is deliberately
left out of cairn's checker. The other pile (decisions, contracts, todos) is
checked. The split has to exist: if the ignored pile were checked, cairn would
complain that research notes cite sources it cannot see, and the tests would
fail.

**The question.** Make that split an official rule, or leave it an informal
choice nobody wrote down?

**History.** George already said yes once. A review then found the write-up
claimed something false: it said every part of the fake project has a decision
explaining it, when the top-level part has none and cairn only requires them
for the smallest parts. The wording was fixed. Because the text changed after
he approved it, his yes no longer covers it.

**Either way.** Yes: it becomes a rule, and later work cites it instead of
working out the reasoning again. No: nothing breaks, the arrangement stays,
just unofficial. Nothing is blocked on this and no warning depends on it.

### 2. A go or drop on the decision-count warning

**What it is.** Cairn warns when one part of the codebase has collected more
than ten decisions, on the theory that a pile of rules in one place is hard to
read. Two parts are at twelve, so two warnings show.

**The proposal.** Let bigger parts carry more before warning: the whole
project twenty, mid-sized parts fifteen, small parts ten as now.

**The correction.** George approved this when told it would clear both
warnings. That was wrong, and the mistake was mine: I had not checked how the
parts are classified. It clears one. The other stays, because that part is
classified as small even though it has grown large.

**Either way.** Go: one warning clears, one stays. Drop: both stay. Either
way nothing is blocked, because these warnings never stop work.

## What was done on 2026-07-31

Eight pull requests, seven pieces of work, all merged and green.

The big one was the approval-tiers feature. Before it, every decision in cairn
needed George's signature, however small. Now a decision can be marked
"local", meaning narrow enough for the agent to approve on its own, but only
with real evidence: two independent reviews, each tied by a fingerprint to the
exact text and files being approved. If a single byte of any of it changes,
the reviews stop counting and it needs fresh ones. Anything touching the parts
every adopting project inherits stays George-only, permanently.

The review process found thirteen ways to slip something past that gate, all
of them fixed with a test that fails if the hole reopens.

Then the feature was pointed at the first real decision, and it blocked it:
two reviews independently caught the false claim described above. That is item
1 in the waiting list.

The rest: the parked-deferral decision approved and its todo closed; the
portfolio-hygiene todo written; the driver-v2 change proposed; the
release-milestone todo landed.

## Other work now queued

- **todo.pending-queue-briefing**: the approval queue lists decisions by name
  only, so ruling on one means opening a file. It should show the question and
  the options in place. George raised this on 2026-07-31.
- **todo.ratification-candidate-pointer**: a bug found just after the tiers
  feature merged. The commit check looks for decisions in a fixed folder
  instead of the folder each project configures, so a project that moved that
  folder gets a check that silently sees nothing.
- **todo.portfolio-hygiene**: thirty-two open todos predate the current
  mission statement. Sweep each one: keep, rewrite, or close.
- **driver-v2-selection**: proposed, not started. Replaces the hand-written
  work queue with the repository's own answers about what to do next.
- **todo.release-next-milestone**: cut a release when the checklist in it is
  satisfied. It is not satisfied yet, because item 1 above is still open.

## State of the repository

Clean, parked at `origin/main` (`a7ed6f5`). All checks pass locally and in CI.
Eight warnings show, all of the harmless kind that never block work: two
research notes not linked from a decision, three sources that cannot be
verified because they are conversations, one deferred spec rule, and the two
decision-count warnings from item 2.
