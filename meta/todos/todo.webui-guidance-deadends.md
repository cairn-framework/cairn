---
node: cairn.ui
status: done
created: 2026-07-17
---

# Webui Guidance Deadends

## Problem
The findings empty state gave no freshness signal or next command, and the command palette did not show the Enter affordance for its highlighted result.

## Fix
Expose the server-owned timestamp for the cached scan, show it with the findings CTA, and add Enter hints for highlighted results and the report action.

Tracking: gh:#305
