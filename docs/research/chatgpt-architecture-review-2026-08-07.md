# Verdict

Cairn has **strong architecture discipline and unusually good self-governance**, but the newly merged coordination substrate is not yet a safe foundation for the in-repo driver.

The coordination implementation is split into sensible files, extensively documented, and well tested. However, the outer module remains shallow: CLI, QueryAPI, the wave composer, verifier, and future driver are all expected to understand raw fact-kind strings and JSON payload fields. The public interface therefore exposes much of the implementation rather than concentrating complexity behind one deep module.

## Top recommendation

**Harden and deepen coordination before starting the driver reaction loop.**

This is time-sensitive because `todo.driver-in-repo` is open and is about to introduce the second major adapter. If the driver consumes the current raw interface, provisional fact vocabulary and validation rules will become distributed across CLI, QueryAPI, driver, and tests.

This should **not** reintroduce the deleted generic `StateBackend`. There is still one storage implementation, so a backend abstraction would add interface without leverage. The required deepening is around coordination semantics, not interchangeable storage.

## Highest-priority defects

### 1. Public fact writes do not contain their filesystem path

`NewFact.recorded_at` and `kind` are public, unconstrained strings. They are interpolated into the filename, and the persistence function creates parent directories. By code inspection, a Rust library caller can supply absolute or parent-containing path components and cause a write outside the intended `facts/` directory. Current CLI callers generate safe values, so this is not presently a free-form CLI exploit; it is a public-library and imminent-driver risk.

### 2. Malformed leases fail open

The appender validates the fact-family prefix, actor and commit, but does not validate payload shape. Reader predicates interpret a missing or malformed `expires_at` as "not held." A malformed `lease.grant` can therefore exist while wave composition considers the unit dispatchable—the inverse of the documented fail-closed policy.

### 3. Append-only and cache integrity are assumed, not enforced

Ordinary facts use a replace-capable atomic write. The parse cache is keyed by filename and its cached `Envelope` is accepted without binding it to the current file bytes. Neither normal reads nor `coord verify` recompute the fact ID or check filename/body congruence. A replaced fact can therefore be hidden by an old cache entry, while modified cache content can become authoritative.

### 4. The cursor recreates the rejected high-water failure

The underlying reader correctly lists the entire store. QueryAPI then filters the result using `filename > since` and returns a filename cursor. If another fact lands in the same second but sorts below the previous cursor, a polling caller misses it. This is the exact same-second failure the full-list design was introduced to avoid.

### 5. `since` has incompatible meanings

For ruling and lease queries, `since` means a fact filename. For `wave stats`, the same field is compared against an RFC 3339 `recorded_at` timestamp. Both advertise `CoordinationRequest`. This is a concrete example of the generic `QueryRequest` becoming an implicit union whose valid field meanings are known only by convention.

### 6. Immutable facts reference disposable evidence

Large decline preimages are written under `cache/` and referenced from immutable facts. That sidecar is durable evidence, not reparsable cache. Deleting the supposedly disposable cache can make the recorded explanation incomplete.

## Other architecture findings

**Wave planning has the wrong internal home.** Write-set derivation, readiness, plan identity and composition policy live under `query_api::wave`, while `ruling_run` reaches directly into that implementation. Keep `cairn wave` as the passive public query, but consider moving deterministic composition into a first-class planning module, with QueryAPI and ruling-run as adapters. This preserves the accepted rule that the driver consumes the passive query and that core code never dispatches work.

**The external QueryAPI wire is justified; its interior is too type-erased.** CLI and MCP are two real adapters, so the shared seam earns its keep. The stable heterogeneous JSON response also has a valid reason to exist. Internally, however, tool-name strings, unrelated optional fields, schema-name strings and a second manual MCP schema match duplicate protocol knowledge. Typed per-family requests behind the existing wire would improve locality without breaking consumers.

**The query contract has already drifted.** It states schema version **1** and a fixed registry size of **36**; implementation is at schema version **11** with **50** tools. It also omits the new coordination dependency and request fields. Cairn has contract-staleness machinery, but it did not protect these factual statements. Volatile counts and versions should either be generated or omitted from prose contracts.

**PR reviewability weakened at the highest-risk point.** PR #589 carried 141 files and CodeRabbit skipped review because it exceeded the configured limit. The internal decision and panel process was extensive, but the implementation still missed several cross-file invariant contradictions. Future substrate work should land as smaller, independently reviewable slices.

## Recommended sequence

1. Add failing tests for path containment, exact fact kinds and payloads, malformed-lease behaviour, cache/file identity, same-second cursor safety, and durable sidecars.
2. Record the required refinement to the completed `atomic_write` decision, then make facts write-once and validate identity on every authoritative read.
3. Deepen `cairn.coord` and reduce its public surface before the driver consumes it.
4. Resume `todo.driver-in-repo`.
5. Later, move deterministic wave planning to a domain module and type the QueryAPI interior.

The existing CI and testing discipline are strong; these findings concern invariants that the present gates do not model, rather than missing baseline engineering controls.

## Deletion-test and do-not-change notes (from the visual report)

- `src/cli/mod.rs`: large, but its external interface is small; continue incremental command extraction, do not split merely to satisfy a line count.
- Filesystem store: one implementation is enough; a generic backend seam would add interface without leverage.
- Preserve: passive core, caller-supplied observation time, console never writes lease or singleton facts, coordination stays family-local under the git common dir, no big-bang WebUI state centralisation.
- Low-severity: digest validation accepts uppercase hex while documentation requires lowercase.
