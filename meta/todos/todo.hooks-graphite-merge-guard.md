---
node: cairn.kernel.hooks
status: done
created: 2026-07-12
---

# Exact-head verification for merge candidates

gh:#86

The merge guard must verify that the commit selected for merge is byte-exactly
the head that passed review and the gates. It is not a Graphite-specific
post-merge-loss detector. The check should enforce the same invariant as
`gh pr merge --match-head-commit`: a changed or replaced candidate head fails
closed before merge, instead of trying to discover dropped code after the
merge.

Current enforcement surfaces:

- `src/hooks/mod.rs` runs `cairn hook all` and combines blocking findings.
- `src/hooks/ratification.rs` already distinguishes the staged index from
  checked-out `HEAD` (`RatificationMode::Index` and `RatificationMode::Head`),
  so the candidate tree, not an edited worktree, is the reference.
- `scripts/dogfood.sh` is the working-tree pre-push and CI dogfood gate. It
  runs `cairn lint` and `cairn hook all` with the built binary.
- `src/cli/commands/hook.rs` parses `cairn hook all --head` and transport
  arguments.

Implementation unit (S, delivered in this PR):

- `scripts/auto-pr.sh` captures `GATED_HEAD_SHA` immediately after
  `gh pr checkout`, confirms the checkout has not changed after all gates,
  and passes `--match-head-commit "$GATED_HEAD_SHA"` to `gh pr merge`. GitHub
  then refuses a merge when the remote pull request head moved after review
  and gates.
- `tests/dogfood_gate.rs` executes the merge runner against mocked gate and
  `gh` commands and asserts that the exact gated SHA is passed.

No new hook runner behavior is needed: `src/hooks/mod.rs` and
`src/hooks/ratification.rs` remain the current candidate-tree checks;
`cairn hook all` and the pre-push `scripts/dogfood.sh` gate the checked-out
commit that the merge boundary pins. `src/cli/commands/hook.rs` continues to
provide the `--head` candidate-tree mode for CI.

Sizing resolution: the existing merge boundary provides one S-sized
integration point. It records the post-checkout head, checks local stability,
and delegates remote race rejection to `gh`; the regression test covers the
exact-head argument. Broader Graphite-specific post-merge scanning remains out
of scope.

Success criterion: a merge request cannot merge unless its candidate commit
equals the SHA that passed the reviewed and gated hook and dogfood battery;
any local head change or remote head mismatch blocks the merge before it
starts.

Re-minted from GitHub issue #86 by todo.github-issues-cleanup
(2026-07-12); the issue is closed pointing at this artefact.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves maintainable. It protects the merge path that keeps maintenance work safe.

2026-08-07 audit (todo.roadmap-assumption-audit): re-scope per res.chatgpt-issue-audit: rewrite as exact-head verification of the merge candidate rather than a Graphite-specific guard.

2026-08-09: resolved the audit disposition by adopting the exact-head scope
above. The S-sized merge-boundary guard and regression test are implemented;
the todo is closed via `./target/debug/cairn todo set`.
