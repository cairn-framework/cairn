---
id: dec.hook-git-install
nodes: [cairn.kernel.cli]
status: accepted
date: 2026-07-12
---

# Hook Git Install

## Context

Cairn's reconciliation hooks (`cairn hook structural|interface|tension|all`)
enforce structural integrity at commit boundaries, but users had to wire them
into Git manually. This left the commit gate absent in otherwise Cairn-managed
repositories, and the manual step was error-prone.

## Decision

Add explicit `cairn hook install`, `cairn hook status`, and `cairn hook
uninstall` lifecycle commands. Installation writes a marker-commented shell
script that invokes `cairn hook all`. The marker (`# Managed by Cairn. Do not
edit.`) is the ownership signal: install refuses to replace any unmarked
entry, and uninstall removes only marked entries.

Key design choices:

1. **Dispatch before scanner.** Lifecycle commands resolve Git paths via `git
   rev-parse`, not via the blueprint scanner. This allows installation in
   repositories with no blueprint at all (e.g., a freshly initialized project).

2. **Symlink-aware ownership.** Existence checks use `symlink_metadata`, not
   `Path::exists()`, so a dangling symlink is treated as an occupied entry and
   refused rather than silently written through.

3. **Repository root, not CWD.** `git_root()` resolves the true repository
   top-level via `git rev-parse --show-toplevel`, so the `.pre-commit-config.yaml`
   conflict check and hook-path resolution work correctly when invoked from a
   subdirectory.

4. **Platform-gated permissions.** The Unix executable-bit chmod is behind
   `#[cfg(unix)]` to preserve Windows compilation.

5. **Explicit opt-in.** `cairn init` never installs hooks silently. Users run
   `cairn hook install` when they want the gate.

## Rationale

The marker-based ownership model is simple, human-readable, and survives manual
inspection. Symlink awareness prevents a class of silent-clobber bugs. Git-root
resolution handles the common case of running Cairn from a project subdirectory.
Platform gating keeps the Windows CI target green without sacrificing the Unix
executable bit that makes the installed hook work.

## Trade-offs

- The marker is textual, not cryptographic. A user who copies the exact comment
  into their own hook would see it treated as Cairn-owned. This is acceptable:
  the marker protects against accidental clobbering, not adversarial tampering.
- Windows hooks do not get an executable bit. Git on Windows uses different
  hook-execution semantics (shell scripts run via Git's bundled bash), so the
  file is written regardless; the chmod is skipped.
