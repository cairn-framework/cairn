# What CAIRN is, positively

Load this file when: making architecture decisions, adding new artefact types,
or proposing changes to the two-chain topology.

Three principles, complementary to the negative-space "What to avoid" list in CLAUDE.md:

1. **Typed artefacts encode obligations, not labels.** Each direct type (`contract`,
   `decision`, `todo`, `research`, `review`, `source`) has a different role in the
   two-chain topology. The kernel's enforcement value comes from those role differences.
   Treating types as decorative labels (or proposing a flat schema) is the same mistake
   as flattening the two chains into a six-layer stack.

2. **Authoring guidance is template-driven and tag-extensible, never closed-enum.**
   Domain-specific vocabulary belongs in project config (`artefact_types`) or in tag
   conventions, both of which are extensible. The kernel speaks taxonomy; the project
   speaks domain.

3. **AI assists authoring; AI does not substitute for the reconciler.** AI may propose
   edges, draft contracts, suggest narrative summaries, all reviewable through the
   change-isolation primitive. AI may not produce the deterministic reality fingerprint
   that drift detection compares against. The enforcement layer stays mechanically
   checkable.

These three are the positive form of the rejections in "What to avoid."
Stated in `docs/spec.md` §3.5 with rationale.
