---
id: res.brownfield-extraction-external-run
nodes:
  - cairn.brownfield
  - cairn.kernel.cli
sources:
  - src.turtles-adr-run
date: 2026-08-10
---

# The brownfield decision-extraction flow, run against rancher/turtles

Closes the external-repository half of `todo.brownfield-extraction-external-run`.
The flow under test is `dec.brownfield-extraction-mechanism`: the deterministic
`cairn onboard decisions` index plus the shipped cairn-dev reference
`references/task-brownfield-decision-extraction.md`. The in-repo fixture
assertion is separate and already landed
(`todo.brownfield-extraction-drafting-test`).

Repository: `rancher/turtles`, `src.turtles-adr-run`.
Commit: `d54023d5c399a5bdc95581c54255974e4ff6522a`.
Cairn: `0.9.0`, built from this repository at `origin/main` `dc450728`.
Run date: 2026-08-10. Worked in a shallow clone at `/tmp/cairn-xrun/turtles`,
which is a scratch directory and not durable evidence: this artefact carries
everything the run produced, so the clone can be deleted at any time.

## 1. Commands, in the order they were run

```bash
git clone --depth 1 https://github.com/rancher/turtles.git
cd turtles && git rev-parse HEAD     # d54023d5c399a5bdc95581c54255974e4ff6522a

cairn init --from-code --apply       # 11 added_nodes, 11 added_artefacts
cairn scan
cairn onboard decisions              # 11 bound, 19 unbound: every ADR unbound
cairn onboard decisions --json

# hand edit 1: append a node declaring path "./docs/adr", carrying the
#              decisions and research pointers
cairn scan
cairn onboard decisions --json       # 30 bound, 0 unbound

# hand edit 2: wrap every node in a System block and move the decisions and
#              research pointers onto it, per step 0 of the shipped reference
mkdir -p meta/decisions meta/research
cairn scan
cairn onboard decisions              # 30 bound, 0 unbound: the retained report
cairn onboard decisions --json

# hand author meta/research/adr-evidence-survey.md (method: primary)
cairn decision new proxy-types-over-unstructured \
  --node docs.adr --informed-by res.adr-evidence-survey
cairn decision new deletion-via-owner-references-and-imported-annotation \
  --node docs.adr --informed-by res.adr-evidence-survey
# author both draft bodies, then read all 19 documents end to end
rm meta/decisions/deletion-via-owner-references-and-imported-annotation.md
# revise the survey and the surviving draft against the full read
cairn scan
cairn pending
```

Both hand edits were needed before the shipped reference could be followed as
written, and both are findings rather than incidental setup. Section 7 records
them.

## 2. Evidence counts, per source class

The four classes are the landed `kind` labels. Counts are from the final
`cairn onboard decisions --json` run at the recorded commit.

| Kind | Count | Bound | Unbound |
|---|---|---|---|
| `document` | 19 | 19 | 0 |
| `code-target` | 11 | 11 | 0 |
| `readme-section` | 0 | 0 | 0 |
| `invariant-comment` | 0 | 0 | 0 |
| **Total** | **30** | **30** | **0** |

Wire header: `schema_version: 1`, `bound_count: 30`, `unbound_count: 0`.

`readme-section` is zero because no heading in the root `README.md` is exactly
Decision, Rationale, or Invariant; the file opens with an HTML banner block and
its headings are product-facing. `invariant-comment` is zero because the tree
carries no `// invariant:` or `# invariant:` marker at all. Per
`res.onboard-decision-evidence-scope` the invariant scan covers every source
file the survey observed while `code-target` covers discovery candidates only,
so the two counts are not comparable; here the wider scope simply found nothing.

## 3. Node bindings produced

The first index run, against the blueprint exactly as
`cairn init --from-code --apply` derived it, returned 11 bound and 19 unbound:
every ADR document was unbound, because the derivation declares only code
directories and no node claimed `docs/`. Declaring one node over `./docs/adr`
moved all 19 to bound. Both results are observed, not inferred.

Bindings after that edit:

| Evidence | Kind | Bound node |
|---|---|---|
| all 19 files under `docs/adr/` | `document` | `docs.adr` |
| `api/rancher/k3s/v1` | `code-target` | `api.rancher.k3s.v1` |
| `api/rancher/management/v3` | `code-target` | `api.rancher.management.v3` |
| `api/rancher/provisioning/v1` | `code-target` | `api.rancher.provisioning.v1` |
| `api/v1alpha1` | `code-target` | `api.v1alpha1` |
| `examples` | `code-target` | `examples` |
| `internal/controllers` | `code-target` | `internal.controllers` |
| `internal/controllers/clusterctl` | `code-target` | `internal.controllers.clusterctl` |
| `internal/provider` | `code-target` | `internal.provider` |
| `internal/sync` | `code-target` | `internal.sync` |
| `test` | `code-target` | `test` |
| `util/predicates` | `code-target` | `util.predicates` |

Unbound evidence at the final run: none. Unbound evidence at the first run: all
19 `document` entries, the same paths listed above.

Separately, `cairn scan` reports 13 Go files as
`CAIRN_RECONCILE_ORPHANED_FILE` against the derived blueprint, including
`util/annotations/helpers.go`, which defines the constant one considered
decision names. Those files are outside the evidence index (they are not
discovery candidates) but they are the reason a decision's code can be partly
unreachable from the graph.

## 4. The drafts this run retained

Two drafts were produced. One was handed on; one was withdrawn during the full
read of all 19 documents. Both are retained here verbatim, because the
withdrawal is the substantive result of the run.

### 4.1 Retained draft: `dec.proxy-types-over-unstructured`

Written by `cairn decision new proxy-types-over-unstructured --node docs.adr
--informed-by res.adr-evidence-survey`, then edited in the body and in the
permitted provenance fields only. Path in the clone:
`meta/decisions/proxy-types-over-unstructured.md`. It derives from
`docs/adr/0009-use-structured-proxy-types.md`, a path in `rancher/turtles` at
`d54023d5c399a5bdc95581c54255974e4ff6522a` that carried no cairn-specific
annotation before the run: the file predates this run by three years and the
whole `docs/adr/` tree is ordinary project documentation.

Verbatim, as it stood when the run ended:

```markdown
---
id: dec.proxy-types-over-unstructured
nodes:
  - docs.adr
status: proposed
date: 2026-08-10
informed_by:
  - res.adr-evidence-survey
revisit_triggers:
  - "controller-runtime stops accepting generated deepcopy types through the client.Object interface"
  - "the Rancher API packages become importable without pulling the Rancher fork of client-go into the operator dependency tree"
  - "a proxied Rancher resource drifts from the upstream Rancher API version and the divergence is not caught by envtest"
---
# Proxy Types Over Unstructured

Extracted from `docs/adr/0009-use-structured-proxy-types.md` (title
"9. Use structured proxy types", ADR status proposed, dated 2023-08-09) at commit
`d54023d5c399a5bdc95581c54255974e4ff6522a`. Evidence, quotations, and the
corroborating code paths: `res.adr-evidence-survey`, section "Evidence 1".

## Context

The operator reconciles several custom resources that Rancher owns. Importing
`github.com/rancher/rancher/pkg/apis` directly would tie the operator to that
package's fork of `client-go`, so ADR 0009 line 20 sets the constraint: operate
on those resources "without a need to import `Rancher` API packages directly".
Two ways satisfy it: dynamic unstructured objects, or a locally declared subset
of each resource.

`docs/adr/0005-rancher-integration-strategy.md` (ADR status accepted, 2023-06-01)
took the first way for the same constraint. ADR 0009 takes the second and, for
the proxied resources, reverses it. ADR 0009 links back to ADR 0005 for context
(line 20) but never names the reversal, and ADR 0005 has no forward pointer, so
this draft states the relationship rather than leaving a reader to find it.
Whether that makes this a superseding decision in the graph is the maintainer's
call; the extraction flow does not set `supersedes`.

## Decision

Declare a local proxy type for each Rancher resource the operator touches, under
`./api/rancher`, carry `kubebuilder` markers on those types so deepcopy
implementations are generated and the types satisfy the controller-runtime
`client.Object` interface, and register the proxy packages in the scheme at
operator startup, instead of reading those resources as
`unstructured.Unstructured`.

## Rationale

The rejected alternative is `unstructured.Unstructured`. ADR 0009 lines 28 to 31
give four reasons for proxy types: the generated definitions are independent of
the Rancher types, `client.Object` and its helpers become available, conversion
to and from unstructured disappears along with the risk of setting unknown API
fields, and tests can use `envtest` rather than `fakeClient`, which the ADR ties
to controller-runtime issue 2308. Direct control over the proxied specifications
is what lets the project track current Rancher API versions.

The code at the recorded commit implements this choice. The draft records that
current behaviour and stays `status: proposed` until the maintainer rules:
the markers are present
(`api/rancher/management/v3/cluster.go:33-34`,
`api/rancher/provisioning/v1/cluster.go:24-25`), the generated deepcopy files
exist in all three packages, and `main.go:78-79` registers
`provisioningv1` and `managementv3` in the scheme.

## Consequences

- Creating, watching, listing, and reading proxied Rancher resources no longer
  goes through `unstructured.Unstructured` (ADR 0009 line 37).
- Integration tests for these resources run under `envtest` (ADR 0009 line 38).
- Each proxied package carries generated deepcopy code that must be regenerated
  when a type changes: `api/rancher/k3s/v1/zz_generated.deepcopy.go`,
  `api/rancher/management/v3/zz_generated.deepcopy.go`,
  `api/rancher/provisioning/v1/zz_generated.deepcopy.go`.
- The proxy specifications are a hand-maintained subset, so a Rancher API change
  is a change the project must mirror.
- The reversal is scoped to the proxied resources. `unstructured.Unstructured`
  is still used elsewhere, for example `internal/provider/rancher.go`,
  `internal/provider/wrangler.go`, and `test/framework/*.go`.

## Bindings

`cairn onboard decisions` bound the evidence document to `docs.adr`, the node
declaring `./docs/adr`, and that is the binding on this draft. The code this
decision governs sits behind three separate `code-target` entries in the same
index run: `api/rancher/k3s/v1` bound to `api.rancher.k3s.v1`,
`api/rancher/management/v3` bound to `api.rancher.management.v3`, and
`api/rancher/provisioning/v1` bound to `api.rancher.provisioning.v1`.
```

At the end of the run this draft carried `status: proposed` and no
`ratified_by`, `receipts`, or `supersedes`. `cairn pending` in the clone listed
exactly one entry:
`dec.proxy-types-over-unstructured (age 0d, binding) nodes: docs.adr`.

### 4.2 Withdrawn draft: `dec.deletion-via-owner-references-and-imported-annotation`

Written by `cairn decision new deletion-via-owner-references-and-imported-annotation
--node docs.adr --informed-by res.adr-evidence-survey` from
`docs/adr/0003-deletion-strategy.md`, then deleted from the clone before the run
ended. Verbatim, as it stood at withdrawal:

```markdown
---
id: dec.deletion-via-owner-references-and-imported-annotation
nodes:
  - docs.adr
status: proposed
date: 2026-08-10
informed_by:
  - res.adr-evidence-survey
revisit_triggers:
  - "Kubernetes garbage collection stops being a safe basis for removing the Rancher cluster object, for example under a cross-cluster ownership model"
  - "a user needs a CAPI cluster to be re-imported after a Rancher-side deletion, which the imported annotation currently blocks"
  - "the imported annotation acquires a second meaning anywhere in the operator, so a single flag no longer distinguishes deliberate removal from a fresh import"
---
# Deletion Via Owner References And Imported Annotation

Extracted from `docs/adr/0003-deletion-strategy.md` (title "3. Deletion
strategy", status proposed, dated 2023-08-22) at commit
`d54023d5c399a5bdc95581c54255974e4ff6522a`. Evidence and quotations:
`res.adr-evidence-survey`, section "Evidence 2".

## Context

Removing a CAPI-imported cluster from Rancher has to leave the CAPI cluster and
the infrastructure it provisioned intact (ADR line 20). Without a rule, deleting
the Rancher-side object would either cascade into the CAPI cluster or be undone
immediately by the operator's own import loop.

## Decision

Own the Rancher cluster object from the CAPI cluster through the Kubernetes
owner-reference chain at creation time, and mark the CAPI cluster with the
`ClusterImportedAnnotation` (`imported="true"`) when the Rancher cluster is
deleted, so the operator does not re-import it.

## Rationale

Owner references make deletion of the Rancher object a native garbage-collection
consequence of deleting the CAPI cluster, with no bespoke cleanup path. The
annotation covers the other direction: a user deleting the Rancher cluster from
the UI wants it gone, not recreated on the next reconcile, and an annotation on
the CAPI cluster is the record that survives the deleted object.

## Consequences

- Deleting the CAPI cluster removes the Rancher cluster through Kubernetes
  garbage collection rather than through operator code (ADR line 37).
- Deleting the Rancher cluster is selective: the CAPI cluster and its
  infrastructure remain, and re-import is suppressed by annotation (ADR line 38).
- The suppression is durable. Re-importing such a cluster means removing the
  annotation, which is a manual step.
- The annotation is a single shared flag. Any second use of `imported` on a CAPI
  cluster would collide with this meaning.

## Bindings

`cairn onboard decisions` bound the evidence document to `docs.adr`, the node
declaring `./docs/adr`, and that is the binding on this draft. The code this
decision governs sits behind separate `code-target` entries in the same index
run: `internal/controllers/import_controller.go:481-529` under
`internal.controllers`, and `util/predicates/cluster_predicates.go:59` under
`util.predicates`. The constant itself,
`util/annotations/helpers.go:24-25`, is in a file the derived blueprint leaves
unowned, and `cairn scan` reports it as `CAIRN_RECONCILE_ORPHANED_FILE`.
```

Reading the remaining documents disproved the first half of its Decision
section. ADR 0008 ("Kubernetes' native garbage collector using the owner
reference chain is not a viable option due to the namespaced vs global scoped
conflict") and ADR 0011 ("Turtles will no longer manage
`provisioning.cattle.io/v1` clusters, and instead will work with
`management.cattle.io/v3` resources") retire the ownership chain for the default
import path. The annotation half survives and is in the code, but no single
document states "the annotation, not the ownership chain", so keeping the draft
would have meant handing the maintainer a decision the project never recorded.

It is withdrawn rather than handed on because a ruling spent on a disproved
premise is a ruling wasted. It is not part of
`todo.brownfield-extraction-maintainer-ruling`.

## 5. Fired-or-not verdict on every revisit trigger

Against the `revisit_triggers` frontmatter of
`dec.brownfield-extraction-mechanism`, which that artefact names as the sole
source of reconsideration conditions. All four entries, quoted from the
frontmatter, in order.

### Trigger 1: NOT FIRED

> Cairn gains a first-party inference backend with a deterministic, reviewable
> contract for decision prose

Nothing in this run changed that. `cairn onboard decisions` called no model, and
the summariser remains disabled by default with no hosted backend. Every word of
narrative in the drafts was written by the agent from evidence it read, exactly
as the mechanism intends.

### Trigger 2: FIRED, on one of its three limbs

> external validation shows that the deterministic onboard index misses a
> material ADR-like location, cannot preserve a real node binding, or produces
> too much unrelated evidence for an agent to review

Limb by limb, because they do not fire together.

**"misses a material ADR-like location": not fired.** Every decision record in
this repository is under `docs/adr/`, and the index found all 19. Grepping the
rest of `docs/` for a `## Decision` heading outside `docs/adr/` returns nothing.
The closed set was sufficient here.

**"cannot preserve a real node binding": FIRED.** Two independent observations.

First, out of the box there was no binding at all. The blueprint that
`cairn init --from-code --apply` derives declares only code directories, so all
19 ADR documents came back unbound on the first index run. The reference's
instruction for that state ("Disambiguate blueprint ownership first, or leave
it") is correct but leaves the operator to invent a node the derivation did not
produce.

Second, once a node claims `docs/adr`, every ADR binds to that one node. The
binding rule resolves the path of the evidence, so a document-derived decision
can only ever bind to the owner of the document. The nodes a decision actually
governs (`api.rancher.k3s.v1`, `api.rancher.management.v3`,
`api.rancher.provisioning.v1` for the retained draft) arrive as separate
`code-target` entries with nothing in the wire joining them to the document. The
machine-visible consequence: after the draft landed in the clone, `cairn scan`
still reported `CAIRN_PROVENANCE_NO_DECISION` for all 11 code nodes, and
reported none for `docs.adr`. The extraction satisfied provenance for the
documentation node and left the code exactly as uncovered as before.

**"produces too much unrelated evidence for an agent to review": not fired.**
All 30 entries were read in one session, the 19 documents averaging about 50
lines. The volume was reviewable.

**Recorded as a maintainer-facing finding, not absorbed.** Per this unit's
non-goals the response is a follow-up todo, not a widening of this unit:
`todo.onboard-decisions-document-code-link`.

### Trigger 3: NOT FIRED

> the onboard surface acquires a different ownership or mutation contract,
> making a decision subcommand ambiguous or unsafe

`cairn onboard decisions` mutated nothing in the clone. The only writes in the
whole run came from `cairn init --from-code --apply` and `cairn decision new`,
both of which own their writes explicitly. The no-subcommand orphan report and
the `decisions` subcommand stayed distinct.

### Trigger 4: NOT FIRED

> the pack distribution model changes so the cairn-dev reference is no longer
> shipped through the current canonical and adapter surfaces

Directly observed the other way. `cairn init --from-code --apply` installed the
agent pack into the clone (15 files, bundle 1.0.0, cli 0.9.0), and
`.claude/skills/cairn-dev/references/task-brownfield-decision-extraction.md` was
among them. The reference reached a real external repository through the shipped
path, with no manual copying.

## 6. Retained evidence report and external research artefact

### 6.1 The `cairn onboard decisions` report that produced the drafts

Verbatim output of `cairn onboard decisions` in the clone at the recorded
commit, after the two blueprint edits in section 1:

```text
Decision evidence (30 bound, 0 unbound):

--- Bound evidence ---

  docs/adr/0000-template.md [document] -> docs.adr
    Title

  docs/adr/0001-use-adrs.md [document] -> docs.adr
    1. Use ADRs to record decisions

  docs/adr/0002-use-helm.md [document] -> docs.adr
    2. Use helm charts for releases

  docs/adr/0003-deletion-strategy.md [document] -> docs.adr
    3. Deletion strategy

  docs/adr/0004-running-out-of-rancher-manager-cluster.md [document] -> docs.adr
    4. Running out of Rancher Manager cluster

  docs/adr/0005-rancher-integration-strategy.md [document] -> docs.adr
    5. Rancher Integration Strategy

  docs/adr/0006-import-strategy.md [document] -> docs.adr
    6. Cluster Import Strategy

  docs/adr/0007-rancher-turtles-public-api.md [document] -> docs.adr
    7. Rancher Turtles Public API

  docs/adr/0008-managementv3-clusters-support.md [document] -> docs.adr
    8. Management V3 clusters support

  docs/adr/0009-publish-chart-to-rancher-charts.md [document] -> docs.adr
    9. Helm chart repository

  docs/adr/0009-use-structured-proxy-types.md [document] -> docs.adr
    9. Use structured proxy types

  docs/adr/0010-migrate-to-v3-cluster-resource.md [document] -> docs.adr
    10. Manually migrate to v3 cluster resource

  docs/adr/0011-v1-to-v3-migration.md [document] -> docs.adr
    11. Migration to v3 clusters

  docs/adr/0012-clusterctl-provider.md [document] -> docs.adr
    12. Clusterctl Config resource

  docs/adr/0013-self-managed-rancher-cluster.md [document] -> docs.adr
    Self managed Rancher cluster

  docs/adr/0014-turtles-ui-installation.md [document] -> docs.adr
    Turtles UI installation

  docs/adr/0015-capiprovider-architecture.md [document] -> docs.adr
    CAPIProvider Architecture

  docs/adr/0016-capi-version-pinning.md [document] -> docs.adr
    CAPI version pinning strategy (community vs prime)

  docs/adr/0017-release-process.md [document] -> docs.adr
    17. Release Process

  api/rancher/k3s/v1 [code-target] -> api.rancher.k3s.v1
    api.rancher.k3s.v1

  api/rancher/management/v3 [code-target] -> api.rancher.management.v3
    api.rancher.management.v3

  api/rancher/provisioning/v1 [code-target] -> api.rancher.provisioning.v1
    api.rancher.provisioning.v1

  api/v1alpha1 [code-target] -> api.v1alpha1
    api.v1alpha1

  examples [code-target] -> examples
    examples

  internal/controllers [code-target] -> internal.controllers
    internal.controllers

  internal/controllers/clusterctl [code-target] -> internal.controllers.clusterctl
    internal.controllers.clusterctl

  internal/provider [code-target] -> internal.provider
    internal.provider

  internal/sync [code-target] -> internal.sync
    internal.sync

  test [code-target] -> test
    test

  util/predicates [code-target] -> util.predicates
    util.predicates
```

### 6.2 The external research artefact the drafts cite

`meta/research/adr-evidence-survey.md` in the clone, hand-authored before the
drafts as the reference requires, `method: primary` because the evidence is
first-hand. Its disposition table covers all 19 documents; its final section
records what the index could not tell the drafter. Reproduced verbatim so the
drafts' `informed_by` is not a dangling pointer once the clone is gone:

```markdown
---
id: res.adr-evidence-survey
nodes:
  - docs.adr
  - api.rancher.k3s.v1
  - api.rancher.management.v3
  - api.rancher.provisioning.v1
  - internal.controllers
  - util.predicates
method: primary
date: 2026-08-10
---

# Decision evidence selected from the Turtles ADR set

Primary evidence, read first-hand at commit
`d54023d5c399a5bdc95581c54255974e4ff6522a`. Produced by
`cairn onboard decisions --json` over a blueprint derived by
`cairn init --from-code --apply`, then read at the paths the index reported.

The index returned 30 bound entries and 0 unbound: 19 `document` entries under
`docs/adr/`, all bound to `docs.adr`, and 11 `code-target` entries bound to the
discovered code nodes. It returned no `readme-section` entry (the root
`README.md` is HTML-wrapped and carries no Markdown heading named Decision,
Rationale, or Invariant) and no `invariant-comment` entry (no `// invariant:`
marker exists in the tree).

Every one of the 19 documents was read at its reported path before drafting.
The per-document disposition is in the last section. One document survived that
review as a settled, code-confirmed choice and became the single draft.

The draft is created from its `document` entry, so it binds to `docs.adr`, the
node that owns the document. The code nodes listed below are cited, not bound:
they come from separate `code-target` entries in the same index run and are
recorded so a reader can reach the code the decision constrains.

## Evidence 1, drafted: proxy types instead of unstructured objects

Document evidence, `docs/adr/0009-use-structured-proxy-types.md`, bound to
`docs.adr`, title "9. Use structured proxy types", ADR status proposed,
dated 2023-08-09. Draft: `dec.proxy-types-over-unstructured`.

Line 24, Decision section, verbatim:

> The `rancher-turtles` operator will be using `kubebuilder`
> [annotations](https://kubebuilder.io/reference/markers) on specified for the
> `Rancher` proxy types located under the `./api/rancher` directory, to generate
> deep copy definitions and therefore allow specified resources to match the
> `Object` [interface](https://github.com/kubernetes-sigs/controller-runtime/blob/main/pkg/client/object.go#L45)
> provided by the controller-runtime.

Line 37, Consequences section, verbatim:

> We no longer use **unstructured.Unstructured** when creating, watching,
> listing and reading resources using controller-runtime, instead we register
> our proxy `Rancher` API types in the schema builder directly on operator
> startup.

The named alternative is unstructured objects, and line 20 records the
constraint the choice protects: operate on Rancher custom resources "without a
need to import `Rancher` API packages directly".

This reverses an earlier document in the same directory.
`docs/adr/0005-rancher-integration-strategy.md`, ADR status accepted, dated
2023-06-01, decided the opposite for the same constraint:

> The decision is that we will use **unstructured.Unstructured** so that we
> don't have to depend of the Rancher Manager apis package.

ADR 0009 links back to ADR 0005 for context (line 20,
`./0005-rancher-integration-strategy.md#context`) but never names the reversal,
and ADR 0005 carries no forward pointer to ADR 0009.

Code-target entries from the same index run that this choice governs:

| Entry | Bound node |
|---|---|
| `api/rancher/k3s/v1` | `api.rancher.k3s.v1` |
| `api/rancher/management/v3` | `api.rancher.management.v3` |
| `api/rancher/provisioning/v1` | `api.rancher.provisioning.v1` |

The code confirms ADR 0009 is the live position, which is why this document was
drafted and ADR 0005 was not:

- kubebuilder markers are on the proxy types
  (`api/rancher/management/v3/cluster.go:33-34`,
  `api/rancher/provisioning/v1/cluster.go:24-25`,
  `api/rancher/management/v3/setting.go:22-23`) and on the package
  (`api/rancher/management/v3/groupversion_info.go:19`,
  `api/rancher/provisioning/v1/groupversion_info.go:19`);
- the generated deepcopy files exist:
  `api/rancher/management/v3/zz_generated.deepcopy.go`,
  `api/rancher/k3s/v1/zz_generated.deepcopy.go`,
  `api/rancher/provisioning/v1/zz_generated.deepcopy.go`;
- the proxy packages are registered in the scheme at startup,
  `main.go:78-79` (`provisioningv1.AddToScheme`, `managementv3.AddToScheme`).

`unstructured.Unstructured` still appears elsewhere in the tree
(`internal/provider/rancher.go`, `internal/provider/wrangler.go`,
`test/framework/*.go`), so the reversal is scoped to the proxied Rancher
resources, not to the whole operator.

## Evidence 2, considered and withdrawn: the deletion strategy

`docs/adr/0003-deletion-strategy.md`, bound to `docs.adr`, ADR status proposed,
dated 2023-08-22, was drafted from its Decision section and then withdrawn
during the full read of the other 18 documents. It is recorded here because the
withdrawal is the most useful thing this run learned.

Lines 26 and 28, Decision section, verbatim:

> **Ownership Chain:** Rancher cluster is associated with CAPI cluster (CAPI
> cluster owns the Rancher cluster) through the owner references chain during
> their creation process.

> **Cluster Annotation:** When deleting a Rancher cluster, the operator will
> annotate the corresponding CAPI cluster with the `ClusterImportedAnnotation`
> (`imported=“true”`) annotation. This annotation will prevent automatic
> re-import of the CAPI cluster after corresponding Rancher cluster deletion.
> The underlying infrastructure provisioned by CAPI is left intact.

Two later documents contradict the first half, and neither is linked from
ADR 0003.

`docs/adr/0008-managementv3-clusters-support.md`, Consequences, verbatim:

> Kubernetes' native garbage collector using the owner reference chain is not a
> viable option due to the namespaced vs global scoped conflict. This means that
> there needs to be a custom logic to manage deletion of resources.

`docs/adr/0011-v1-to-v3-migration.md`, Consequences, verbatim:

> Turtles will no longer manage `provisioning.cattle.io/v1` clusters, and
> instead will work with `management.cattle.io/v3` resources.

The owner-reference half therefore describes a path the project has left. The
annotation half is still live (ADR 0011 refers to the ADR 0003 "deletion
strategy annotation" as behaviour it must avoid triggering) and is visible in
the code:

| Path | Bound node | What it shows |
|---|---|---|
| `internal/controllers/import_controller.go:481-529` | `internal.controllers` | sets `turtlesannotations.ClusterImportedAnnotation` on the CAPI cluster |
| `util/predicates/cluster_predicates.go:59` | `util.predicates` | skips a cluster that already carries the annotation |
| `util/annotations/helpers.go:24-25` | none | defines `ClusterImportedAnnotation = "imported"` |

`util/annotations/helpers.go` is one of the 13 files `cairn scan` reports as
`CAIRN_RECONCILE_ORPHANED_FILE` against this derived blueprint, so the constant
the surviving half of the decision names sits in a file no node owns.

A faithful decision here would be "the annotation, not the ownership chain",
which is not what any single document says. Drafting it would mean writing a
decision the project never recorded, so this run stopped at recording the
conflict.

## Disposition of all 19 indexed documents

| Document | ADR status | Disposition |
|---|---|---|
| `0000-template.md` | template | Not a decision. Section scaffolding with placeholder text. |
| `0001-use-adrs.md` | proposed | Process choice about recording decisions, not about the system. |
| `0002-use-helm.md` | proposed | Release packaging, overtaken in practice by 0009-publish, 0016, and 0017. The chain, not this document, is the current position. |
| `0003-deletion-strategy.md` | proposed | Drafted, then withdrawn. See Evidence 2. |
| `0004-running-out-of-rancher-manager-cluster.md` | proposed | Reads like a real choice (two clients, a `rancher-kubeconfig` flag), but no `rancher-kubeconfig` flag and no `RancherClient` type exist in the Go source at this commit. The only `rancherKubeconfig` identifiers are e2e test locals in `test/framework/rancher_helpers.go` and `test/e2e/suites/v2prov/v2prov_test.go`, unrelated to the two-client design. Documented, not implemented. |
| `0005-rancher-integration-strategy.md` | accepted | A real choice, reversed for the proxied types by 0009. Recorded inside Evidence 1. |
| `0006-import-strategy.md` | accepted | A real choice (replicate the UI import steps, reject the v2prov import feature). Candidate for a later pass. |
| `0007-rancher-turtles-public-api.md` | accepted | A real choice (`CAPIProvider` under `turtles-capi.cattle.io`), partly reworked by 0015. Needs the 0015 chain read with it. |
| `0008-managementv3-clusters-support.md` | proposed | A real choice (label-based linking for v3 clusters). Quoted in Evidence 2 for the bound it sets. |
| `0009-publish-chart-to-rancher-charts.md` | proposed | Chart hosting choice, reopened by 0016 and 0017. Packaging mechanics. |
| `0009-use-structured-proxy-types.md` | proposed | **Drafted.** See Evidence 1. Note the duplicate `0009` number in this directory. |
| `0010-migrate-to-v3-cluster-resource.md` | proposed | Manual migration runbook; its own note names 0011 as the preferred method. Superseded procedure. |
| `0011-v1-to-v3-migration.md` | proposed | Automatic migration under a feature flag. Quoted in Evidence 2 for the scope it sets. |
| `0012-clusterctl-provider.md` | proposed | A real choice (`ClusterctlConfig` singleton CRD) whose future 0016 explicitly reopens. |
| `0013-self-managed-rancher-cluster.md` | proposed | Bootstrap-and-pivot direction with a staged plan and unmet prerequisites. Roadmap, not a settled constraint. |
| `0014-turtles-ui-installation.md` | proposed | A real choice (UI extension as a Helm chart dependency). Packaging; no code node in this blueprint owns it. |
| `0015-capiprovider-architecture.md` | accepted | A real choice (embed CAPI Operator as a library) and realised: `internal/controllers/operator_reconciler.go:36-37` imports `sigs.k8s.io/cluster-api-operator/controller`, lines 64-80 build `controller.GenericProviderReconciler` around the `CAPIProvider` type, and `main.go:251-253` registers the wrapper. The strongest candidate not drafted in this run, held back to keep the run to one reviewable draft; it needs the 0007 chain read with it. |
| `0016-capi-version-pinning.md` | proposed | Conditional and open: "If the Go build tag approach for pinning providers doesn't work well, switch to pinning with Helm charts", plus an Open questions section. A decision in progress. |
| `0017-release-process.md` | proposed | Release mechanics in two phases, one of them future. Not a code-governing constraint. |

Eleven of the 19 record a genuine choice. One was drafted. The reasons for
holding the other ten are in the table: five are packaging or release mechanics
that no code node in this derived blueprint owns, three are superseded,
in-progress, or documented but not implemented, and two are live, code-confirmed
choices held back only to keep this run to one reviewable draft.

## What the index could not tell the drafter

Recorded here because it is a property of the mechanism, not of this project.

1. **No status.** The index reports a document's title, never its ADR status
   line. Nine documents here say "Status: proposed" while describing behaviour
   the code already implements, and 0016 says proposed and means it. Only
   reading the file separates them.
2. **No supersession.** The index reports 19 sibling documents in one flat list.
   That 0009 reverses 0005, that 0008 and 0011 bound 0003, and that 0011
   supersedes 0010 are all invisible in the wire and recoverable only by reading
   all 19. Drafting from a single document without that reading would have
   produced a wrong decision here, and did once before it was withdrawn.
3. **No document-to-code link.** A `document` entry binds to whoever declares
   its directory, so every ADR here binds to `docs.adr`. The code nodes a
   decision governs come from separate `code-target` entries with nothing
   joining the two.
4. **No implementation check.** ADR 0004 describes a `rancher-kubeconfig` flag
   and a `RancherClient` that exist in no Go source at this commit. The index
   reports the document as evidence either way.
```

## 7. What the run says about the mechanism

Three observations the maintainer should have alongside the draft. Only the
second fires a trigger.

1. **The reference's prerequisite assumes a System block the brownfield entry
   point never writes.** Step 0 of `task-brownfield-decision-extraction.md` says
   to check that "the System block declares both artefact directories". The
   blueprint `cairn init --from-code --apply` writes has no System block at all:
   it is a flat list of discovered Containers and Modules. Wrapping them in a
   System was hand edit 2 of section 1. Follow-up:
   `todo.brownfield-extraction-reference-gaps`.

2. **Document evidence binds to the documentation node, never to the code.**
   Trigger 2, above. Follow-up:
   `todo.onboard-decisions-document-code-link`.

3. **The index carries no status and no supersession.** Nine of these ADRs say
   "Status: proposed" while describing shipped behaviour, one says proposed and
   means it, and three chains (0005 reversed by 0009, 0003 bounded by 0008 and
   0011, 0010 superseded by 0011) exist only in the prose. An agent drafting
   from a single entry without reading the whole directory produces a wrong
   decision, which is what happened here before the second draft was withdrawn.
   This is not a defect in the closed set: reporting a document's status field
   would only repeat what the document claims, and these documents claim wrong.
   It is an argument that the reference's read-each-candidate step is
   load-bearing and should say why. Follow-up:
   `todo.brownfield-extraction-reference-gaps`.
