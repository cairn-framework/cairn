# Console state-source matrix

Round 1 output of `todo.console-orchestration-ux-design` task 1, authored
2026-08-05. Status: presented for maintainer acceptance under the
ratification proviso in `studio/orchestration-grill-brief.md`. A rejection
routes back through the owning artefact; implementation of every row belongs
to `todo.console-signed-widening`, not this unit.

Authority: `dec.orchestration-placement` clause 3 (the four kinds of truth),
as corrected by the Q2 grill ruling recorded in
`todo.console-orchestration-ux-design` (lease facts are cairn truth, so their
rows name cairn queries; only live execution state names the driver
contract). Colour tokens come last in every row and are redundant by
construction: no distinction below survives on colour alone.

## The grammar in one paragraph

Each of the four families owns a carrier surface, and a state never leaves
its family's carrier. Declared intent and observed fact meet on the node
chassis (the map). Projections live in bands, chips, and readouts that always
carry a derivation mark. Execution state lives on run cards and annunciator
lamps, and is never painted onto a node. The first channel of any cross-family
pair is therefore the carrier itself; the channels listed per row keep every
within-family pair at two or more non-colour differences as well.

## The 2x2 the legend must render

The map's four states are one 2x2 (declared against observed), shown today as
four peer colours (defect 1 in the todo's evidence). The legend becomes the
quadrant itself:

|                    | observed: yes                  | observed: no             |
|--------------------|--------------------------------|--------------------------|
| **declared: yes**  | `synced` (solid, filled, calm) | `ghost` (dashed, hollow) |
| **declared: no**   | `orphaned` (tilted, filled)    | (not a state; absent)    |

Channel encoding carries the axes, not just the cells: **hollowness means not
yet observed** (dashed border, hollow dot), **tilt means not yet declared**
(-0.4deg, `--ui-orphan-tilt`). `drift` is not a quadrant: declaration and
observation both exist and disagree, so drift renders as a finding overlay on
a synced-family chassis (thick keel, count badge), sourced from lint, never
from `node.state`. The Rust enum has no Drift variant (`src/map/graph.rs:14-23`);
the shipped UI's treatment of drift as a fourth peer state is the confusion
this matrix removes.

## Non-colour channel inventory

Legal channels, each with one meaning everywhere: carrier surface; border
style (dashed = not yet observed); keel width; dot or glyph fill (hollow =
not yet real); tilt (= not yet declared); state word (mono, lowercase, the
wire word); lamp (uppercase tracked micro label, execution only); count
badge; band membership and sort position; seal glyph (decisions: hollow =
unsigned, filled = accepted, struck = superseded); timestamp readout;
derivation mark (`derived · observed HH:MM`, projections only); cross-link
row; mono against sans (machine identity against human names).

## Family: declared intent (authored)

Carrier: node chassis (dashed vocabulary) and artefact rows.

| State | Clause 3 kind | Source of truth (today) | Non-colour channels | Colour |
|---|---|---|---|---|
| `ghost` node | intent | `GET /api/graph` `node.state = "Ghost"`; `cairn get <id> --json`; enum `src/map/graph.rs:14-23` | dashed border; hollow keel and dot; state word `ghost` | `--ghost`, `--ghost-wash` |
| todo `open` | intent | `GET /api/node/<id>/todos`; `cairn todos <node>` | status word chip; hollow marker; backlog row position | neutral chalk |
| todo `in_progress` | intent | same | status word; lease cross-check line (`backed by lease <unit>` when a lease fact exists, `no lease recorded` when not) | neutral chalk |
| todo `blocked` | intent | same | status word; named blocker row (mono id); bar glyph on marker | neutral chalk |
| todo `done` | intent | same | status word; filled check glyph; history grouping | neutral chalk |
| decision `proposed` | intent | `GET /api/pending`; `cairn pending` (fields incl. `ruling_summary`, `rubric`, `age_days`, `changed_since_review`) | hollow seal glyph; queue membership (waiting-on-you); age readout (`waiting N days`) | `--hinge-wash` |
| decision `accepted` | intent | `GET /api/node/<id>/decisions`; `cairn decisions <node>` | filled seal glyph; date readout; lineage-plate authority position | neutral chalk |
| decision `superseded` | intent | same (status frontmatter) | struck title; `superseded by dec.<slug>` pointer row; history grouping | `--ink-faded` |

The `in_progress` lease cross-check is deliberate: an intent claim of
activity with no recorded lease fact is a smell the console surfaces rather
than smooths over.

## Family: observed fact (recorded)

Carrier: node chassis (solid vocabulary), lease tags, outcome rows. All
fields recorded; nothing here is inferred at render time.

| State | Clause 3 kind | Source of truth | Non-colour channels | Colour |
|---|---|---|---|---|
| `synced` node | fact | `GET /api/graph` `"Synced"` | solid keel; filled dot; calm chalk body (default state: no wash, no signal) | neutral at rest: the shipped teal `--synced` keel on every synced node is the signal spend the inversion removes; teal touches a synced node only on selection or focus |
| `orphaned` node | fact | `GET /api/graph` `"Orphaned"` | tilt (-0.4deg); `no declaration` meta line; solid keel, filled dot | `--orphaned`, `--orphan-wash` |
| `drift` on a node | fact | `GET /api/lint` findings for the node; `cairn lint --json` (not `node.state`) | thick or doubled keel; finding-count badge; pulse (static badge persists under reduced motion) | `--drift`, `--drift-wash` |
| finding `error` | fact | `GET /api/lint` | filled lamp word ERROR; first sort position; channel row | `--error` |
| finding `warning` | fact | `GET /api/lint` | hollow lamp word WARNING; second sort position | `--warning` |
| finding `info` | fact | `GET /api/lint` | no lamp, plain meta text; last sort position | `--info` |
| lease held | fact | **proposed cairn query**: the lease-facts read surface (a Q8 refactor seam; schema owned by rung 3 of `todo.parallel-dispatch-granularity`; fields fixed by Q2: unit id, holder harness and session, commit at grant, `granted_at`, `expires_at`, renewal) | claim tag on the unit row (mono holder id); expiry readout (`expires HH:MM`); recorded-fields table in detail view | `--hinge-wash` |
| outcome recorded | fact | **proposed cairn query**: the outcome-facts read surface, paired with the lease surface above (facts recorded by the driver through a sanctioned verb, `todo.driver-in-repo` task 3; harness tokens `ITERATION COMPLETE`, `LOOP EXHAUSTED`, `LOOP HALTED`, plus driver-derived class, keyed by unit id and commit) | verbatim token text (mono); timestamp and commit readout; history rail position | neutral chalk |

Lease and outcome rows name cairn queries and not a driver contract, per
the Q2 correction: both are cairn truth the driver writes through
sanctioned verbs. Neither read surface exists yet; naming and shape stay
with the owning todos, and this matrix consumes whatever they rule.
The owning todos define each future surface's schema and its versioning;
the spine's existing `schema_version` discipline (stamped onto every query
response in `src/query_api/mod.rs:317-318` and `:376-377`, stripped at the
HTTP boundary in `src/ui/server.rs:299-302`) is the in-repo precedent
available to them, not a contract these rows can claim before it is ruled.

## Family: derived projection (computed at query time, never stored)

Carrier: bands, chips, and readouts. Every projection renders a derivation
mark with its observation time. A projection never paints a node chassis and
never claims to be a recorded fact.

| State | Clause 3 kind | Source of truth (today) | Non-colour channels | Colour |
|---|---|---|---|---|
| ready (frontier) | projection | `GET /api/frontier` `ready[]` `{node, name, tier, has_contract, blocking: []}`; `cairn frontier --json` (`src/query_api/handlers/graph.rs:146-158`) | ready-band membership; tier ordinal readout; derivation mark | neutral; band title carries no signal |
| blocked (frontier) | projection | `GET /api/frontier` `blocked[]` with `blocking` ids | blocked-band membership; `blocked by N` count with mono ids; derivation mark | neutral |
| stale lease | projection | **console-derived** from the proposed cairn lease-facts read surface (`expires_at` and renewal fields, row above) plus an explicit observation time (clause 3: the core evaluates no expiry) | `stale` chip with `observed HH:MM`; residue rows (branch, attempt history, last activity); explicit `no outcome recorded` line | `--drift` accent on chip edge |
| next recommended | projection | `GET /api/status` `next_recommended`; `cairn status --json` | single `next` slot position; source attribution line (`from cairn status`); derivation mark with observation time; empty slot states that no recommendation exists | neutral |
| dependency tier | projection | `GET /api/roadmap` `tiers[]`; frontier `tier` field | stratum band position; tier ordinal label; derivation mark on the band header | neutral |

The stale row is the honesty test the Q2 ruling demands: an expired lease
with no live driver renders as **stale and unclassified**, never as failed,
never as crashed, never as a terminal outcome that was not recorded.

## Family: execution state (live, driver-owned)

Carrier: run cards in the runs rail and the bezel annunciator. Source for
every row: **a proposed, versioned driver-owned contract** (v1; owner
`todo.driver-in-repo`; the console consumes it read-only and renders nothing
when it is absent). No query exists today; `lease` and driver liveness appear
nowhere in `src/` (verified 2026-08-05).

| State | Clause 3 kind | Contract field (proposed) | Non-colour channels | Colour |
|---|---|---|---|---|
| run active | execution | session liveness per unit | lamp ACTIVE (filled); elapsed readout; run card with mono unit id linking to the map | `--ci-signal` lamp only |
| run waiting | execution | queue or gate wait per unit | lamp WAITING (hollow); wait-reason line (gate name or queue position) | neutral |
| run blocked on human | execution | blocked marker plus queue entry ref | lamp BLOCKED (barred); cross-link row into the waiting-on-you queue entry | `--drift` lamp edge |
| driver attached, idle | execution | contract answers; zero runs | annunciator DRIVER IDLE; last-poll timestamp; runs rail empty state stating the fact | neutral |
| driver absent | execution | no contract registered | annunciator NO DRIVER; runs rail empty state naming the attach step; no timestamps anywhere (nothing was observed) | neutral |
| driver unresponsive | execution | contract registered; heartbeat older than threshold, derived with observation time | annunciator DRIVER UNRESPONSIVE with `last heartbeat HH:MM`; every run card demoted to `as of HH:MM` readouts | `--drift` annunciator edge |
| driver crashed | execution | recorded termination fact in the contract (exit status, exit time, whether a supervisor observed it) | annunciator DRIVER CRASHED with exit readout (`exited 137 at HH:MM`); runs rail shows last-known cards under a crash notice row | `--error` annunciator edge |

Driver-absent, driver-idle, driver-crashed, and driver-unresponsive are
four rendered states a reader tells apart without a terminal: absent has
no timestamps at all, idle has a fresh poll timestamp and an explicit
empty statement, crashed has a recorded exit readout (status and time),
unresponsive has stale timestamps, no termination record, and says
exactly that. Crashed is a recorded fact; unresponsive is derived
evidence and never hardens into a crash verdict on its own.
Unresponsive-or-crashed driver plus expired-lease produces the stale
projection row above; the console never promotes any of it to a terminal
outcome on the unit.

## Pairwise proof obligation

Acceptance demands every state differ from every other in at least two
non-colour channels. The argument has three parts, because carrier surface
alone does not separate everything.

**Disjoint carriers.** Projection states live only in bands, chips, and
derivation-marked readouts; execution states live only on run cards and
annunciator lamps (uppercase tracked treatment). Any pair touching those
two families gets carrier plus the family marker (derivation mark, lamp)
before row channels are counted.

**Shared carriers.** Intent and fact share the node chassis and the
channel rows, so carrier proves nothing there; the axis encoding does.
On the chassis, not-yet-observed intent is dashed border plus hollow keel
and dot, while observed fact is solid border plus filled dot: two channels
(border style, fill) for every chassis-crossing pair, before tilt, badges,
or state words. On artefact rows, intent rows (todos, decisions) carry
status word chips and glyph markers in the backlog and queue surfaces,
while fact rows (lease tags, outcomes) carry mono recorded-field readouts
with timestamps in unit detail and history: chip-plus-glyph against
mono-plus-timestamp, again two channels row-wise.

**Repeated labels.** The word `blocked` appears in three families, so the
state word is not a distinguishing channel for those pairs; they are
separated explicitly. Todo `blocked` (intent) is a lowercase status word
chip on a backlog row with a bar glyph and a named blocker line. Frontier
blocked (projection) is band membership with a `blocked by N` count and a
derivation mark. Run blocked on human (execution) is an uppercase barred
lamp on a run card with a cross-link row into the waiting-on-you queue.
Each pair differs in carrier, case treatment, and marker: three channels.

The five near-pairs that historically collapse are called out:
ghost/orphaned (dashed+hollow against tilted+filled), synced/drift
(calm against thick keel + badge), stale/unresponsive (unit chip against
driver annunciator, different carriers), idle/absent (fresh timestamp and
empty statement against no timestamps and attach instruction),
crashed/unresponsive (recorded exit readout against a stated missing
record with stale heartbeat).

## Deliberately absent from this matrix

- `planned`, `declared`, `buildable` as state words: see the vocabulary
  ruling in `studio/orchestration-console-brief.md`.
- Progress percentages, ETAs, success predictions: no fact exists; the
  console does not invent one.
- A `Drift` node state: drift is finding-sourced, not a `NodeState` variant.
- Any console-writable execution state: the console shows and records
  rulings; it never holds a lease or advances a run
  (`dec.orchestration-placement` clause 4).
