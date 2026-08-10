---
id: res.brownfield-node-modules-ignore-suppression
nodes:
  - cairn.brownfield
sources: [src.autodocs]
method: primary
date: 2026-08-10
---

# Suppressing `node_modules` from brownfield init: the behavioural evidence

`res.autodocs-arm-a-brownfield-run` defect 3 established the redundancy of
manifest-derived `node_modules` ignore entries from two source facts and
recorded that the run could not demonstrate it behaviourally: injecting a
`node_modules` tree left the finding count unchanged, but so did a control
directory under a non-ignored name, so nothing could be attributed to the ignore
machinery. It closed by saying a purpose-made fixture was needed. This records
that fixture and the confirming re-run.

## What was measured

Cairn built from the loop worktree at `a405d7b2` plus the fix, debug profile.

**Fixture (deterministic, reproducible from this tree).**
`test_cli_init_from_code_never_suggests_node_modules` in `src/cli/mod.rs` runs
`cairn init --from-code` over a tree carrying a root `package.json` and a nested
`packages/api/package.json`, in two states: the sibling `node_modules`
directories absent, and the same directories present on disk. Both states assert
the proposed `ignore_suggestions` wire and the written `cairn.config.yaml`.
Against the pre-fix binary the absent state proposes `["node_modules",
"packages/api/dist", "packages/api/node_modules"]`; after the fix both states
propose `["packages/api/dist"]` and neither config mentions `node_modules`.

The two states matter because the entries reach the config by two independent
routes, and existence is what separates them: the manifest route fires on a
`package.json` whether or not the sibling directory exists, and the classifier
route fires only on a directory that does. A fixture testing one state alone
would pass while the other route stayed open.

**Arm A re-run (external target, not reproducible from this tree).** The numbers
below come from a run against a third-party repository, so nothing in this
repository verifies them. Reproduce with: clone `TrySita/AutoDocs`, check out
`795ff04ddf6637cf044424f93c9fa807e08181cc` (the commit `src.autodocs` pins), and
run `cairn init --from-code --apply` from the clone root.

| Binary | `node_modules` lines in `cairn.config.yaml` |
|---|---|
| pre-fix (`a405d7b2`) | 15 |
| post-fix | 0 |

The post-fix config is `ignore:` with a single `target` entry. Every
path-specific ignore that run wrote was a `node_modules` entry, so on this
target the fix removes the whole scaffolded block rather than trimming it.

## What this settles, and what it does not

It settles that neither route emits an entry for a directory named exactly
`node_modules` any more, which is the claim defect 3 could not close. It does
not measure whether suppressing them changes what the reconciler enumerates, and
it does not need to: the built-in ignore list (`src/scanner/config/mod.rs`) and
discovery's `is_ignored_dir` (`src/brownfield/walk.rs`) both exclude that exact
name unconditionally, which is why the entries were redundant rather than
load-bearing. The failed attribution attempt in the Arm A run is consistent with
that, not evidence against it.

"Exactly" is load-bearing. The onboard classifier compares path segments after
lowercasing but returns the original casing, so a directory named
`NODE_MODULES` is still classified as an ignore candidate, while the scanner and
discovery match exactly and do not ignore it. That entry is therefore not
redundant and must keep reaching the config, which is why the suppression is
case-sensitive and why
`test_cli_init_from_code_keeps_uppercase_node_modules_suggestion` pins it.

The suppression is deliberately narrow, and the two ignore lists are not the
same list. The scanner's built-in list is `.git`, `target`, `node_modules`,
`.DS_Store`, `.claude` (`src/scanner/config/mod.rs`); discovery's
`is_ignored_dir` is a longer, separate set that also covers `dist` and `build`
(`src/brownfield/walk.rs`). `node_modules` sits in both, so a config entry for
it changes nothing either surface does. `dist`, `build`, `vendor` and
`__pycache__` still earn a config line: discovery may prune them, but the
scanner does not, so only the written entry keeps their contents out of
`cairn scan`'s orphan findings, which
`test_cli_init_from_code_apply_scaffolds_ignores_without_existing_config`
asserts directly.

One adjacent case is redundant by the same test but costs the adopter nothing,
and was left alone. `target` is a classifier pattern and is also in the
scanner's built-in list, so an existing `target/` directory is proposed as an
ignore entry: a tree holding `src/alpha` plus `target/debug` proposes exactly
`["target"]`. It does not reach the config twice. `append_initial_ignore_entries`
matches the suggestion against the unquoted `- target` line the config template
already writes (`ignore_line_matches` strips the quoting before comparing), and
the applied config carries one `target` line, measured. The residue is one line
of pre-apply proposal output with no effect on what gets written or scanned,
which is below the bar for a tracked unit; recorded here rather than filed.
