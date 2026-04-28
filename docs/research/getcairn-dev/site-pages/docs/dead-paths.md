# Why Dead Paths Matter

**Source:** https://www.getcairn.dev/docs/dead-paths
**Captured:** 2026-04-28

## Overview

Most engineering documentation reveals what was built, but fails to capture what wasn't. The rejected alternatives that shaped the final system. Cairn addresses this gap by treating abandoned design paths as first-class knowledge artifacts.

## The Pruned Alternatives Thesis

Every system emerges from a series of branching decisions. At each decision point, engineers weigh options before selecting one path and discarding others. These rejected alternatives represent valuable engineering knowledge, including:

- **Rejection rationale**: specific technical, economic, or scheduling factors
- **Viability conditions**: circumstances that could make alternatives worth reconsidering
- **Design dependencies**: relationships that influenced the decision

## The Knowledge Problem

Traditional documentation scattered this decision history across:

- Trade study reports
- Meeting notes
- Email archives
- Team members' recollections

Within months, the reasoning becomes inaccessible. New team members lack context for existing design choices. Stakeholders propose previously-evaluated alternatives, triggering redundant analysis and wasted effort.

## Cairn's Solution

The Dendritic Lens elevates pruned alternatives through:

- Visual prominence in the system tree with clear distinction markers
- Structured metadata including: prune reasons, first principles, evaluation timing, decision classifications, and cross-dependencies

This approach preserves engineering decision history permanently, enabling informed what-if analysis and preventing unnecessary re-litigation of settled choices.
