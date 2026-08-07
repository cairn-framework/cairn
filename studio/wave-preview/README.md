# Wave preview

Renders today's rung 3 dispatch wave from committed state (map.json plus
meta/todos), following dec.rung-three-coordination-substrate: containment
closure write-sets, component-boundary disjointness, one hotspot permission
holder per wave, clause 5 sentences. Read-only projection; no driver, no
buttons, nothing dispatches.

    python3 studio/wave-preview/generate.py
    open studio/wave-preview/index.html

Styling comes from docs/design-system/fonts.css and tokens.css, per the
design-system rule in AGENTS.md.
