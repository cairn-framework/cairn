# Design: agent-guidance-baseline

## Approach

Run a preregistered, randomised, paired, three-arm navigation study at Cairn
revision `24a328f` with OMP `17.0.8`, model
`openai-codex/gpt-5.6-sol`, medium thinking, a 180-second limit, and only the
`read`, `grep`, `glob`, and `bash` tools. The development corpus contains four
read-only tasks: locate and impact-map tasks in BurntSushi/ripgrep
`4649aa9700619f94cf9c66876e9549d83420e16c` and pallets/flask
`7fff56f5172c48b6f3aedf17ee14ef5c2533dfd1`.

The arms differ only in navigation treatment:

1. `target`: repository source and generic navigation tools.
2. `cairn`: the target arm plus the pinned Cairn binary and project map.
3. `pack`: the Cairn arm plus the canonical Cairn development skill.

Every candidate response is graded blind by two independent graders against
the frozen per-task facts and three two-point components in
`manifests/development.json` inside
`archive/strongholds/agent-guidance-baseline/evidence.tar.gz`. Disagreements
use a stricter evidence-supported adjudication. The archive's
`manifests/pilot-order.json` and `manifests/final-order.json` record the
deterministic paired order. After the three-trial pilot, its
`analysis/sample-size-audit.json` computed
$n=\max(3,\lceil((1.96+0.84)s_d/1)^2\rceil)$ independently for
target-minus-Cairn and Cairn-minus-pack. The larger result required one further
complete paired trial per task, producing four trials per task and arm.

## Failure accounting

The first 36 assigned runs failed before model execution because the isolated runner omitted OMP authentication state. They remain the primary intention-to-treat cohort. A protocol amendment fixed only session authentication isolation. The 48 later valid runs are a secondary engaged-run analysis and never replace the failed cohort.

## Confidentiality

The sealed confirmation split, prompts, ground truth, and runner authentication database are excluded from the repository. Development prompts, commitments, order manifests, sanitised packets, grades, adjudications, and compact run records are archived.

## Repository changes

- Extend `meta/research/agent-experiment-linklint.md`, the existing linked
  primary agent-experiment record, with the baseline method, results, failure
  accounting, and reversible hypotheses.
- Add the compact public study bundle under
  `archive/strongholds/agent-guidance-baseline/`.
- Add no production query, router, pack, or runtime code.

## Interpretation boundary

A one-point change on the six-point quality scale is the smallest effect of interest. Any smaller observed difference is descriptive only. Follow-up hypotheses must be reversible and remain implementation-neutral.
