---
node: cairn.kernel.query
status: done
created: 2026-07-16
---

# Bundle Real Gates

`cairn bundle` and `brief` do not surface the project's actual
verification gates. All three call sites
(`src/query_api/handlers/bundle.rs:93`, `src/cli/render/bundle.rs:74`,
`src/cli/render/remediate.rs:370`) emit static text from
`docs/design-system/copy.toml` whose [brief] gates entry still claims
language and build gates belong in the repo's hook config "not in cairn",
contradicting the shipped `gates:` config feature (PR #340,
`src/scanner/config/mod.rs:62-68`, run by `src/cli/accept/gates.rs`). An
agent implementing a ghost node gets boilerplate instead of the exact
verification recipe.

Fix: thread the actual gate recipe (config gates when present, otherwise
the BatterySelection result) into the bundle/brief render paths, and
rewrite the stale [brief] gates entry in copy.toml.

Second step, deferred until a polyglot consumer materialises: optional
per-target gate overrides on the existing `targets:` entries, with
supplement-not-replace precedence decided explicitly; note per-target
gates imply per-target accept execution semantics that do not exist
today.

Motivation: `res.a2ui-analysis` finding 7 (a2ui codebase blueprints carry
per-codebase commands and its AGENTS.md bans guessed build sequences).
Overlap: `todo.agent-context-bundle` is the natural home for the second
step; the stale copy fix proceeds independently. First step needs no
change proposal; the per-target step does.

## Resolution

2026-07-16: Completed step 1 by resolving the accept gate recipe once from project configuration and language fallback, then rendering the exact name and command in bundle and brief human and JSON surfaces. Tests cover configured `gates:` and Rust battery fallback recipes. JSON keeps `gates` as a string, so `SCHEMA_VERSION` remains 1 and no wire-shape or snapshot change was needed. Updated the brief copy to describe Cairn accept gate resolution. Step 2, optional per-target gate overrides and their accept execution semantics, is deferred to its own proposal. Accept previously resolved from the process cwd, predating this change; once the brief displayed exact recipes this exposed a real `--file` root divergence, fixed by threading the project root through accept resolution and every spawned gate command.
