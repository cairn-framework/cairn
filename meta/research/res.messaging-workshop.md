---
id: res.messaging-workshop
nodes:
  - cairn.root
date: 2026-07-10
method: primary
---

# Messaging workshop research brief (grandslam-offer workflow)

Living research document for Cairn's public messaging (README, landing page).
Built during a full grandslam-offer workshop run against the existing assets.
Reuse this brief for future messaging rounds; update personas after each
adversarial panel (persona evolution protocol).

## Primary source: real user feedback (reddit DM, 2026-06-28)

A real user who evaluated Cairn cold: "your ROI is not clear for new user....
Specially if ask them to maintain contracts, architecture, modules and stuff.
Like imagine if they do all this what do they actually gain?? Fewer
Architecture mistake? Better context? Less drift?? All of this is hard to
measure. [...] And devs are lazy." The maintainer's reply captured the real
usage model: "the developer doesnt really need to manage the contracts and
all of that [...] you converse with the agent [...] Then you end up with
leaner context usage, focused results with agentic coding."

This exchange triggered the agent-first repositioning (PR #250).

## Customer voice (web research, 2026-07-10)

Dominant pain vocabulary, verbatim, with sources:

- "goldfish memory": "Claude Code has the memory of a goldfish and the
  confidence of a 10x engineer" (r/ClaudeCode thread title).
- "Every AI coding agent [...] starts every session completely blind. You
  re-explore the entire codebase every session... you burned half your
  context window on reconnaissance before you wrote a single line of code."
  (dev.to/creatman, "The context problem nobody talks about", 2026)
- "Claude systematically ignores CLAUDE.md instructions and destructively
  modifies prohibited code areas" (github.com/anthropics/claude-code#5516)
- "Rules in prompts are requests. Hooks in code are laws." (comment on
  dev.to/minatoplanb, "I wrote 200 lines of rules for Claude Code. It
  ignored them all.")
- "The real problem isn't memory... it's that there's no system of record"
  (HN comment, news.ycombinator.com/item?id=46426624)
- "token anxiety" / "range anxiety" (HN, item 47586176)
- Workarounds are self-defeating: handoff notes, progress.md, repomix dumps
  produce "context bloat" that eats the window they were meant to save.

Full quote corpus with URLs: session research runs CustomerVoice and
CompetitorPromises (2026-07-10). Key phrases adopted into copy vocabulary:
"starts every session blind", "system of record", "rules are requests,
hooks are laws" (concepts, not verbatim lifts).

## Competitor promise landscape (web research, 2026-07-10)

| Camp | Tools | Promise | Gap |
|---|---|---|---|
| Docs agents might read | CLAUDE.md/AGENTS.md, ADRs, log4brains, structurizr, arc42 | consistency via prose | prose cannot fail a build; rots silently |
| One-shot context aids | repomix, gitingest, aider repo map | whole repo as AI-friendly context | reflects code only, no intent, no gate, stale on next commit |
| Memory layers | mem0 and memory MCPs | long-term agent recall | recall not enforcement; hallucinated or stale memories |
| Agent governance | Mneme HQ, unkode, Archyl, MS Agent Governance Toolkit | stop agents violating the system | SaaS motion, rules files without a map, or one-directional |

Emptiest ownable slot (synthesis): a free, repo-native architecture map that
is itself the enforceable truth, reconciled both ways against real code and
gating drift at commit/CI. Mneme's line "AI coding agents can complete the
task and still violate the system" independently validates the category.

Growth signals: 84% of developers use AI coding tools (Stack Overflow 2025);
r/ClaudeCode ~281k members growing ~2.9k/day (FreeSubStats 2026).

Congregation points: r/ClaudeCode, r/ClaudeAI, r/CursorAI, r/ChatGPTCoding,
r/LocalLLaMA, Cursor and Anthropic Discords, Latent Space, Simon Willison.

## Personas (drafts pending maintainer validation)

1. Solo agent-native builder. Ships side projects with Claude Code on a
   capped plan. Pain: token anxiety, sessions starting blind, agent rewrites
   working code it forgot. Burned by upkeep tools (wikis, ADR folders).
   Adopts only if the first session visibly pays. Fear: another artefact to
   babysit.
2. Burned tech lead. Small team, several devs and agents in one repo. Pain:
   no system of record; agents made contradictory architectural calls;
   CLAUDE.md ignored; a shipped layering violation cost a weekend. Wants
   enforcement agents cannot bypass (CI). Fears: a wrong map defended by the
   gate, false-positive noise, maintenance burden.
3. AI-engineering explorer. Early adopter, follows Latent Space and similar,
   tries new agent tools weekly, files feedback, stars repos. Wants a novel
   mechanism that demonstrably works; allergic to overclaiming READMEs.

## Value equation (Phase 3, panel-calibrated)

Dream 7-8, Likelihood 5-6 (weakest: no third-party proof), Time 6-8
(brownfield first map requires review+archive step), Effort 6-7 (manual
AGENTS.md paste is the weak link). Panel-verified product gaps filed as
todos: todo.map-orphaned-section-severity-sort,
todo.brownfield-one-step-first-map, todo.init-wire-agents-md-flag.

## Workshop decisions ratified so far

- Agent-first positioning: the agent authors and maintains the map; the
  developer converses. Hand-authoring is the secondary path.
- Category-of-one framing: falsifiability (the map can be proven wrong).
- Named guarantees: Clean Exit; Nothing-Leaves-Your-Machine.
- No scarcity or urgency devices: fake scarcity destroys developer trust.
- High-Value Leader position: never lead with "free".
- All shipped copy must match binary behaviour exactly; overclaims found by
  panels are softened in copy and filed as product todos.
