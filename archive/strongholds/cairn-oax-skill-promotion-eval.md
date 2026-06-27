# cairn-oax skill-promotion eval evidence (2026-06-27)

Full evidence behind res.cairn-oax-skill-promotion and dec.explore-teaches-provenance.

## Method

A fresh `cairn init`'d test bed was built (System "Shop" with modules shop.auth
and shop.cart, real Rust source under each path, one accepted decision
dec.opaque-session-tokens carrying rationale on shop.auth). Two state-investigation
prompts were run through `omp -p ... --mode json`, each under two arms:

- pack: the shipped five-skill pack (`--skills=cairn-dev,cairn-explore,cairn-propose,cairn-apply,cairn-archive`)
- pack + skill: the same pack plus the candidate `cairn-first-state-investigation`

Tool calls were parsed from the `tool_execution_start` events; answer correctness
was checked by keyword. Sample size was one run per arm per scenario (directional,
not definitive).

## Results

| scenario | arm | skills triggered | used `cairn rationale`? | manual meta/src reads | correct |
|---|---|---|---|---|---|
| provenance: "what decisions shaped shop.auth" | pack | cairn-explore | no | 3 | yes |
| provenance | pack + skill | cairn-explore, cairn-first-state-investigation | yes | 2 | yes |
| health check | pack | cairn-explore | n/a | 3 | yes |
| health check | pack + skill | cairn-explore, cairn-first-state-investigation | yes (extra) | 1 | yes |

An earlier pass compared against `--no-skills` and showed a large but misleading
lift (~36% fewer tool calls); that baseline does not represent a real cairn-init'd
repo, which always carries the pack.

## Interpretation

The only durable, non-overlapping lift was on the provenance question: without the
candidate skill the agent never reached `cairn rationale` and read the decision
file by hand. The structural cause, confirmed by inspecting the shipped
`cairn-explore` SKILL.md, is that its command list covered structure and health
only (status, get, neighbourhood, files, islands, lint, check, ui) and named no
provenance command. The health-check scenario showed no lift; `cairn-explore`
already covered it. Conclusion: merge the provenance path into `cairn-explore`
rather than ship a fourth overlapping state skill.
