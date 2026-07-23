---
id: res.loop-efficiency-observations
nodes:
  - cairn.kernel.cli
  - cairn.kernel.query
  - cairn.mcp
date: 2026-07-12
method: primary
---

# Loop efficiency observations (living log)

## Question

Where does cairn's output spend tokens without informing the loop, and what
context is an agent missing at the moment it starts a unit? This is a living
observation log: append a dated entry when a session produces new evidence,
rather than minting a fresh research artefact per anecdote. Hypotheses
graduate to implementation todos (or decisions) once entries corroborate
them; a discovery todo that merely tests a hypothesis may be filed earlier
(as todo.agent-context-bundle does for the bundle hypothesis below).

## Method

Session-level observation of the cairn dev loop running on this repo. Each
entry records what was observed in one session, labelled observed (seen in
transcript output) or hypothesis (single occurrence, plausible mechanism,
unconfirmed).

## Entries

### 2026-07-12 (loop session, PRs #274-#278)

Observed:
- The decision-deferred `CAIRN_SPEC_RULE_UNIMPLEMENTED` info finding printed
  in full, identically, on every `cairn scan`, `cairn lint`, `cairn hook all`,
  and pre-push gate invocation across the session. Unchanged and not
  actionable during this session (its deferral decision exists); the
  observation supports presentation deduplication, not suppression - a
  deferred finding becomes relevant again when its decision or the
  implementation state changes.
- `cairn status` output is O(backlog): it lists every open todo plus several
  identical trailing log entries. The loop consumed only the "Next
  recommended" line, the finding count, and active changes.
- "Next recommended" matched the unit actually executed at all four
  iteration boundaries; bd was never invoked. Native-todo adoption was
  strong this session (first clean single-tracker session; one data point,
  not proof the fallback pattern is gone).
- Closing one unit required coordinated manual edits across three todo
  files (status flip, run record, supersession pointer), and a reviewer
  caught a genuine lifecycle contradiction introduced during those edits.
- A post-merge `cairn scan` dirtied `map.json`, requiring a separate chore
  PR (#277) to land the snapshot refresh.

Hypotheses (single occurrence each):
- When the recommended unit is blocked on an unmet dependency, `cairn
  status` offers no fallback candidate; a top-3 with one-word blocked
  reasons would remove a manual triage step.
- Snapshot refresh could ride the triggering PR instead of a chore PR, but
  a pre-push hook that mutates the feature branch produces diffs the author
  did not stage; a scripted one-command follow-up may capture most of the
  value without hook-mutation semantics. Needs a decision if pursued.
- Temporal context: an agent implementing a todo cannot cheaply see how
  often the target node has changed before (a per-node revision count or
  version marker derived from git history plus the archive trail). Value
  unproven; parked here until the context-bundle work surfaces a concrete
  need.

### 2026-07-23 (agent context bundle inventory)

Question: which existing Cairn query compositions should the later
context-bundle evaluation compare, and what fixed sampling and accounting rules
will make that comparison reproducible?

Method: compare the query registry, handlers, serializers, CLI renderers, and
MCP adapters at repository revision `5dacf46`. The later evaluation runs against
the immutable revisions registered by `todo.agent-guidance-baseline`. This entry
does not open the sealed confirmation set, measure candidates, change the public
query surface, or give a router authority over a hypothetical command.

CLI `--json` prints the query data payload and adds `schema_version`; `locate`
prints only its bare match array. MCP returns the outer query envelope with
`project_context`, `rules`, and `findings`. Both transports use the same
handlers, but their bytes are not interchangeable. An exact transport and
argument list is therefore part of a candidate.

#### Surface inventory

| Verb | Default payload | Opt-in or lean behaviour | Task decision served |
|---|---|---|---|
| `context` | Human output defaults to a depth-1 rolled projection; CLI JSON and MCP return every node and edge plus artefact, finding, and ready-backlog counts | Human `--scope` and `--depth` change presentation; `--depth all` gives the full human graph. There is no compact JSON or token-budget mode | Choose the project area before opening node detail |
| `get <node>` | Full node identity, hierarchy, ownership, paths, contracts, state, files, and accepted decision IDs | CLI `--symbols` adds root-node symbols; MCP has no corresponding flag | Confirm ownership and inspect node symbols and accepted authority |
| `neighbourhood <node>` | Node plus direct inbound and outbound edges, contract paths, and accepted decisions | Todos, research, reviews, deprecated decisions, and active changes are separate include flags. Optional arrays are empty by default | Decide blast radius and which nearby artefacts need inspection |
| `bundle <node>` | Node, contract body or explicit missing contract, accepted direct-node decisions, provenance research and sources, outbound dependency symbols, and gates | Fixed projection with no lean or expansion flags. Human rendering is not byte-equivalent to JSON | Decide whether the node has enough contract, rationale, dependency interface, and gate context to implement |
| `deps <node>` | Direct dependency node IDs | `--transitive`; inbound uses `--direction in` or the MCP dependents tool | Decide prerequisite order or downstream impact |
| `rationale <node>` | Enriched accepted decisions for the node and one-hop neighbours, with `via` labels, linked research, and sources | Fixed enriched projection with no lean flag | Decide why the node is shaped this way and which evidence supports that authority |
| `locate <symbol>` | Exact symbol-name matches with node, file, line range, kind, and signature | Exact match only; no fuzzy or full-text mode | Decide the definition site for a preregistered symbol |
| `todos [node]` | Enriched todo metadata and body, project-wide when node is absent and descendant-inclusive when present | Status filter for `open`, `in_progress`, `done`, or `blocked` | Decide which native work item governs the node and whether it is selectable |
| `sources <node>` | Enriched sources directly connected through the node's decisions and research | Node-direct rather than neighbourhood-wide; no lean flag | Decide which external evidence backs the node's authority |

There is no global compact, field-selection, summary, or token-budget switch.
The available reductions are query-specific:

- `status --brief` reduces status rendering but is adjacent to, not part of,
  the nine context verbs.
- Human `context --scope <node> --depth <N>` bounds presentation only and is
  not equivalent to JSON or MCP context.
- `neighbourhood` omits optional artefact families unless include flags are
  passed.
- `deps` is direct unless `--transitive` is passed.
- Lean artefact objects omit bodies. Enriched `rationale`, `todos`, and
  `sources` include path, title, and body.
- CLI `get --symbols` is an executable detail mode. MCP omits that request
  flag, so transport choice changes root-symbol availability.

Unless a row says otherwise, its default payload describes CLI JSON and MCP
handler data. Human renderers are leaner:

- human `get` prints ID, name, description, state, and accepted decision IDs,
  omitting JSON kind, hierarchy, ownership, paths, contracts, files, and span;
- human `neighbourhood` prints terse node, edge, contract, and artefact lines,
  omitting JSON node structure and artefact objects;
- human `bundle` prints the contract body, decision IDs, dependency interfaces,
  and gates but omits the JSON rationale research and sources;
- human `todos` prints node, status, and path, omitting title and body;
- human `sources` prints ID, verification, and file, omitting title and body;
- human `rationale` prints IDs, status, path, and `via` or citation links,
  omitting enriched titles and bodies.
- human `deps` and `locate` preserve the same node IDs or match facts as JSON
  but use headers and lines instead of structured objects.

#### Overlap and missing facts

| Concern | Existing overlap | Missing or transport-sensitive fact |
|---|---|---|
| Node identity and ownership | `context`, `get`, `neighbourhood`, and `bundle` | No compact node projection shared by all transports |
| Topology | `context`, `neighbourhood`, `deps`, and `bundle.dependencies` | `deps` returns IDs only; `bundle` covers outbound dependencies only |
| Decisions and provenance | `get` decision IDs, `neighbourhood` decisions, `bundle.rationale`, `rationale`, and `sources` | Scope and enrichment differ between direct-node and one-hop views |
| Native work | `todos`, optional neighbourhood todos, status backlog, and bundle gates | `bundle` has no todos or active changes; backlog and native todos answer different questions |
| Symbols and files | CLI `get --symbols`, `bundle.dependencies`, `locate`, and `files` | MCP `get` omits root symbols; `locate` needs an exact prompt-visible name |
| Contracts and gates | `get` contract paths, neighbourhood contract paths, bundle contract body, and bundle gates | No other named composition returns both contract body and gates |
| Change contact | Optional neighbourhood active changes and change queries | `bundle` omits touching active changes |

No existing verb returns, in one fixed node-scoped projection, the contract
body, open native todo, files, state, inbound and outbound topology, accepted
authority, provenance, dependency interfaces, touching changes, and gates. This
is an inventory fact, not evidence that a new verb is worthwhile.

This evaluation begins after a task has been assigned to one or more target
nodes. It measures post-routing context assembly, not target discovery. Grade
facts only when they change one of these decisions: confirm ownership; recover
contract and authority; order prerequisites and impact; recover dependency
interfaces and gates; or reconcile touching changes and proof. The navigation
baseline separately grades selection, definition discovery, and
time-to-correct-file. Payload that supports none of the post-routing decisions
is irrelevant here. Support repeated after a decision has enough evidence is
duplicate.

#### Fixed sample rule

Use one evaluation manifest derived from the baseline corpus and freeze it
before candidate output is opened. It is not a second task corpus. Each task
record MUST contain `task_id`, `split`, `repository_id`, `task_class`, a
`prompt_symbols` array containing only exact symbol names visible in the task
prompt, and a non-empty `required_facts` array. Each fact has a unique `id` and
a non-empty `evidence_files` array copied from the baseline's preregistered
ground truth. The stored manifest, not evaluator extraction, is authoritative.

For every task, first map each fact's evidence files to the deepest owning
Cairn nodes at the pinned revision. When equally deep nodes overlap, choose the
lowest bytewise UTF-8 node ID. A fact counts once for every node owning at
least one of its evidence files. `required_nodes` is the distinct owned-node
set. The `primary_node` has the greatest distinct fact count, then greatest
distinct owned evidence-file count, then lowest bytewise UTF-8 node ID.

Keep every fact with an unowned evidence file in the task recall denominator.
Its unowned support has no node attribution and remains unsupported unless a
task-global locate result supports it. If `required_nodes` is empty, set
`primary_node` to the reserved sampling value `__unowned__`, run no node-scoped
invocations, and run only task-global invocations. The task remains in every
denominator and run table. Otherwise run node-scoped steps for every required
node in ascending bytewise UTF-8 order.

Then, for every non-empty `(split, repository_id, task_class)` stratum:

1. Encode
   `agent-context-bundle-v1\0<repository_id>\0<split>\0<task_class>\0<task_id>`
   as UTF-8 bytes and compute SHA-256.
2. Sort by lowercase hexadecimal digest bytes and then ascending bytewise
   UTF-8 `task_id`, never locale collation.
3. Select two tasks, or every task when the stratum has fewer than two.
4. In the first pass, admit at most one task per `primary_node`; then fill any
   remaining slot from the original digest order.

`todo.agent-context-bundle-evaluation` builds, freezes, and executes only the
development-split manifest after baseline publication. The treatment unit
builds and executes the confirmation-split manifest only after it authorizes
opening the sealed prompts and ground truth. No confirmation metadata is
computed in the context-bundle evaluation.

#### Deferred measurement protocol

For every invocation, retain the exact request or argument vector, ordinal,
monotonic start and end time, exit or error state, and raw stdout and stderr
bytes. Keep failures and empty outputs in the intention-to-treat data. Evidence
grading uses stdout only. Report stderr bytes, Unicode scalars, and tokens as a
separate cost stream; never merge streams. Aggregate payload cost is the sum of
the independently measured stdout and stderr counts. Time-to-evidence inspects
stdout only.

Count raw bytes; Unicode scalar values after strict UTF-8 decoding; calls;
elapsed time; and time to first sufficient evidence. Internal filesystem opens
are not observable from these CLI responses and are reported as `not measured`,
never inferred from surfaced paths. Record first-turn or catalogue,
just-in-time instruction, and tool-result tokens separately. The evaluation
manifest MUST provide non-empty
`tokenizer.implementation`, `tokenizer.package_version`,
`tokenizer.encoding`, `tokenizer.vocabulary_revision`, and
`tokenizer.vocabulary_sha256` fields before output is opened. Tokenize exact
captured streams only with that pinned implementation; reject mismatches.

Required-fact recall is fully supported required fact IDs divided by all
required fact IDs. Annotate supporting stdout byte spans with fact IDs before
tokenization. Evidence units are JSON terminal scalar leaves, splitting
multiline strings into nonblank JSON-pointer-plus-line units, or nonblank
human-output lines. Evidence-unit precision is relevant units divided by all
units and is reported per invocation and transport only; never aggregate or
rank it across JSON and human transports.
An evidence unit is relevant if any of its bytes intersects an annotated
required-fact support span; a partial intersection counts the unit once.

Transport-neutral precision is stdout tokens overlapping required-fact support,
including duplicates, divided by all stdout tokens. Classify every stdout token
exactly once:

- `relevant-first` overlaps support before a fact is sufficient, including
  partial support and the support that makes it sufficient;
- `duplicate` overlaps support only after every fact it supports is sufficient;
- `irrelevant` overlaps no required-fact support.

When a token overlaps several classes, use the first class above. Per-invocation
recall and token precision use only that invocation's stdout. Per-task measures
use stdout tokens accumulated in invocation order, so sufficiency and
duplication carry forward. Per-stratum recall and token precision are micro
averages: sum task numerators and denominators before division. Also publish
the per-task distribution. A failed or empty invocation has recall, evidence
precision, and token precision `0`; every task has at least one required fact,
so recall never has a zero denominator. Precision is `0` when its denominator
is zero.

Qualitative grading stays blind to candidate identity. Publish per-invocation,
per-task, and per-stratum results and dispersion. This entry performs none of
those calculations.

#### Candidate compositions

`$CAIRN` is the absolute path to the pinned baseline binary recorded in the run
manifest. Every `JSON` step below is the literal CLI vector shown with
`--json`; every `human` step omits it. Measure both streams separately. Execute
steps in written order for every `required_node` in bytewise UTF-8 order. Then,
once per task rather than once per node, run
`$CAIRN locate <symbol> --json` for every manifest `prompt_symbols` value in
bytewise UTF-8 order. An empty symbol array produces no locate calls.

1. Bundle-centred:
   `$CAIRN bundle <node> --json`;
   `$CAIRN todos <node> --status open --json`;
   `$CAIRN neighbourhood <node> --include-changes --json`.
2. Primitive:
   `$CAIRN get <node> --symbols --json`;
   `$CAIRN deps <node> --json`;
   `$CAIRN deps <node> --direction in --json`;
   `$CAIRN rationale <node> --json`;
   `$CAIRN todos <node> --status open --json`.
3. Topology-first:
   human `$CAIRN context --scope <node> --depth 1`;
   `$CAIRN neighbourhood <node> --include-todos --include-research
   --include-changes --json`;
   `$CAIRN get <node> --symbols --json`;
   `$CAIRN deps <node> --transitive --json`;
   `$CAIRN deps <node> --direction in --transitive --json`;
   `$CAIRN sources <node> --json`.

The paper-only `context_projection_v1` is an output-only hypothesis, not an
implemented invocation. For each node, a fixture builder captures
`$CAIRN get <node> --json`, `$CAIRN bundle <node> --json`,
`$CAIRN neighbourhood <node> --include-changes --json`,
`$CAIRN rationale <node> --json`, and
`$CAIRN todos <node> --status open --json`. Once per task it captures
`$CAIRN context --json` and the exact locate sequence above. Those captures
are fixture preparation, not candidate-visible
output. The evaluation grades only the emitted projection bytes for recall,
precision, bytes, characters, and tokens. Calls, files, and elapsed time are
reported as `not measured` until an implementation exists and never enter a
candidate ranking. Emit exactly one UTF-8 RFC 8785 canonical JSON document per
task. The `nodes` array has one row per `required_node`; it is empty for
`__unowned__`. Task-global locate results appear once in top-level `symbols`,
including when `nodes` is empty. The closed schema is:

```json
{
  "schema_version": 1,
  "nodes": [{
    "node": {"id": "", "name": "", "kind": "", "state": "", "paths": [], "files": []},
    "contract": {"body": ""},
    "missing": [],
    "edges": [{"direction": "in|out", "node": "", "label": ""}],
    "todos": [{"path": "", "status": "", "title": "", "body": ""}],
    "decisions": [{"id": "", "via": [], "title": "", "body": ""}],
    "research": [{"id": "", "title": "", "body": ""}],
    "sources": [{"id": "", "verification": "", "title": "", "body": ""}],
    "dependencies": [{"node": "", "symbols": [{"name": "", "file": "", "line": 0, "end_line": 0, "kind": "", "signature": ""}]}],
    "active_changes": [""],
    "gates": ""
  }],
  "symbols": [{"symbol": "", "node_id": "", "file": "", "line": 0, "end_line": 0, "kind": "", "signature": ""}]
}
```

Populate fields from exactly these sources:

- Each `nodes` row corresponds to one requested node.
- `nodes[].node` comes from the displayed scalar and array fields in `get`.
- `nodes[].contract.body` comes from `bundle.contract`. If absent, emit
  `contract: null` and add `"contract"` to that row's `missing`.
- `nodes[].edges` comes from `context.edges` filtered to rows whose `source` or
  `target` is that node. Direction is `out` when `source` matches and `in` when
  `target` matches; node is the opposite endpoint; copy `label`.
- `nodes[].todos` comes only from `todos.todos`.
- `nodes[].decisions`, `research`, and `sources` come only from corresponding
  `rationale` arrays. Bundle and neighbourhood artefact rows do not participate.
- `nodes[].dependencies` and `gates` come only from `bundle`; dependency
  symbols use exactly the fields displayed in the schema.
- `nodes[].active_changes` is the string array copied from
  `neighbourhood.active_changes`.
- Each top-level `symbols` row is one element from the bare CLI locate array,
  augmented with the requested manifest `prompt_symbols` value from that call's
  argument vector as `symbol`.

Copy listed values without summarization and ignore every unlisted response
field. The closed type of `contract` is either JSON `null` or an object
containing exactly the displayed `body` string. Use empty arrays
for absent collections and an empty string for absent gates. Deduplicate and
sort every object array by each row's complete RFC 8785 canonical JSON bytes;
sort scalar arrays by bytewise UTF-8 value. Apply the same rule recursively to nested arrays.
Sort top-level `nodes` by ascending bytewise UTF-8 `node.id`, overriding the
generic object-array rule.
Filter todos to direct-node open todos, decisions to accepted decisions, and
symbols to exact requested symbols. This projection adds no
search, storage, fingerprint, scheduling, or workflow state. It has no public
command name and must not enter guidance unless a later decision and
implementation deliver it.

`todo.agent-context-bundle-evaluation` owns every measurement, threshold,
recommendation, and query-implementation follow-up. It waits for the baseline
corpus and uses the fixed rules above without opening confirmation data early.
