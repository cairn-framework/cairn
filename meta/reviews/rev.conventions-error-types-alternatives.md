---
node: cairn.root
date: 2026-08-11
reviewer: xai-grok-build/grok-4.5/contestedness-alternatives
review_type: agent_cross_model
subject_hash: sha256:f7e82160074f1bd7b9d760748f2aa845335c68e85063ddb8617ffc99cea76794
lens_prompt_hash: sha256:1ceb131f531393b6d998c5641ce6741774cce8f6b0305d4fd2876f4db4179003
---

# Receipt review: error types are hand-written without thiserror (alternatives lens)

## Claims verified

- Independent canonical manifest hashing produced the `subject_hash` above.
- Prose and enforcement now describe one `thiserror`-specific predicate; no
  broad derive-macro prohibition remains.
- The bounded assertion rejects the superseded mandate. The Cargo metadata
  guard catches renamed normal, dev, build, target, and member dependencies.
- Removing the source scanner resolves its duplicate walker, prose/import false
  positives, and multiline-derive false negatives without weakening the
  narrowed rule.
- Formatting, clippy, workspace tests, and the conventions test pass on the
  reviewed bytes.

## Alternatives

A broad error-derive ban would need an unbounded crate list or syntax-aware
policy machinery for a divergence observed only with `thiserror`. Retaining the
raw scanner costs more than it guards. Exact bullet equality would block benign
copy edits. Adopting `thiserror` or deleting the replacement rule contradicts
the unit acceptance. No alternative is live.

## Findings

No blocking findings. Accepted residual: a differently phrased stale mandate
could pass the prose assertion, but the package metadata gate still prevents
the dependency state; unused workspace declarations and transitive re-exports
are not package declarations.

## Contestedness

CONVERGENT. Prose, decision, research, and test assert one predicate, and the
simpler surface is the better-enforced one.

## Verdict

PASS
