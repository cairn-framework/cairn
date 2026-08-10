---
id: src.turtles-adr-run
file: https://github.com/rancher/turtles/tree/d54023d5c399a5bdc95581c54255974e4ff6522a
verification: external
type: repo
date: 2026-08-10
---

# rancher/turtles at commit d54023d5, the external repository for the extraction run

Apache 2.0 Go repository: the Rancher Turtles operator, which integrates Cluster
API with Rancher Manager. Selected for `res.brownfield-extraction-external-run`
because it is a real third-party project (not a fixture, not a cairn tree) whose
19 ADRs are enough evidence to force real document selection while staying
reviewable in one session.

`verification: external` rather than `verified`: this repository holds none of
those bytes. The run worked in a shallow clone under `/tmp`, which is scratch
space, not durable evidence, and can be deleted at any time; the research
artefact carries everything the run produced. `file` is therefore the pinned
tree URL, and every path and line quoted in the research artefact is quoted
against that commit.

## What was cloned

```bash
git clone --depth 1 https://github.com/rancher/turtles.git
git rev-parse HEAD    # d54023d5c399a5bdc95581c54255974e4ff6522a

# The clone took the default-branch tip, which was that SHA at run time. A
# replay should fetch the commit instead, because the branch has moved since:
#   git fetch --depth 1 origin d54023d5c399a5bdc95581c54255974e4ff6522a
```

Shape at that commit, as counted in the clone:

- 124 Go files, plus `charts/`, `config/`, `hack/`, `test/`, and `examples/`.
- 19 Markdown files under `docs/adr/`, one of them the ADR template, numbered
  0000 to 0017 with `0009` used twice (`0009-publish-chart-to-rancher-charts.md`
  and `0009-use-structured-proxy-types.md`).
- No `docs/decisions/` directory.
- A root `README.md` that opens with HTML banner markup and carries no Markdown
  heading whose whole text is Decision, Rationale, or Invariant.
- No `// invariant:` or `# invariant:` comment anywhere in the tree.

## Licence

Apache License 2.0 (`LICENSE` at the pinned commit). Quotations in
`res.brownfield-extraction-external-run` are short excerpts for provenance.
