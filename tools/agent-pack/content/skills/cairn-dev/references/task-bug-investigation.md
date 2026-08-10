# Bug investigation

Goal: locate the defect, prove it with a failing test, fix the cause.

## Query sequence

```bash
cairn locate <Symbol>                    # exact definition sites for a name in the report
cairn get <node> --symbols --json        # what the owning node declares
cairn neighbourhood <node> --include-changes   # blast radius, and whether a change is already touching it
cairn rationale <node>                   # accepted decisions that constrain the fix
cairn deps <node> --direction in         # who calls in, so you know what the fix can break
```

Run them in that order. `locate` takes an exact symbol name, so start from a name
the bug report actually contains (a function, type, error string, or CLI flag). If
the report names no symbol, start at `cairn context` and narrow by area instead.

## From graph to source

The graph tells you who owns what and what depends on it; it does not tell you
what the code does. Once you have the owning node, read the symbol spans it
declares and follow call chains with the language server; the discipline is in
`graph-navigation.md`.

## Fix it

1. Write the failing test first. Red before green. The test names the observable
   behaviour, not the internal call.
2. Fix the cause, not the symptom. A suppressed warning or a special-cased input
   is not a fix.
3. Check the inbound dependents from the query above; a fix that changes a public
   interface changes their contract too.
4. If the cause was a wrong assumption encoded in the blueprint (a missing edge, a
   node owning the wrong files), correct the blueprint in the same change.

## Verify

Run the router's gate plus the repository's own gates, and prove the fix at
the boundary the bug was reported at: if it was reported as a CLI
misbehaviour, run the command.
