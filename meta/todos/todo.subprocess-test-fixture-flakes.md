---
node: cairn.root
status: open
created: 2026-08-10
---

# Subprocess Test Fixture Flakes On CI

## Scope
Two tests that drive shell-script fixtures through `std::process::Command` fail
intermittently on the CI Linux runner and never locally. Identify each cause and
make both deterministic, or fix whatever defect they are intermittently
exposing. Not by widening an assertion: each one currently defends a real class
boundary. Cross-node, hence anchored at `cairn.root`: one test lives under
`cairn.authoreval`, the other under `cairn.tests` and exercises
`scripts/auto-pr.sh`.

## Evidence
Both observed on 2026-08-10 while landing PR #661, which touched neither test
nor any code they exercise. Five `check` attempts over two commits, in order:

1. Run 31348131685 attempt 1, commit `402929bd`: failed on the authoreval test.
2. Run 31348131685 attempt 2, same commit: passed.
3. Run 31348527350 attempt 1, commit `a0d64906`: failed on the dogfood test.
4. Run 31348527350 attempt 2, same commit: failed on the dogfood test.
5. Run 31348527350 attempt 3, same commit: passed.

`main` at `91120e60` passed a re-run over the same window. The two failures have
different call paths and may have different causes; treat them as two
investigations that happen to share a shape.

1. `authoreval::loop_tests::test_command_backend_classifies_an_unparseable_answer_as_protocol`:

   ```
   assertion `left == right` failed
     left: Invocation
    right: Protocol
   ```

   `BackendErrorClass::Invocation` covers `NonZeroExit`, `Io`, and
   `ScriptExhausted` (`src/authoreval/backend/mod.rs`), so the call failed
   before reaching `serde_json::from_str`. The assertion prints only the class,
   so the cause is not in the log. `CommandBackend` has a writer thread that
   tolerates only `ErrorKind::BrokenPipe`, and the stub exits without draining
   stdin, so a non-`BrokenPipe` write error is one candidate the dogfood test
   cannot share.

2. `auto_pr_passes_the_gated_head_to_merge_guard` (`tests/dogfood_gate.rs:205`).
   `scripts/auto-pr.sh` exited non-zero having printed only its banner, with
   empty stderr, in 0.11s. The other three `auto_pr_*` tests passed in the same
   run against the same fixture, and they reach further into the script. Only
   two commands can exit non-zero and silently that early: the fake `gh`'s
   `else exit 1` and the fake `jq`'s `*) exit 1`, both in
   `tests/dogfood_gate.rs`. This test spawns through `Command::output` and has
   no writer thread, so candidate 1 above cannot apply. Its assertion already
   prints both child streams, and both were empty, so the next step here is
   diagnostics inside the fakes rather than around them.

## Ruled out
- **`echo "$PR_JSON" | jq` tripping `pipefail` on SIGPIPE.** The fake `jq` never
  drains stdin, so this looked likely. Measured on Debian: 400/400 failures with
  a 200 KB payload, 0/400 with the real payload of roughly 130 bytes, which fits
  the pipe buffer, so `echo` never blocks. Not the cause.
- **`scripts/auto-pr.sh` itself, or the fake tools, being wrong on Linux.** The
  whole fixture, replicated in shell inside a Debian container against the real
  script, passed 30/30.
- **`ETXTBSY` from a sibling fixture.** Linux raises it only while the
  executable being launched is itself open for writing. Every fixture writes its
  scripts with `fs::write`, which closes before `chmod` and `spawn`, and each
  owns a distinct `tempdir`, so no sibling holds the file another test execs.

PR #661 is not a direct code-path regression: it modified neither test nor the
code either one calls. It did add sibling `loop_tests` that write and execute
shell scripts, so it changed the parallel workload, and that stays in the causal
set until a cause is measured.

## First steps
Path-specific, because the two failures share no mechanism yet:

- Authoreval: print the full `BackendError`, not only its class, so the next
  occurrence names `NonZeroExit`, `Io`, or `ScriptExhausted` and its detail.
- Dogfood: make the fake `gh` and fake `jq` fallback branches print what they
  were called with before exiting 1. Both currently exit silently, which is why
  a captured-stderr assertion learned nothing.

## Acceptance
- Each cause is identified from a printed error, not inferred.
- Both tests are deterministic under the full suite on the CI runner, or the
  defect either exposed is fixed with its own test.
- Both still fail on a plausible bug: a backend answering outside the response
  shape must still classify as `protocol`, and `auto-pr.sh` must still pin the
  reviewed head.

## Sizing
S.

## Non-goals
Do not widen either assertion to accept the failing value, and do not mark
either test `#[ignore]`. That deletes the only coverage of a real boundary.
