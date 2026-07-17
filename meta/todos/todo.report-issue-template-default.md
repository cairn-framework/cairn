---
node: cairn.ui
status: done
created: 2026-07-17
---

# Report Issue Template Default

gh:#415

## Problem

The webui "Report an issue" button (`openReportIssue`,
src/ui_assets/utils.js) opened `issues/new?labels=feedback&title=...`,
a blank issue that bypassed `.github/ISSUE_TEMPLATE/bug-report.yml`.
The crash panic hook in src/report.rs did the same.

## Fix

- The webui button and the crash hook open
  `issues/new?template=bug-report.yml` with the `version` field
  prefilled; the crash link also prefills `what-happened` with the
  panic context, the webui link seeds it with a short report header.
  No `labels` query parameter: the template applies the `bug` label and
  the URL parameter needs triage permission.
- `cairn feedback` keeps its freeform blank issue: the bug form's
  required "what you expected" field does not fit open-ended friction.
  The chooser copy in .github/ISSUE_TEMPLATE/config.yml explains this.

Tests: `test_crash_issue_url_prefills_bug_form_fields`,
`test_issue_url_builds_exact_feedback_url` (src/report.rs).
