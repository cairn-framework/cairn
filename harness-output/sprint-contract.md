# Sprint contract — Cairn Graph Explorer greenfield run

Testable success criteria. Evaluation is browser-based on rendered mocks only.

1. Both mocks open directly from file:// with no console errors and no network requests.
2. Real data: header shows 25 nodes and 27 edges from map.json; the graph canvas renders all 25 nodes and all 27 dependency edges; the inspector shows a real node (cairn.brownfield or richer) with its real decision artefact title, id and date; the findings surface shows the real CAIRN_SPEC_RULE_UNIMPLEMENTED info finding.
3. All four zones present: status header, graph canvas, inspector, command surface (search or filter plus view hints).
4. Node states are legible: synced is the calm default; the visual system demonstrably reserves distinct treatments for ghost and orphaned (shown in a legend or by simulated example labelled as legend).
5. At 1440px nothing overlaps or clips; layout is intentional at a desktop viewport. 390px behaviour must degrade gracefully (no horizontal scroll of the page frame).
6. No anti-goal violation from spec.md (slop patterns, pure black or white, toy data).
7. Copy is plain language, British spelling, no em-dashes.
8. Ship threshold per workflow defaults: weightedAverage >= 7.0 with all four criteria >= 7, else iterate per decision table.
9. Zone rubric recorded per mock: header, graph canvas, inspector, command surface, each scored 1-10 on hierarchy, density, colour discipline, state clarity.
