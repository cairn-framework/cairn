# CAIRN Governance Pack for Gas City

Reference adapter pack that makes CAIRN commands available as Gas City formulas.

## Install

1. Install CAIRN (the `cairn` binary must be on `$PATH`):
   ```bash
   cargo install --path .
   # or via Homebrew once published
   ```

2. Install this pack into your Gas City city:
   ```bash
   ln -s $(pwd)/adapters/gascity ~/.gc/packs/cairn-governance
   # or copy if you prefer
   cp -r adapters/gascity ~/.gc/packs/cairn-governance
   ```

3. Import the pack in your `city.toml`:
   ```toml
   [packs]
   cairn-governance = "~/.gc/packs/cairn-governance"
   ```

4. Verify formulas are visible:
   ```bash
   gc formula list
   ```

## Formulas

| Formula | Purpose | Exit semantics |
|---|---|---|
| `cairn-reconcile` | Run reconcilers and report findings | 0 = clean, 1 = advisory, 2 = blocking |
| `cairn-lint` | Lint blueprint and contracts | 0 = clean, 1 = advisory, 2 = blocking |
| `cairn-drift-gate` | Strict scan; fails on any warning or error | 0 = clean, 2 = blocking |
| `cairn-onboard` | Suggest blueprint entries for orphaned files | 0 = suggestions produced, 1 = none found |

Each formula shells out to the corresponding `cairn` command and preserves its exit code so Gas City can branch on results.

## Architecture

This pack is thin integration glue. CAIRN remains a standalone tool; this pack only exposes its CLI surface to Gas City's formula dispatcher. If you replace Gas City with another orchestrator, only this directory needs re-implementation. The CAIRN core and integration contract stay unchanged.
