# Live OMP adapter validation, 2026-07-26

Raw evidence behind `res.pack-omp-adapter-validation` and
`dec.pack-adapter-roots`. Host: `omp` 17.1.3 at `$HOME/.bun/bin/omp`.
Binary under test: `cairn` 0.8.0 built from the loop worktree at the tip of
`loop/todo.agent-pack-omp-adapter`. Shell-profile noise lines are stripped;
nothing else is edited.

## 1. Live discovery with the pack installed

Command: `omp -p "List the cairn skills you can see, then read skill://cairn-dev
and quote its first heading. Do not use bash." --cwd /tmp/omp-adapter-smoke
--no-session --auto-approve --mode json`

Tool calls made by the host: 1 (read {"path": "skill://cairn-dev"})

Answer:

```text
**Cairn skills visible (8):**

| Skill | Purpose |
|---|---|
| `cairn-apply` | Apply a change, run verification gates, mark complete |
| `cairn-archive` | Archive a completed change |
| `cairn-dev` | Entry point / router for repos with `cairn.blueprint` |
| `cairn-explore` | Query the Cairn graph: architecture, nodes, findings, decisions |
| `cairn-loop-implement` | Implement-and-test step of one dev loop iteration |
| `cairn-loop-landing` | Landing/merge step: commit, PR, review, squash-merge |
| `cairn-loop-reconcile` | Plan-reconciliation step after Verify, before Land |
| `cairn-loop-recovery` | State-recovery for interrupted loop iterations |
| `cairn-loop-scope` | Scoping step: success criterion, prerequisite rerouting |

**First heading of `skill://cairn-dev`** (after YAML frontmatter):

> `# cairn-dev`

No bash used — read tool only.
```

## 2. Live discovery with no pack installed (control)

```text
$ cd /tmp/omp-adapter-ab && omp -p "List every skill you can see whose name starts with cairn. If there are none, say NONE. Do not use bash." --cwd /tmp/omp-adapter-ab --no-session --auto-approve --max-time 240
[exit 0]
Three, all resolving via `skill://` (each `hide: true`, so invoke explicitly):

- **cairn-waved-parallel-fanout** — waved `task` fan-out over the cairn backlog, one PR per unit
- **cairn-typescript-project-traps** — driving a cairn-governed TS/bun project around cairn 0.1.x Rust-only defaults
- **cairn-bootstrap-blueprint** — replacing the starter `cairn.blueprint`, clearing `CAIRN_BLUEPRINT_CHANGE_NO_DECISION`

Source: `$HOME/.claude/skills/` (globbed, then each read back through `skill://` to confirm it's actually visible, not just on disk).
Still starting after 10s — phase: createAgentSession > discoverAdvisorConfigs
  logs: $HOME/.omp/logs/omp.2026-07-26.66805.log · re-run with PI_DEBUG_STARTUP=1 for streaming phase markers
Working...
```

## 3. Native command surface

```text
$ cd /tmp/omp-adapter-smoke && omp -p "Do not run it and do not use bash. Answer two things: (1) is a project slash command named cairn-loop available here, and (2) reading its file only, what exact path does its step 2 tell you to load when no campaign is pinned?" --cwd /tmp/omp-adapter-smoke --no-session --auto-approve --max-time 240
[exit 0]
**1) Yes.** `cairn-loop` exists as a project slash command: `.omp/commands/cairn-loop.md` (name: "Cairn Dev Loop", category Workflow). It's the OMP-adapter copy; no `.claude/commands/` variant here.

**2) No campaign pinned → step 2 loads:**

`skills/cairn-dev/references/loop-mode.md` under the installed pack root.

Here the pack root is `.omp/` (the file states `.claude/` for the Claude adapter, `.omp/` for OMP), so the concrete path is `.omp/skills/cairn-dev/references/loop-mode.md` — which does exist in the tree. If it can't be loaded, the file mandates: touch nothing, report, final line `LOOP HALTED`.
Still starting after 10s — phase: createAgentSession > discoverAdvisorConfigs
  logs: $HOME/.omp/logs/omp.2026-07-26.48543.log · re-run with PI_DEBUG_STARTUP=1 for streaming phase markers
Working...
```

## 4. Campaign smoke on the OMP install

```text
$ rm -rf /tmp/omp-campaign-smoke && mkdir -p /tmp/omp-campaign-smoke/.omp && cd /tmp/omp-campaign-smoke && printf 'System App "S" id "s" {}\n' > cairn.blueprint
[exit 0]

$ cd /tmp/omp-campaign-smoke && <loop-worktree>/target/debug/cairn --file /tmp/omp-campaign-smoke/cairn.blueprint pack install --loop
[exit 0]
Installed the cairn agent pack (omp, bundle 1.0.0, cli 0.8.0).
  written: 21
  refreshed: 0
  adopted: 0
  unchanged: 0

$ cd /tmp/omp-campaign-smoke && <loop-worktree>/target/debug/cairn --file /tmp/omp-campaign-smoke/cairn.blueprint pack campaign start --loop
[exit 0]
Campaign pinned to the loop entry and its 6 closure assets.
Resolved the loop entry (omp, bundle 1.0.0, cli 0.8.0).
  bundle: a8246173df3a70ad1fbef36f408198ee16443a3d757c64ca06322ad1d8fe91ae
  cli: 770bc51e7c004958201277e5ff5c7c30fd4be34cbcc20f9ff77ef81b6981d4ec
  prompt: .omp/commands/cairn-loop.md 7dcd14a97d43dbeec866fb222e13adfe853b2ab934d16e38656f82db62b23450
  closure: .omp/skills/cairn-dev/references/loop-mode.md 4d7aec9d48a0a5beb9caa0c12ca5cb293d3a90bf00d4ed4df60abd248c8ca7c5
  closure: .omp/skills/cairn-loop-scope/SKILL.md 3f52a300646a7be81e07928d127d1016f71b8089de84c3c72d8b7a6cb02d81f1
  closure: .omp/skills/cairn-loop-implement/SKILL.md bd3a779b4f47ffec6ac3050716d1e74384e8bf7313b3e0851d812513fa39bb76
  closure: .omp/skills/cairn-loop-recovery/SKILL.md e051a5849024bfe82091903324dc07128b85f614cad8b65bb769a20d8931f684
  closure: .omp/skills/cairn-loop-reconcile/SKILL.md c58ff94d461622acbd89f3033300869afcaef6bfb72f383e50d7529bf67d61dc
  closure: .omp/skills/cairn-loop-landing/SKILL.md 1aab30be1601aa9b287cab2edc2f122c2a21661fc531fe36c404c416ed91641f
  pinned: .cairn/state/campaign/a8246173df3a70ad1fbef36f408198ee16443a3d757c64ca06322ad1d8fe91ae

$ cd /tmp/omp-campaign-smoke && <loop-worktree>/target/debug/cairn --file /tmp/omp-campaign-smoke/cairn.blueprint pack campaign verify --loop
[exit 0]
Campaign bytes match: loop entry, 6 closure assets.
  pinned: .cairn/state/campaign/a8246173df3a70ad1fbef36f408198ee16443a3d757c64ca06322ad1d8fe91ae

$ cd /tmp/omp-campaign-smoke && printf 'tampered procedure\n' >> .omp/skills/cairn-loop-scope/SKILL.md
[exit 0]

$ cd /tmp/omp-campaign-smoke && <loop-worktree>/target/debug/cairn --file /tmp/omp-campaign-smoke/cairn.blueprint pack campaign verify --loop
[exit 1]
HALT: the installed pack no longer matches this campaign.
.omp/skills/cairn-loop-scope/SKILL.md differs from the bytes the packager wrote

$ cd /tmp/omp-campaign-smoke && <loop-worktree>/target/debug/cairn --file /tmp/omp-campaign-smoke/cairn.blueprint pack update --loop
[exit 0]
Updated the cairn agent pack (omp, bundle 1.0.0, cli 0.8.0).
  written: 0
  refreshed: 0
  adopted: 0
  unchanged: 20
Modified since cairn wrote them, left untouched:
  .omp/skills/cairn-loop-scope/SKILL.md

$ cd /tmp/omp-campaign-smoke && <loop-worktree>/target/debug/cairn --file /tmp/omp-campaign-smoke/cairn.blueprint pack campaign verify --loop
[exit 1]
HALT: the installed pack no longer matches this campaign.
.omp/skills/cairn-loop-scope/SKILL.md differs from the bytes the packager wrote

$ cd /tmp/omp-campaign-smoke && <loop-worktree>/target/debug/cairn --file /tmp/omp-campaign-smoke/cairn.blueprint pack campaign end
[exit 0]
Campaign snapshot removed. Pack drift is information again.

$ cd /tmp/omp-campaign-smoke && <loop-worktree>/target/debug/cairn --file /tmp/omp-campaign-smoke/cairn.blueprint pack campaign start --loop
[exit 1]
.omp/skills/cairn-loop-scope/SKILL.md differs from the bytes the packager wrote

$ cd /tmp/omp-campaign-smoke && <loop-worktree>/target/debug/cairn --file /tmp/omp-campaign-smoke/cairn.blueprint pack campaign verify --loop
[exit 1]
No campaign is active here.
```

## 5. Repair and re-pin

```text
$ cd /tmp/omp-campaign-smoke && rm .omp/skills/cairn-loop-scope/SKILL.md && <loop-worktree>/target/debug/cairn --file /tmp/omp-campaign-smoke/cairn.blueprint pack update --loop
[exit 0]
Updated the cairn agent pack (omp, bundle 1.0.0, cli 0.8.0).
  written: 1
  refreshed: 0
  adopted: 0
  unchanged: 20

$ cd /tmp/omp-campaign-smoke && <loop-worktree>/target/debug/cairn --file /tmp/omp-campaign-smoke/cairn.blueprint pack campaign start --loop | head -3
[exit 0]
Campaign pinned to the loop entry and its 6 closure assets.
Resolved the loop entry (omp, bundle 1.0.0, cli 0.8.0).
  bundle: a8246173df3a70ad1fbef36f408198ee16443a3d757c64ca06322ad1d8fe91ae

$ cd /tmp/omp-campaign-smoke && <loop-worktree>/target/debug/cairn --file /tmp/omp-campaign-smoke/cairn.blueprint pack campaign verify --loop | head -2
[exit 0]
Campaign bytes match: loop entry, 6 closure assets.
  pinned: .cairn/state/campaign/a8246173df3a70ad1fbef36f408198ee16443a3d757c64ca06322ad1d8fe91ae
```

## 6. Routed just-in-time load

Command: `omp -p "I have a bug to investigate in this cairn project. Follow the
cairn-dev router and load ONLY the one just-in-time reference it routes a bug
investigation to. ..." --cwd /tmp/omp-adapter-smoke --no-session --auto-approve
--mode json`

Tool calls made by the host, in order:

```text
read {"path": "skill://cairn-dev"}
read {"path": "skill://cairn-dev/references/task-bug-investigation.md"}
```

Answer:

```text
Loaded reference: **`references/task-bug-investigation.md`** (via `skill://cairn-dev/references/task-bug-investigation.md`).

Routing basis: the cairn-dev Routes table maps "Find why something misbehaves, then fix it" → `references/task-bug-investigation.md`. That was the only matching row, so nothing else was read.

Stopping here as instructed: no investigation started, no `cairn context` / `cairn scan` run, no bash, no other files read.
```

## 7. Two-revision campaign: adoption needs a new campaign

Revision A is the pack this branch ships. Revision B is the same pack with one
canonical body changed (`content/skills/cairn-loop-scope/SKILL.md`), re-rendered
and recompiled, then installed over A with `pack update`.

```text
$ rm -rf /tmp/omp-two-revision && mkdir -p /tmp/omp-two-revision/.omp && cd /tmp/omp-two-revision && printf 'System App "R" id "r" {}\n' > cairn.blueprint
[exit 0]

$ cd /tmp/omp-two-revision && <loop-worktree>/target/debug/cairn --file /tmp/omp-two-revision/cairn.blueprint pack install --loop | head -2
[exit 0]
Installed the cairn agent pack (omp, bundle 1.0.0, cli 0.8.0).
  written: 21

$ cd /tmp/omp-two-revision && <loop-worktree>/target/debug/cairn --file /tmp/omp-two-revision/cairn.blueprint pack campaign start --loop | sed -n '1,4p'
[exit 0]
Campaign pinned to the loop entry and its 6 closure assets.
Resolved the loop entry (omp, bundle 1.0.0, cli 0.8.0).
  bundle: a8246173df3a70ad1fbef36f408198ee16443a3d757c64ca06322ad1d8fe91ae
  cli: e3226af0c02c44e7195b04e6f357cb069f263dfc0667e3341c872a8628ed64eb

$ cd <loop-worktree> && printf '\n<!-- revision B marker for the two-revision campaign smoke -->\n' >> tools/agent-pack/content/skills/cairn-loop-scope/SKILL.md && cargo run -q -p cairn-agent-pack -- --write | tail -1 && cargo build -q 2>&1 | tail -2; echo REBUILT
[exit 0]
Agent pack write succeeded.
REBUILT

$ cd /tmp/omp-two-revision && <loop-worktree>/target/debug/cairn --file /tmp/omp-two-revision/cairn.blueprint pack update --loop | head -4
[exit 0]
Updated the cairn agent pack (omp, bundle 1.0.0, cli 0.8.0).
  written: 0
  refreshed: 1
  adopted: 0

$ cd /tmp/omp-two-revision && <loop-worktree>/target/debug/cairn --file /tmp/omp-two-revision/cairn.blueprint pack campaign verify --loop
[exit 1]
HALT: the installed pack no longer matches this campaign.
.omp/skills/cairn-loop-scope/SKILL.md changed since the campaign was pinned.

$ cd /tmp/omp-two-revision && <loop-worktree>/target/debug/cairn --file /tmp/omp-two-revision/cairn.blueprint pack campaign end
[exit 0]
Campaign snapshot removed. Pack drift is information again.

$ cd /tmp/omp-two-revision && <loop-worktree>/target/debug/cairn --file /tmp/omp-two-revision/cairn.blueprint pack campaign start --loop | sed -n '1,4p'
[exit 0]
Campaign pinned to the loop entry and its 6 closure assets.
Resolved the loop entry (omp, bundle 1.0.0, cli 0.8.0).
  bundle: b4ec652fe0df043a6b95cfbf6f601b82f71dce5f482d3e3c78ef69798d17ed44
  cli: fa829d39305dcde964d37ab7314538aa3b2931ab21b7481045dd370271bd0271

$ cd <loop-worktree> && git checkout -- tools/agent-pack/content/skills/cairn-loop-scope/SKILL.md .claude/skills/cairn-loop-scope/SKILL.md && cargo build -q 2>&1 | tail -1; echo RESTORED
[exit 0]
RESTORED
```

Pinned bundle before: `a8246173df3a70ad1fbef36f408198ee16443a3d757c64ca06322ad1d8fe91ae`.
After `pack update` installed revision B, the running campaign verified as
`HALT` rather than adopting it. Only after `campaign end` did a fresh
`campaign start` pin revision B, at bundle
`b4ec652fe0df043a6b95cfbf6f601b82f71dce5f482d3e3c78ef69798d17ed44`. The canonical
body was restored afterwards, so the branch ships revision A.
