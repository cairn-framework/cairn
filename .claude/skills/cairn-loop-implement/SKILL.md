---
name: cairn-loop-implement
description: The implement-and-test procedure for one cairn-dev loop iteration: derive and create the unit branch, make the smallest change satisfying the success criterion, keep the blueprint honest, and cover changed behaviour with a test. Loaded by cairn-dev loop mode at its Implement step; declares the typed exits that step routes on. Not for ordinary development sessions.
license: MIT
compatibility: Requires Cairn CLI.
---

# Implement one loop unit

Loaded by `cairn-dev` loop mode at its Implement and test step. Inputs: the
selected unit, its resolved `node`, the success criterion from `cairn-loop-scope`,
and the bound `$CAIRN`.

Declared exits, exactly one, as the last line you return to loop mode:

- `IMPLEMENTED`: the change is written and its test exists; loop mode runs Verify.
- `LOOP HALTED`: implementation cannot proceed without a maintainer.

## 1. Get on the right branch

The branch is `loop/<tail>`, where `<tail>` is the derived form from loop mode's
Isolation rule: `todo.<slug>`, `<finding-code>.<node>`, or `split.<slug>`. Every
later step (push, PR, Cleanup) uses this exact name.

If it is already checked out (adopted at verdict time, or created earlier this
session during MISSION materialisation or decomposition), continue on it.
Otherwise create it from fresh origin/main:

```bash
git checkout --detach origin/main && git checkout -b loop/<tail>
```

If the derived name already exists but was NOT adopted by the verdict and NOT
created this session, you missed a preflight row. Do not improvise and do not
halt here: return to loop mode's preflight table, which owns that state and can
adopt or quarantine the branch. Say which name collided.

## 2. Make the smallest change that satisfies the criterion

Not the best change you can think of. The smallest one that makes the criterion
true. Anything else is a separate unit.

- Touch only what the criterion requires. Do not improve adjacent code, reformat,
  or refactor things that are not broken. Match the surrounding style even where
  you would have chosen differently.
- Remove imports, variables, and helpers that YOUR change orphaned. Leave
  pre-existing dead code alone; mention it instead.
- If you write far more code than the criterion needs, stop and reconsider. A
  senior engineer calling the result overcomplicated is a defect.

## 3. Keep the blueprint honest

The graph must still describe the code when you are done:

- Every new file falls under a node `path`, tests included. If none fits, extend a
  node's paths or declare a new Module.
- Every new cross-module call gets a blueprint edge. Check for a cycle first:
  `$CAIRN deps <target> --transitive`.
- User-facing strings go to the copy location named by loop mode's Repo bindings.
  Never hardcode them in source, and never guess the path: read the binding.

## 4. Test the behaviour

Changed behaviour gets a test. For a bug fix, write it first: red, then green.

The test defends the observable contract named by the success criterion. A test
that cannot fail on a plausible bug is not coverage. Keep it deterministic and
safe to run in the full suite.

Substantial work goes through the `cairn-propose` and `cairn-apply` skills rather
than being improvised here.

## 5. Return

Report what changed, which nodes were touched, the blueprint edits if any, and the
test you added. Then output your single exit token as the final line.
