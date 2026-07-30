# Tasks: source-tracked-verification-mode

- [x] Add `SourceVerification::Tracked`, the `"tracked"` parse arm, and the
      `"tracked"` wire arm in `source_verification`
- [x] Add the `Tracked` validation arm: lexical path rule, metadata probe,
      canonical containment, `CAIRN_SOURCE_SHA256_UNEXPECTED` on a declared
      `sha256`
- [x] Route `CAIRN_SOURCE_SHA256_UNEXPECTED` into the remediate source-issue
      list; register CA040 and generalise CA031 in
      `docs/registries/error-codes.md`; add the `[findings.codes]` copy entry
- [x] Write tests: parse case table, validate acceptance table (file, dir,
      missing, absolute, `..`, bare `./`, escaping symlink, leading `./`,
      sha256-unexpected, unknown value), serialise wire value, remediate action
- [x] Amend `docs/spec.md` block per clause 6 and re-check moved spec-rules
      anchors; add the three clause 7 Enforced rows
- [x] Update `docs/conventions.md` (field list, flexibility bullets, line 425,
      CA031 wording), `docs/artefacts.md`, both agent-pack references, and
      re-render the `.claude/` copies
- [x] Move both source records to `verification: tracked` and trim their
      deliberate-`unverified` paragraphs; amend
      `todo.source-self-reference-finding` Acceptance to four values
- [x] Produce the evidence: `cargo test` green, zero
      `CAIRN_SOURCE_UNVERIFIED` in `cairn lint --json`, `cairn scan --strict`
      exit 0
