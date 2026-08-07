---
id: res.skill-absorption
nodes: [cairn.root]
date: 2026-08-07
sources: [src.mattpocock-skills, src.context-engineering-claude5]
---

# Skill absorption: what mattpocock/skills teaches cairn's native workflows

Maintainer direction 2026-08-07: do not bolt third-party skills onto cairn;
take what works and make it native, keeping the declarative and probabilistic
split. This research maps each technique to its cairn adoption point. Three
verdicts: **absorb** (rewrite natively at a named seam), **converged** (cairn
already does this; note the validation and steal any sharper wording), and
**decline** (does not fit the substrate).

## Absorb: into the driver's workflow routes (Q4 seam)

Q4 made workflows inert typed artefacts: match predicate, harness route with
context (skills, briefing), limits, outcome routing. The route briefs are
where these disciplines become declarative context the harness executes
probabilistically, which is exactly the split the maintainer named.

1. **Gated phases with named completion criteria** (`diagnosing-bugs`). Its
   phase 1 rule is the transferable core: no hypothesis until a named,
   already-run, red-capable command exists; the loop is tight (fast,
   deterministic, sharp) before anything else happens. Adoption: the retry
   route. When `cairn ruling retry` approves an attempt on a Q3
   claim-failed-verification outcome, the route brief demands a red-capable
   reproduction command as its first completion criterion, quoting the
   failing verification evidence from the outcome fact. A retry without a red
   loop is the exact waste the maintainer is paying for.
2. **Repair discipline for quarantine release** (`resolving-merge-conflicts`
   by role; file unread, the seam stands on its own). Rung 3's repair path is
   merge-conflict outcome, quarantine, `cairn ruling release`. The release
   route brief carries the conflict-resolution discipline and the conflicting
   paths from the outcome fact.
3. **Red-green-refactor as the implement-class brief** (`tdd`).
   `cairn-loop-implement` already mandates covering changed behaviour; the
   sharper form is the ordered gate: failing test shown before the change,
   pass shown after. Adoption: wording in the implement route brief, not new
   machinery.

Owner: `todo.driver-in-repo` task 2 authors `wf.default`; this research is
its input, linked from that todo.

## Absorb: into the agent-surface prune (`todo.context-engineering-pass`)

4. **The `writing-for-agents` framework**, whole. Context pointers whose
   wording does the triggering; the two loads (context load on the model,
   cognitive load on the human, spent deliberately); steps versus reference on
   an information hierarchy, progressive disclosure protecting the top;
   completion criteria with premature completion as the named failure;
   leading words over restatement; positive phrasing over negation (a
   prohibition earns its place only as a hard guardrail, then paired with the
   positive target); pruning by single-source-of-truth, cache, sediment, and
   no-op tests, where the no-op test is model-relative and settled by running
   the document. This is the method the pass todo was missing; it is now
   named there. Two immediate applications the pass must make: AGENTS.md's
   guardrails section is negation-heavy and each line either earns hard-
   guardrail status or gets a positive restatement; and the Terminology
   section is exactly a ubiquitous-language cache and should be kept, tight,
   under that justification.
5. **ASD-STE100 Simplified Technical English as a named register**
   (`wait-what`). Cairn's plain-language convention (5th-grade pass, plain
   register) gains a citable standard and a leading word: STE. Candidate
   addition to `docs/agent/voice.md` at the pass, not before.

## Absorb: into maintainer communication (the regime's presentation forms)

6. **Re-pitch on demand** (`wait-what`). When the maintainer signals a
   message did not land, re-pitch: brief context, plain register, repo
   vocabulary, no new information. Costless convention, effective
   immediately; recorded here as the named behaviour.
7. **The questionnaire form for genuine forks** (`to-questionnaire`). The
   panel regime's four pre-hoc classes still reach the maintainer; when one
   does, it goes as a discovery questionnaire: purpose and the decision
   riding on it, one-paragraph context, questions most-important-first, one
   idea per question, an answer stub under each, a why-this-matters line only
   where misreading is likely, and a closing catch-all. Grill the send, not
   the subject: the agent asks itself who answers and what must come back,
   then writes to that gap. This upgrades the forced-choice sentence for the
   rare case where the fork has several coupled questions.

## Converged: cairn already holds the position

- **Shared language file** (`CONTEXT.md` in grill-with-docs): cairn's
  blueprint, contracts, and AGENTS.md Terminology section are that file,
  reconciled rather than free-floating. Validation, not adoption.
- **Handoff hygiene** (`handoff`): reference artefacts by path, never restate;
  cairn's point-dont-restate doctrine and `.cairn/session-handoff.md` already
  rule this. Its redaction rule is worth carrying into any future handoff
  surface.
- **Grilling** (`grill-me`, `grill-with-docs`): cairn's orchestration grill
  and convergence-minutes practice predate this and go further (ratification
  provisos, recorded rulings). The panel regime deliberately reduces grill
  frequency; no adoption.
- **Phase boundaries with typed exits**: the cairn-dev loop skills already
  declare typed exits per step, which is the completion-criterion discipline
  under another name. Validation.

## Declined

- Installing the skills themselves, under either installer. The shipped pack
  is a binding surface with an eval-hardened promotion path
  (`res.cairn-oax-skill-promotion` lineage); third-party prose does not enter
  it on vibes, and techniques above are absorbed as cairn-native text
  instead.
- Issue-tracker integrations (`to-tickets`, `triage` labels): cairn's intake
  is `cairn feedback` promoting to native todos (`dec.native-todos-first`);
  a second tracker vocabulary would fork the front door.

## Absorption hygiene

Anything absorbed from an outside source is screened before it lands, so
foreign voice never bleeds into how the agent speaks or writes here. The
screen is `scripts/check-voice-markers.sh` over the changed files:
em-dashes fail (the accepted repo ban), and en-dashes, curly quotes,
ellipses, no-break spaces, and the AI-marker lexicon the script carries
(filler verbs and phrase-form tells) warn for a human look. This
session's absorbed artefacts screened at zero hits. The rule is standing:
quote sources verbatim only inside their own source artefact, and rewrite
everything else in the repo voice (docs/agent/voice.md, plain register,
no em-dashes, British spelling).

## Consequences

- `todo.context-engineering-pass` now names the absorbed method (amended this
  session).
- `todo.driver-in-repo` gains a `related:` edge to this research; its task 2
  route briefs consume sections 1 to 3.
- The questionnaire and re-pitch forms bind the agent side of sessions from
  now on; they cost no code.
