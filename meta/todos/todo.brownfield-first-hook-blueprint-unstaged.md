---
node: cairn.kernel.hooks
status: open
created: 2026-08-09
---

# First `cairn hook all` after onboarding errors on an unstaged blueprint

Filed from the Arm A brownfield stress test over TrySita/AutoDocs
(`res.autodocs-arm-a-brownfield-run`, source `src.autodocs`).

## Evidence

Reproduced on a clean clone. After `cairn init --from-code --apply`, measured on
three states of the same working tree:

| State of `cairn.blueprint` | `cairn hook all` |
|---|---|
| untracked, nothing staged | exit 1, `Error: CAIRN_HOOK_AFFECTS_SUBSET cannot read or parse the candidate blueprint while checking ratification evidence` |
| staged, not committed (`git add -A`) | exit 0 |
| committed | exit 0 |

Staging alone clears it, so the trigger is an untracked or unstaged blueprint,
not an uncommitted one.

The hook reads the candidate blueprint from the git candidate tree: the index
under the default `RatificationMode`, `HEAD` in CI
(`src/hooks/ratification/git.rs:107-119`). `candidate_local_decisions` turns a
`None` from that read into a hard Error
(`src/hooks/ratification.rs:135-143`). A freshly onboarded repository has
`cairn.blueprint` untracked, so the candidate read has nothing to return.

This is the adopter's first gate result. The next-steps ladder that
`cairn init --from-code` prints ends with "6. Check the commit gate:
`cairn hook all`" and never says to stage first, so following the printed
sequence in order meets a hard Error.

## Tension to resolve

The fail-closed behaviour is deliberate: the comment at
`src/hooks/ratification.rs:61-71` argues that inside a work tree an unanswerable
git may be hiding an acceptance, so enumeration failure must never be silent.
"Blueprint absent from the candidate tree" is distinguishable from "git refused
to answer", and only the former can be proven benign, so the two should not
share one Error. But "absent" is itself two states, and only one is benign: a
blueprint that was never tracked (fresh onboarding) versus a tracked blueprint
whose deletion is staged. Collapsing those would let a commit that removes the
blueprint walk through the gate it is supposed to trip.

## Scope

Pick one and implement it:

- Treat "no blueprint in the candidate tree AND none in `HEAD`" as nothing to
  gate, the same way `inside_work_tree` returning false already short-circuits,
  while leaving every other read failure a hard Error. The `HEAD` clause is what
  keeps a staged deletion of a previously tracked blueprint failing closed; do
  not implement the benign case as a bare "candidate read returned `None`".
- Or amend the printed next-steps copy in `docs/design-system/copy.toml` so the
  adopter stages the onboarding output before the gate step. Staging is
  sufficient; do not prescribe a commit.

## Acceptance

- A test asserts `cairn hook all` on a repository whose `cairn.blueprint` is
  untracked produces the chosen outcome; that a tracked blueprint whose deletion
  is staged still errors; and that a genuinely unreadable candidate blueprint
  still errors.
- Re-running Arm A end to end reaches step 6 without an Error finding.
