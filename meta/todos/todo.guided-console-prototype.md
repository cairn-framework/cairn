---
node: cairn.ui
status: open
created: 2026-08-06
related: [todo.console-orchestration-ux-design, todo.console-signed-widening, todo.driver-in-repo, todo.parallel-dispatch-granularity, dec.webui-write-authority]
---

# Guided console prototype: the creation journey, tested as a user

Ordered by the round 2 ruling of `todo.console-orchestration-ux-design`
(prototype first, 2026-08-06): further hypothetical scenario mocks are
deferred in favour of a prototype the maintainer tests as a user against
a demo project, and design then continues against that real feedback.
This unit owns the prototype. The register reference is
`studio/mocks/orchestration-guided-journey.html` (aligned 2026-08-06):
plain language primary, progressive disclosure, one run action, waves as
sentences beside a ghost mini-map, a glyph-strip working drawer.

## Scope: the creation journey only

Four moments, end to end, and nothing else:

1. **Describe.** The maintainer describes UX and outcomes in the
   conversation rail; the description decodes into the blueprint.
2. **The map forms visually.** Ghost structures appear and fill in as
   the description lands: mini-map strata, wave-number badges, waves as
   sentences ("these fill in as it runs").
3. **The grill drains doubts.** Cairn agentic workflows surface doubts
   as plain questions with selectable answers, extrapolating from the
   requirements and stress-testing until the map of what is to be done
   exists visually.
4. **Run.** One action records the run ruling (`cairn ruling run`,
   `dec.webui-write-authority` clause 4); the driver observes it,
   re-reads state, and dispatches the waves; layers unlock as
   dependencies land. Until that decision is signed and the verb
   exists, the run plate stays honestly not wired.

Explicit non-goals, deferred to design-against-feedback: the return and
orient reactive layer beyond what the journey needs, the queue-drain
surfaces, the narrow layout, the driver-states four-up,
decision-to-consequence as its own screen, workflow tuning surfaces, and
universal harness compatibility (prototype first, no deep
hypotheticals).

## Deployment target

The maintainer's own environment, not an abstraction: self-hosted in
Docker on the maintainer's cloud server, long running, driving the
maintainer's own harness. Portability across harnesses is out of scope
by the same ruling.

## Design authority

Design-system tokens and components only (`docs/design-system/`),
product lane per `dec.marketing-visual-world`. The guided journey mock
is the register reference. `dec.webui-write-authority` is accepted
(2026-08-06), so its clause 5 carries those obligations and they bind
this prototype's surfaces. The run verb is sanctioned by its clause 4 but
not yet implementable: the fact store and the plan identity it takes are
rung 3 design (`todo.parallel-dispatch-granularity`), so the run plate
stays not wired and says so on its face.

## Session 1: what exists now (2026-08-06)

The prototype is built and runs the whole creation journey. It lives in
`studio/prototype/`, outside `src/`, so nothing here ships in the crate and
`src/ui_assets/` stays untouched for `todo.console-signed-widening`.

- `server.mjs`: the long-running service. Serves the console, holds the
  journey, streams progress over server-sent events.
- `lib/cairn.mjs`: the graph adapter. Composes layers, parts, states, and
  waves out of `cairn context`, `cairn frontier`, and `cairn get`.
- `lib/harness.mjs`: spawns the maintainer's harness non-interactively,
  reads its JSONL event stream, and turns it into plain activity lines.
- `lib/prompts.mjs`: the three closed prompt contracts (describe, grill,
  settle).
- `lib/journey.mjs`: the four moments as a state machine, persisted so a
  restart or a closed tab loses nothing.
- `ui/`: the console, in the aligned specimen's register and class
  vocabulary, on design-system tokens served live from
  `docs/design-system/` rather than copied.
- `Dockerfile`, `compose.yaml`: builds `cairn` from this checkout and
  installs the harness, so the prototype tracks local cairn.
- `README.md`: how to run it, what each screen fact derives from, and what
  the maintainer is asked to do.

### Verified end to end, not asserted

Driven in a browser against a fresh project with the specimen's own
wording ("I want a calculator app. Typing a sum shows the answer as I
type, and I can see my past sums."). All four moments landed:

- **Describe and the map forms.** The harness wrote a real blueprint: 11
  ghost parts in 3 plain-language layers (Screen, Working, Foundations),
  13 edges. The mini-map filled in while the step was still running,
  because the map is read from cairn on every frame rather than at the end.
- **Waves.** Four waves, from `cairn frontier` tiers, rendered as
  sentences: "Numbers, Store and Settings. Nothing rests under them, so
  they go first, side by side." The notes are derived from the graph's
  `blocking` sets, so they cannot drift from it.
- **The grill.** Four plain questions, one marked as the call worth
  arguing with (uneven division, reaching the Numbers foundation).
  Answering the last one settled them automatically, keeping run as the
  one action.
- **Run.** Refuses, and says why on its face.

### Defects the smoke test caught, and their fixes

1. Mid-journey the browser froze on a stale frame while the service held
   current state: the event stream had stopped reaching that one client. The
   first suspect was Node's default five minute `requestTimeout`, and a
   control experiment disproved it (a three second `requestTimeout` leaves an
   open event-stream response untouched), so that change was reverted rather
   than kept on a false rationale. The cause is not isolated. What is fixed is
   the class: the client now reconnects on any stream loss, and because the
   first frame of a connection is the whole state, a reconnect is also a
   catch-up. Two real hazards on the push path were closed at the same time,
   both able to strand every client: a slow cairn call could stall frames
   behind it (calls are now bounded, and only one frame is ever in flight),
   and one dead socket aborted the write loop before the live ones got their
   frame.
2. The conversation auto-scrolled to the newest line, which hid the grill's
   head and its first question. Questions now open at the top of the block;
   anything else keeps the reader where they were.
3. The settle step recorded 11 decisions from 4 answers, putting calls the
   person never made into artefacts that then carry authority over the build.
   Three changes, because a prompt is a request and not a guarantee. The
   settle contract now allows exactly one decision per answer and requires a
   count before finishing (re-run against the same four answers: 4 recorded,
   down from 11). The code checks the delta itself and, when it does not
   match, tells the person on screen how many calls were recorded beyond
   their answers (verified with a stub harness that writes six: the warning
   fires with the right numbers). It reports rather than deletes, because
   removing an artefact the person has not seen would trade one silent act
   for another. And the drawer reads its tally from cairn rather than from the
   answers held in the browser, and labels it as decisions, which is what cairn
   counted, so it cannot claim a total the project does not carry.

### The review pass, and what it changed

Two read-only reviewers, one on correctness and one on simplicity and honesty.
Both raised findings that were fixed, and both raised findings that were
checked and rejected with evidence.

Rejected: a claimed Docker blocker (the build stage was said to be missing
`.claude/skills`, which `include_str!` needs). Only the `impeccable` subfolder
is excluded, and the image builds. Also rejected, twice: a claimed typo in the
compose default `${CAIRN_PROTOTYPE_HARNESS_ARGS:--p ...}`. With the variable
explicitly unset, compose renders `-p --mode json --no-session --no-skills`.

Fixed, all of them honesty defects in the register rather than crashes:

- The ready screen promised background progress ("you can leave; it keeps
  going", "these fill in as it runs") while run refuses and nothing is
  running. Both claims removed; the standfirst now says plainly that nothing
  is being built because run is not wired.
- The run plate claimed a build policy the graph does not encode (the bottom
  layer first, stops only for questions it cannot answer alone). Layers can
  interleave across waves, as this very run does. The replacement copy then
  overreached in a smaller way, claiming each wave waits for the one before it,
  and a probe settled it: in a part-built map a ghost part whose dependencies
  are already built sits at wave 2 with nothing blocking it, and the map can
  open at wave 2 with no wave 1 in it at all. The plate would have contradicted
  its own wave note two inches below. It now says only that the order comes
  from what depends on what, which holds in both cases.
- That same probe exposed two more: a single-part wave read "nothing is holding
  these up", and a blueprint with no containers rendered "3 parts in 0 layers".
  The note is singular-aware now, and the layer clause only appears when the
  graph declares layers.
- The decision-count warning read as "more was decided than you decided" in
  both directions. It now branches, and a shortfall says that a call the
  person did make is not written down anywhere.
- The harness's success sentence was shown next to a warning proving it
  false. It is now suppressed whenever the measured count disagrees.
- The working panel said "open any time" while permanently open. It is a
  closed `details` now, and raw harness output moved behind its own
  disclosure instead of sitting in the primary layer.
- "A couple of calls are yours to make" appeared above four questions; the
  stage strip read "questions answered" over unanswered questions; the
  activity title said "reading what you wrote" while the grill was running.
  All three now derive from actual state.
- A composer labelled "ask anything, or change your mind" only ever started a
  fresh description. It says so now.
- A wave with nothing blocking it was described as waiting for the wave
  before it. It now says nothing is holding it up.
- A module outside every container was counted as a layer cairn never
  declared. It is shown, labelled as in no layer, and excluded from the count.
- The README claimed wiring run was one edit and that state survives a
  restart. Neither is true as stated; both are now qualified to what holds.
- Dead weight removed: an unobservable `mapped` state, an unused cancellation
  seam, an unused export and field, unused heading rules, a redundant settle
  route, and wire fields nothing read (part ids, blocker lists, findings, the
  cairn version on every frame).

### The Docker path, verified rather than assumed

Built and run, not just written. The image carries `cairn 0.9.0` compiled from
this checkout and the harness at `/opt/bun/bin` (deliberately not under `/root`,
which compose mounts a volume over for the harness sign-in). Compose brings it
up, the boot log names the right project, harness, cairn, and design system, the
console renders with tokens and fonts served from the image, run refuses inside
the container too, and it survives `compose restart`. No `.env` reaches the
image: the scoped ignore keeps credentials out of the build context, and the
repository ignores the path so they cannot be committed either.

### Not wired, and deliberately so

The run plate refuses because `cairn ruling run` does not exist yet:
`dec.webui-write-authority` clause 4 is proposed, not signed. The reason is
stated in one constant (`RUN_NOT_WIRED` in `lib/journey.mjs`), but wiring run is
not one edit: the route, the click handler, and a dispatch path that does not
exist yet all have to move together, after the verb ships.

The console does write, and that is not a contradiction: it writes the
target project's own blueprint and decision artefacts, because turning the
person's words into a map is the journey being prototyped. It dispatches
nothing and records no orchestration fact, which is the boundary
`dec.orchestration-placement` draws.

## Session 2: register defects worked one at a time (2026-08-06)

Driven against the recorded ground truth: the brief's dated amendment
(`studio/orchestration-console-brief.md`, 2026-08-06), the session 1
record above, and the aligned register reference. Every fix was measured
in a browser against two maps, a greenfield one (the calculator, built by
the harness in this session) and a part-built one, because copy that
holds in an all-ghost map can be wrong the moment something is built.

### Preflight, settled before touching the console

- `dec.webui-write-authority` is still **proposed** in `cairn pending`
  (binding, `cairn.ui` and `cairn.root`). So its four ratification
  bullets did not land, no supersession was written, and run stays
  unwired. Clause 6 is exactly this state: silence never grants
  authority.
- The round 2 and 3 artefacts commit: the mocks (the todo cites the
  guided journey by path as its register reference), the brief amendment
  and the two todo records (the owning artefacts for provisional
  rulings), the decision (a binding record the maintainer signs against a
  commit), and the derived `map.json` refresh, which the installed binary
  reproduces.
- `webui-write-authority.md:180-182` stands. Those lines are clause 5's
  compression of bet D, and they carry the superseded record's obligation
  faithfully (`webui-design-authority.md:87-89`: the scorer as a new
  benchmark baseline goes through a segment bump, never an ordinary
  keep). Nothing to revert.

### The part-built fixture

A second console instance runs against a hand-built ledger project where
three of seven parts exist on disk. It reproduces the case the greenfield
map structurally cannot show, the same one session 1's probe found: the
plan opens at wave 2, there is no wave 1, and built parts carry no wave
at all. No harness spend, so it is cheap to re-check.

### Fixed, each verified in both maps before the next was started

1. **Layer glosses were paragraphs, not labels.** The bands rendered the
   container description whole, 20 to 25 words, wrapping to two lines
   (measured: 29px against a 14px single line), where the register sets a
   handful of words beside the layer name. `composeMap` now takes the
   description's leading clause and keeps it only while it still reads as
   a label; past that the layer name stands alone, because a sentence cut
   mid-phrase reads worse than no gloss. Every band is one line now, in
   both maps, and the description is untouched where it was declared.
2. **A part-built plan opened at wave 2 with nothing saying why.** The
   rows read 2 and 3 under a standfirst counting two waves, which reads
   as a broken list. The panel now says it starts at wave 2 because what
   would have come first is already built, and that is provable rather
   than asserted: no wave 1 row means no unbuilt part sits at that depth.
   Absent in the greenfield map, which opens at 1.
3. **The legend explained dashed but never solid.** Built chips had a
   treatment the legend never named. The solid clause now appears exactly
   when a built chip does (3 in the ledger, none in the calculator).
4. **The working drawer was off-register and its glyph grammar was
   broken.** Six items where the reference strips to four, and the
   three-bar map glyph did duty for three unrelated facts (the blueprint,
   the harness command, the project path). Harness and project are one
   fact, where this is running, so they are one item with their own
   glyph. Four items with four distinct glyphs, and five once decisions
   exist. That fifth item is a deliberate deviation from the reference
   strip, not alignment with it: which harness and which project this
   drives is real deployment truth a mock had no host to carry, and the
   bezel is the wrong home for it, because a command line and a
   filesystem path in the primary layer is the jargon the plain-primary
   ruling puts behind disclosure.
5. **One number, three places.** The parts count appeared in the
   conversation summary, again in the quiet line under it, and again in
   the standfirst. The quiet line keeps only what was its own (the map
   filled in as this was written), and how much of the map stands moved
   to the map's own title, which is where a reader looks for it and where
   nothing else was saying it: "3 of 7 built", or "all still to build".
6. **The `RUN_NOT_WIRED` docstring still claimed flipping it was one
   edit.** The README and this todo had already been corrected; the code
   comment had not, so the next reader would have believed the wrong one.

### Raised, and deliberately not built

The amendment asks for readouts of how many processes and agents run.
There is no such telemetry: the harness adapter emits plain activity
labels and one child process runs per step, so a count would either be
the constant 1 or invented, and it would imply a fleet that is not there.
Execution readouts belong to the layered execution graph, which the
amendment defers, and they have nothing to report until run is signed and
wired. Recorded as design input, not a defect.

### Smoke, after every fix

The whole journey again, in a browser: the calculator described, 12 parts
in 3 layers across 4 waves formed, 4 plain questions drained, 4 decisions
recorded from 4 answers (no count warning, so the measured delta agreed),
and Run refused on its face with the reason behind its disclosure. The
settle step amended the map from the answers, which is visible on it: the
wipe control the person asked for is a part now.

## Acceptance

- The maintainer tests the prototype as a user on a demo calculator
  project: describes the app, watches the map form, drains the grill,
  and runs.
- The maintainer's user feedback is recorded (a dated brief amendment or
  `cairn feedback`) and becomes the input to the next design round.

## Sequencing

Consumes `dec.webui-write-authority` (the run verb) once signed, and the
driver seams from `todo.driver-in-repo` as they land (rulings observed,
outcomes recorded, lease facts served). `todo.console-signed-widening`
keeps ownership of the production console implementation the mockups
specify; this unit is the prototype vehicle the round 2 ruling ordered,
and its findings feed that unit rather than replacing it. The design
unit (`todo.console-orchestration-ux-design`) ships no prototype code;
implementation lives here.

2026-08-07 audit (todo.roadmap-assumption-audit): keep; downstream of console UX design work.
