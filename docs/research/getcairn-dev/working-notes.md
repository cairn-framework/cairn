# Working notes: getcairn.dev research

Transient field notes. Each numbered entry corresponds to a file in `screenshots/`. The "What this is" line is a one-line label. The transcription is verbatim or near-verbatim text seen in the image, plus structural annotation. These notes ground the synthesis docs (01 through 10). When the user supplements with new screenshots, append rather than restructure.

This file is committed deliberately; it is the audit trail for which UI text appears where. Future scouts can verify the synthesis against this file without re-reading every image.

---

## Verified UI text transcriptions

### 01-round3-shape-the-starting-model.png

What this is: round 3 of the AI clarifying interview, header view.

Header chip: `ROUND 3 OF 3`. Page title: `Shape the starting model`. Page subtitle: `Answer focused questions so Cairn can reduce ambiguity and generate a stronger first-pass architecture.` Right-side meta panel shows `PROGRESS 0 / 3` and `ROUND 3 (Q [...]) READINESS QUESTIONS REVIEWED · 5 NEW`. Above the right rail: `WORKING SYSTEM BRIEF`, then a small black header `Offshore Unmanned Survey System` with body text describing the system as an offshore unmanned survey vessel (USV) paired with a light work-class ROV deployed via a stern ramp, operated from a shore-based Remote Operations Centre (ROC) over LEO satellite. Sub-header `RELEVANT CONTEXT`. Lower row of tags includes `TRY CONTEXTING`. Page body shows `CURRENT UNDERSTANDING` block (paragraph) and the active question card highlighted in orange: "What is the USV's primary power source, and how is power delivered to the ROV via the tether?" with answer options: "A light work-class ROV at 80m depth requires significant power to operate ... typically 5-20 kW delivered at high voltage..." The choices listed are: "Diesel generator (primary) with battery buffer", "Hybrid diesel-electric with significant battery capacity", "Fully battery-electric (range/endurance limited)", "Not yet defined", with a free-text "Different option" field below. Bottom action: `0 of 3 answered  Build with current decisions`.

### 02-round3-comms-safe-state-question.png

What this is: round 3, second question with "Architecture signals" right rail visible.

Question card open with orange accent: "What happens if the communications link to the ROC is lost, and what is the required safe-state behaviour for both the USV and the ROV?" Option list: "USV holds station, ROV holds depth, await comms restoration up to a defined timeout, then both return to surface/home", "USV escalates pre-loaded return-to-home waypoint; ROV ascends and is recovered automatically", "USV continues current mission plan autonomously; ROV continues survey pattern", "Not yet defined", "Different option". Right rail panel `ARCHITECTURE SIGNALS` lists three auto-extracted points: "A light work-class ROV at up to 1000 m depth requires significant tether power (typically 5-20 kW delivered at high voltage)"; "Regulatory framework shapes the entire autonomy and safety architecture"; "Comms loss handling is a top-level safety requirement that directly drives the onboard autonomy architecture." Below `WHAT HAPPENS NEXT`: "Refine the description / Confirm key topics in scope", "Review before building / Inspect the first architecture pass", "Generate starting model / Nodes, interfaces, and a few architecture pass". Bottom action: `0 of 3 answered  Submit answers & refine`.

### 03-ready-to-build-project-genesis.png

What this is: post-interview "Ready to build" screen showing the genesis transcript.

Page title: `Ready to build`. Subtitle: `Review your system brief. The AI will decompose this into subsystems, interfaces, and a system brief.` Left card: `PROJECT NAME: Offshore Survey USV-ROV`. `YOU GET ALONG WITH 1 Root system node, 4-7 subsystems, Key interfaces, System brief`. Action button: `Build →`. Below: `Back to interview`. Centre column header: `PROJECT GENESIS · 3 ROUNDS · Preserved as provenance`. Below: a list of `KEY DECISIONS` with timestamped Q/A pairs (12 entries observed: power source, ops constraints/regulations, comms link loss safe-state, tether/launch architecture, sea state, navigation/positioning, communications bandwidth, USV size/length range, light/satellite constellation, hybrid diesel-electric architecture, regulatory compliance, full classification society oversight). Right column: `BRIEF DESCRIPTION` with confidence chip `82% confidence` followed by a longer multi-paragraph narrative covering hull architecture, propulsion, ROC ops centre, comms (LEO via Ku-band), payload, spec budgets (300-1000 m operational depth, 4-6 m/s transit, sea state 6).

### 04-build-progress-checklist.png

What this is: post-build progress modal.

Title: `Building Offshore Survey USV-ROV...`. Below: `4s elapsed`. Five-step checklist: 1. `Creating project structure` (checked) - Root node + metadata. 2. `Generating system architecture` (active radio) - AI decomposing into subsystems. 3. `Writing system brief` (pending). 4. `Saving project genesis` (pending). 5. `Ready to explore` (pending). Right rail shows `PROJECT GENESIS / KEY DECISIONS` partial view confirming the genesis is being persisted as a sidebar.

### 05-command-palette-context-aware.png

What this is: command palette overlay invoked while on Overview tab.

Modal header: `Context: Offshore Survey USV-ROV (system)`. Free-text input: `Describe, decompose, connect, verify...`. Below header `CONTEXTUAL ACTIONS`: three rows: "Generate requirements for Offshore Survey USV-ROV" with `GENERATE` button; "Create a state machine for Offshore Survey USV-ROV" with `GENERATE` button; "Describe any changes you'd like to make..." with `FREEFORM` button. Background shows the project root with subsystem cards.

### 06-project-root-system-tree-overview.png

What this is: project root view with subsystem grid and full chrome.

Top breadcrumb: `Cairn / Offshore Survey USV-ROV`. Top-bar pill rail (10 named tabs, left to right): `Overview` (active), `Brief`, `Visuals`, `Requirements`, `Architecture`, `Causality`, `Completeness`, `Narrative`, `Dendritic`, `Verification`. Right of tabs: `REVIEW 1`, `QUALITY 73`, `Command` button, settings gear. Left rail header `SYSTEM TREE` with expandable nodes: `Offshore Survey USV-R...` (current), `USV Platform`, `Power Generation & D...`, `ROV Vehicle`, `Launch & Recovery S...`, `Communications & Da...`, `Autonomy, Control & ...`. Below tree, `TOOLS` section with two columns: `Quality / History / Trace / Assets / Simulation / Usage / Types / Settings`. Below: `Export Project / Import Project / ← Projects`. Main content: `SYSTEM` chip then large title `Offshore Survey USV-ROV` with `SYS-OFSUSR` ID below. Body description paragraph repeats the system brief. Right side: `REQS 0 / FILES 0`. `Inspect` button. Section `SYSTEM BUDGETS` with rows `Mass: 800 kg`, `Power: 18 W`. Six subsystem cards in a 3×2 grid: `USV Platform` (Provides the sea-surface vessel hull, propulsion, station-keeping...), `Power Generation & Distribution`, `ROV Vehicle`, `Launch & Recovery System`, `Communications & Data Link`, `Autonomy, Control & Mission Payload`. Each card footer shows `0 reqs · 0 children · 0% traced` with green check status dots.

### 07-interfaces-visuals-attachments.png

What this is: lower portion of Overview view, showing INTERFACES, VISUALS, and ATTACHMENTS sections.

Bottom of subsystem grid shows two cards' footers (`0 reqs · 0 children · 0% traced`) plus an empty placeholder card "+ Add a new subsystem". `INTERFACES (count: 2)`: row 1 `Power Generation & Distribution → ROV Vehicle` with chips `HVDC power + fibre-optic data` and `4 signals`; row 2 `Autonomy, Control & Mission Payload → Communications & Data Link` with chips `Ethernet / IP` and `4 signals`. `VISUALS (count: 1)`: thumbnail with `View all →` link. `ATTACHMENTS (count: 1)`: thumbnail labelled `Offshor...`.

### 08-subsystem-empty-state-decompose-prompt.png

What this is: empty subsystem detail page (Launch & Recovery System) with the empty-state CTA.

Top tabs same as root view. Subsystem chip `SUBSYSTEM` then title `Launch & Recovery System`. ID `SUB-LARS`. Description block. `Inspect` button. Empty-state diamond icon, then prompt text: `Start decomposing Launch & Recovery System / Break this node into subsystems, add requirements, or define interfaces.` Action: orange button `Open Command Palette`. Footer status text: `LARS Structure & Winch Mass: 600 · Max ROV Handling Mass: 250 · Max Operational Sea State: 4 · Nominal Power Consumption: 5`. Bottom status bar: `Model loaded · Offshore Survey USV-ROV · v0.1.0 · 7 nodes · 0 reqs · 2 interfaces · 0 states · 1 file · Quality: 73 · 1 pending review · ⌘K to command`.

### 09-node-detail-properties-budgets.png

What this is: side-rail node detail panel for SUB-USV (USV Platform).

Header: `SUB-USV · Node` with close X. Title `USV Platform`. `DESCRIPTION` block with truncated paragraph. `TYPE: Subsystem`. `CHILDREN: 0 child nodes`. `BUDGETS` section: `Mass 0 / 8,000 kg (no budget)`, `Power 0 / 15 W (no budget)`. `PROPERTIES (4)` with `+` and `+` buttons. Property rows: `◎ Mass Budget · 8000 kg · Physical · ai · Estimated displacement...`; `⚡ Power Budget (Hotel Load) · 15 W · Electrical · ai · Estimated vessel hote...`; `⚙ Max Transit Speed · 10 m/s · Performance · ai · Estimated max transit...`; `Max Transit Sea State · 6 · ai · Maximum sea state for transit op...`. Below: chip-style "+" buttons for additional candidate properties: `+ Nominal Power`, `+ Nominal Voltage` (rest cut off, scrollable).

### 10-node-detail-decompose-generate.png

What this is: continuation of side-rail node detail for SUB-USV, showing additional property suggestion chips, ATTACHMENTS, INTERFACES, and the action buttons.

Above shown rows: same `Power Budget`, `Max Transit Speed`, `Max Transit Sea State` (continuing from 09). Suggested-property chips: `+ Nominal Power`, `+ Nominal Voltage`, `+ Max Range`, `+ Data Rate`, `+ Protocol`, `+ Peak Power`, `Show all (13)`. Section `ATTACHMENTS` with `+` button: `No files attached`, `+ Attach file`. `INTERFACES (0)`: `No interfaces yet`. `+ New Interface`. Two large action buttons: black-filled `◆ Decompose with AI`, white-outline `◆ Generate Requirements`. Below separator: red-outline `Delete` button.

### 11-status-bar-quality-pending-review.png

What this is: zoomed-in status bar across the bottom of the running app.

Verbatim, left to right: `● Model loaded   Offshore Survey USV-ROV   v0.1.0   |   7 nodes   0 reqs   2 interfaces   0 states   📎 1 file   ● Quality: 73   ◆ 1 pending review   ☾   ⌘K to command`. The `Quality: 73` is rendered with a small filled dot in front. The `1 pending review` chip has an orange border.

### 12-causality-pyramid-view.png

What this is: Causality tab open on USV Platform (subsystem) with side rail showing causal position metadata and a quote.

Top tabs visible: `Overview / Brief / Visuals / Requirements / Causality (active) [chip 4] / Completeness / Verification`. Subsystem chip and title `USV Platform`. Body text: paragraph then large faint icon then phrase `No prerequisite data to visualize / Decompose USV Platform to reveal its causality pyramid`. Action: `Decompose with AI`. Right side rail header `SUB-USV · Node`, then `+ Generate Requirements`. Section `CAUSAL POSITION`: chip `Prerequisite for: Offshore Survey USV-ROV`. Warning: `⚠ No children, pyramid layer incomplete`. Section `DEPENDENCIES`: rows `0 REQUIREMENTS / 0 REQUIREMENTS` (sic, two columns), then `0 INTERFACES / 0 GAPS BELOW`. Italic quote block: `"A technology domain performs no other purpose than to be a collection of parts, techniques, tools, and heuristics."` attribution `Harney, Technology Evaluation, Ch. 1`. Three buttons: `▲ Refocus pyramid on this node`, `+ Decompose with AI`, `+ Analyze Causality`. Bottom: red-outline `Delete`.

### 13-completeness-three-axis-radar.png

What this is: zoom of the Completeness radar widget alone, USV Platform (all 0%).

Title `USV Platform`. Subtitle `Three-axis fidelity assessment: Entities · Processes · Relationships`. Triangle radar with vertices labelled `Entities 0%` (top), `Relationships 0%` (bottom-left), `Processes 0%` (bottom-right). Concentric guide rings labelled at 25%, 50%, 75%, 100%. A purple data-point dot sits at centre (0% on all axes). Below, three colour-coded summary cards: `ENTITIES 0% / 0 children defined`, `PROCESSES 0% / 0 behaviors defined`, `RELATIONSHIPS 0% / 0 interfaces · 0 reqs`.

### 14-completeness-with-fix-with-ai-banner.png

What this is: same Completeness widget with surrounding chrome and the inline nudge banner.

Top tabs visible: `Overview / Brief / Visuals / Requirements / Causality / Completeness M (active) / Verification`. Below the radar and three summary cards: yellow-tinted banner with `⚠` icon: `USV Platform is structurally and behaviorally defined but underconnected. 0% relationship coverage means interfaces and requirements are thin.` Action button: `Fix with AI →`.

### 15-completeness-pgd-with-side-panel.png

What this is: Completeness view for Power Generation & Distribution with side-rail breakdown panel.

Top tabs same. Title `Power Generation & Distribution`. Radar with values: `Entities 0%`, `Processes 0%`, `Relationships 33%`. Summary cards: `ENTITIES 0% / 0 children defined`, `PROCESSES 0% / 0 behaviors defined`, `RELATIONSHIPS 33% / 1 interfaces · 0 reqs`. Banner: `⚠ Power Generation & Distribution has structural definition but lacks behavioral specification. Only 0% process coverage. The model knows what it is, but not what it does.` Button: `Fix with AI →`. Side-rail header `SUB-PWR · Node`. Then `Generate Requirements`. Section `OVERALL COMPLETENESS`: bar with `Score 11%` in orange. `ENTITY COVERAGE` bar empty. Children 0, Depth 1. `PROCESS COVERAGE` bar empty: Behaviors 0. `RELATIONSHIP COVERAGE` bar partial purple: Interfaces 1, Requirements 0. `CHILDREN: No children, leaf node`. Italic block quote: `"Fidelity is multidimensional. A single metric is not very meaningful. A set of descriptions, entity coverage, process coverage, relationship coverage, are much more useful in determining what actions to take."` attribution `Pace, Ch.3 (Loper 2015)`.

### 16-completeness-side-panel-pace-quote.png

What this is: zoom of the side panel from 15.

Header `SUB-PWR · Node`, X close. `Generate Requirements`. `OVERALL COMPLETENESS / Score 11%` (orange bar). `ENTITY COVERAGE` (empty bar): Children 0, Depth 1. `PROCESS COVERAGE` (empty bar): Behaviors 0. `RELATIONSHIP COVERAGE` (purple partial bar): Interfaces 1, Requirements 0. `CHILDREN 0 / No children, leaf node`. Pace Ch.3 quote (verbatim above). Bottom: `Delete`.

### 17-system-tree-completeness-percentages.png

What this is: zoom of left rail system tree showing per-node completeness percentages with status-dot colour coding.

Tree rows: `Offshore Survey USV-... · 40%` (yellow dot, partially expanded indicator), `USV Platform · 0%` (red dot), `Power Generation... · 11%` (red dot, currently selected, orange highlight), `ROV Vehicle · 11%` (red dot), `Launch & Recovery... · 0%` (red dot), `Communications &... · 11%` (red dot), `Autonomy, Control... · 11%` (red dot). Diamond outline to the left of each row is the node-type marker.

### 18-generation-pipeline-route-context-generate-validate.png

What this is: generation-in-flight modal with 4-stage progress.

Header: `◇ Context: Offshore Survey USV-ROV (system)`. Action row: `◆ Generate requirements for Offshore Survey USV-ROV` with primary button `Submit ↵` (currently disabled / pinkish). Below separator: a 4-stage horizontal pipeline. Stage 1 `ROUTE...` (active, filled diamond marker). Stage 2 `CONTEXT` (pending circle). Stage 3 `GENERATE` (pending). Stage 4 `VALIDATE` (pending). Stages connected by line segments. Status text: `Engineering your changes... (3.6s)`. Helper text below: `typically 30-60s`. Footer: `ESC close   ↩ submit`.

### 19-round3-architecture-signals-detail.png

What this is: alternate or fuller view of round 3 with architecture-signals list and "what happens next" rail (overlaps with 02 in content).

Same question card "What happens if the communications link to the ROC is lost, and what is the required safe-state behaviour for both the USV and the ROV?" with same answer options. Right rail `ARCHITECTURE SIGNALS` with three bullets (verbatim same as 02). Below `WHAT HAPPENS NEXT` with three forward-step suggestions ("Refine the description", "Review before building", "Generate starting model").

### 20-completeness-pgd-with-side-panel-alt.png

What this is: alternate composition of Completeness PG&D view; near-duplicate of 15 with slight cropping.

Same data: title `Power Generation & Distribution`, radar with `Entities 0% / Processes 0% / Relationships 33%`. Side panel showing OVERALL COMPLETENESS Score 11%, ENTITY/PROCESS/RELATIONSHIP coverage bars, Pace Ch.3 quote.

### 21-command-palette-ai-suggested-followups.png

What this is: command palette in "AI-suggested follow-ups" mode showing 5 prefill cards.

Top: `6 REQUIREMENTS` count chip with `EXPAND ALL DESCRIPTIONS / COLLAPSE ALL` toggles. Below: a single requirement preview row with `type: performance`, body "The Offshore Survey USV-ROV shall compress and prioritise combined ROV video and sonar payload data to a total uplink data rate not exceeding 50 Mbps before transmission to the ROC via the satellite link." Below it: warning chip `⚠ REQ-006 acceptance criteria references a ROV video quality score. The evaluation methodology and assessor qualification criteria should be defined in the verification plan.` Section header: `WHAT'S NEXT? AI-suggested follow-ups · click to prefill the command palette`. Five clickable cards in a 2×3 grid (one card highlighted): "Generate requirements for mission endurance (fuel, battery, and autonomous energy management) once the deployment duration is confirmed."; "Generate requirements for the USV Platform (SUB-USV) decomposing REQ-001 and REQ-005 into hull, propulsion, and navigation sensor sub-requirements." (HIGHLIGHTED); "Generate requirements for the Power Generation & Distribution subsystem (SUB-PWR) covering HVDC tether power delivery, battery capacity, and load shedding priority."; "Generate requirements for the Communications & Data Link subsystem (SUB-COMMS) covering link redundancy, failover time, and cybersecurity in line with classification society scope."; "Generate requirements for the LARS subsystem (SUB-LARS) decomposing REQ-003 into winch rated load, motion compensation performance, and tether management system requirements." Footer button: `Apply with Warnings`.

### 22-ai-reasoning-panel-token-cost.png

What this is: small "AI Reasoning" rail panel.

Header: `◆ AI REASONING`. Body paragraph (verbatim): "I selected six requirements that collectively cover the most critical system-level performance, safety, and functional behaviours described in the node description and brief: transit speed (mission reach), ROV depth (survey envelope), LARS sea state limit (operational availability), comms-loss response (safety case cornerstone), collision avoidance (COLREGS classification mandatory), and edge compression (satellite bandwidth constraint). Each requirement is written as a singular, testable 'shall' statement with measurable acceptance criteria drawn directly from the quantified parameters in the node description and subsystem context. The 6-requirement cap is reached; the most important follow-ups are requirements for mission endurance, power architecture, and USV transit sea state survivability." Footer line: `Discard changes ~13.2k tokens · ~$0.0790`.

### 23-prefilled-followup-card.png

What this is: zoom of a single prefilled follow-up card.

Body: "Generate requirements for the LARS subsystem (SUB-LARS) decomposing REQ-003 into winch rated load, motion compensation performance, and tether management system requirements." Right edge: arrow `→` indicating clickable. Card has rounded peach/cream background.

### 24-command-palette-prefilled-from-review.png

What this is: command palette modal with prefilled text after clicking a follow-up card.

Header: `◇ Context: Offshore Survey USV-ROV (system)`. Big text input shown with the prefilled body: "Generate requirements for the LARS subsystem (SUB-LARS) decomposing REQ-003 into winch rated load, motion compensation performance, and tether management system requirements." Action button on right: `Submit ↵`. Below input, helper line: `↩ prefilled from review modal · review and press Enter to run, or edit first`. Below helper line: `CONTEXTUAL ACTIONS` section with three rows: "Add verification methods for 6 unverified requirements" with `VERIFY` button; "Create a state machine for Offshore Survey USV-ROV" with `GENERATE` button; "Describe any changes you'd like to make..." with `FREEFORM` button. Footer: `ESC close   ↩ submit`.

---

### 25-review-proposed-changes-with-pipeline-trace.png

What this is: full "Review proposed changes" surface for a 6-requirement generation, exposing the pipeline trace.

Header: `Review proposed changes`. Subtitle: `Generated 6 system-level requirements for the Offshore Survey USV-ROV covering transit performance, ROV depth rating, LARS sea state limit, communications loss response, collision avoidance, and satellite uplink data throughput.` Below header, two badges plus the pipeline trace: `Requirements Specialist` (orange-tinted role badge), `6 operations` (count badge), then a pipeline strip rendered as "Router Haiku · 4.2s  →  Context 0.0s  →  Requirements Sonnet · ~107.9s  →  Validator 4.0s" (timings approximate; the Requirements stage timing reads as a 3-digit number making it the longest). Two main columns: left `CONTEXT / WARNINGS / AI REASONING`. Right: `6 REQUIREMENTS`. Three warnings (REQ-004 references a 120-second comms-loss detection threshold and a 5-60 minute return-to-home timeout; REQ-006 acceptance criteria references a MOS video quality score, evaluation methodology and assessor qualification criteria should be defined in the verification plan; Three architectural parameters remain open: mission endurance, classification society scope, logistics model). Right column lists six requirement cards: REQ-001 (transit speed of not less than 10 knots in sea states up to and including Sea State 4 / significant wave height ≤ 2.5 m), REQ-002 (deploy ROV to maximum operating depth of 1000 m below mean sea level), REQ-003 (complete ROV launch and recovery operations via the stern-ramp LARS in sea states up to and including Sea State 4), REQ-004 (upon detection of a loss of the ROC communications link lasting more than 120 seconds, autonomously command the USV to maintain station and the ROV to hold depth, and shall initiate a return-to-surface and return-to-home sequence for both vehicles if the link is not restored within a further operator-configurable timeout period of between 5 and 60 min). Bottom-left status: `Discard changes ~13.2k tokens · ~$0.0790`. Bottom-right action: orange `Apply with Warnings`.

### 26-pipeline-trace-zoom-named-models.png

What this is: cropped header of "Review proposed changes" for a 3-operation Decompose run, showing the pipeline trace clearly.

Header: `Review proposed changes`. Subtitle: `Decompose REQ-003 into three LARS subsystem requirements covering winch rated load, motion compensation performance, and tether management system.` Badge row: `Requirements Specialist` (orange role chip), `3 operations` (count chip), then pipeline trace verbatim:

```
|| Router Haiku · 3.6s → Context 0.0s → Requirements Sonnet · 47.9s → Validator 46.7s
```

The Router stage explicitly names model **Haiku**. The Requirements stage explicitly names model **Sonnet**. The Validator stage shows wall-clock time (46.7s) but does not name a model in this view. Total elapsed approx 98.2s.

### 27-generation-pipeline-requirements-stage-specialized.png

What this is: live in-flight generation modal, same shape as screenshot 18 but with stage 3 labelled "REQUIREMENTS" instead of "GENERATE".

Header: `◇ Context: Offshore Survey USV-ROV (system)`. Action row: `◆ Generate requirements for the LARS subsystem (SUB-LARS) decomposing REQ-003 into winch rated load, motion compensation performance, and tether management system requirements.` Submit button (disabled). Pipeline strip: `ROUTE` → `CONTEXT` → `REQUIREMENTS` → `VALIDATE`. Status text: `Engineering your changes... (98.2s)`. Helper text: `typically 20-40s for requirement generation`. Footer: `ESC close   ↩ submit`.

This proves the third stage label is **dynamic per operation type**. The earlier captured pipeline (screenshot 18) showed `GENERATE` as the third stage; this run shows `REQUIREMENTS`. The stage 3 label specialises to the kind of work being done. Inferred sibling stage names plausibly include `DECOMPOSE`, `STATE-MACHINE`, `VERIFY`, etc., one per operation kind.

Also: the helper text adapts to operation type. Earlier helper said `typically 30-60s`. This one says `typically 20-40s for requirement generation`. The expectation copy is operation-aware.

Also: 98.2s elapsed for a requirements generation that is "typically 20-40s" implies this run is taking 2 to 5x longer than typical. The UI shows the elapsed counter regardless, building trust through transparent staging rather than hiding the lag.

---

## Images needing user identification

Two images in the cache are not from getcairn.dev. Saved with `XX-unidentified-` prefix:

- `XX-unidentified-github-pr-merge-ui.png`: GitHub PR review screen showing "Some checks haven't completed yet: Graphite / mergeability_check (1 in progress); Graphite / AI Reviews (success); No conflicts with base branch; Squash and merge". Possibly captured during the user's PR workflow today, unrelated to getcairn.dev.

- `XX-unidentified-terminal-git-status.png`: Terminal output showing `Resume this session with: claude --resume ...`, then `git status` output for branch `dev` listing modified files (`.claude/settings.json`, `.claude/settings.local.json`, `CLAUDE.md`, `docs/landing/index.html`) and untracked Sauron-related files. Looks like a session-resume snapshot, not getcairn.dev.

If these were captured as part of the getcairn.dev investigation (e.g., comparing CAIRN's own workflow side by side), the user can rename them; otherwise they should be deleted in a future pass.

---

## Corrections to the prior stronghold

Per mission rule "if a screenshot contradicts the stronghold, the screenshot wins; cite the contradiction here, do not edit the stronghold."

1. **Tab count.** The prior stronghold (re-research section) cites their docs as advertising "twelve lenses" with names like Overview / Requirements / Architecture / Verification. The screenshots show **10 top-level tabs** in the running app: Overview, Brief, Visuals, Requirements, Architecture, Causality, Completeness, Narrative, Dendritic, Verification (per `06-project-root-system-tree-overview.png`). It is plausible the docs page lists 12 conceptual lenses while the app surfaces 10. The doc set treats these as two related but distinct claims and notes the discrepancy.

2. **Tools sidebar.** The stronghold does not enumerate the left-rail tools panel. Screenshots show 8 tools: Quality, History, Trace, Assets (column 1) and Simulation, Usage, Types, Settings (column 2). Captured in `06-project-root-system-tree-overview.png`.

3. **Quality score header chip.** The stronghold mentions confidence scores during the interview (78%, 82%) and per-node scores (e.g. 11%) but does not surface the persistent header-bar `Quality 73` chip nor the `1 pending review` counter. Captured in `06-...overview.png` and `11-status-bar-quality-pending-review.png`.

4. **Generation pipeline (4 stages).** The stronghold does not document the four-stage `ROUTE → CONTEXT → GENERATE → VALIDATE` modal observed in `18-generation-pipeline-route-context-generate-validate.png`. This is new architectural evidence: their generation flow exposes routing, context-assembly, generation, and validation as distinct stages with a live elapsed counter and "typically 30-60s" honest expectation copy.

5. **AI-tagged property values.** The stronghold mentions AI-driven proposals but does not document the explicit `· ai ·` tag in property metadata rows (visible in `09-node-detail-properties-budgets.png` and `10-node-detail-decompose-generate.png`). Each AI-suggested value is labelled inline so users can distinguish authored from generated values.

6. **AI Reasoning panel with token/cost.** The stronghold does not mention the explicit token-and-dollar-cost surfacing at the foot of the AI Reasoning panel (`~13.2k tokens · ~$0.0790` in `22-ai-reasoning-panel-token-cost.png`). Worth noting as a transparency UX choice.

7. **Source-text book quotes.** The stronghold does not mention contextual book quotes (Harney, Pace/Loper) appearing in side rails. Two captured: Harney "Technology Evaluation Ch. 1" in `12-causality-pyramid-view.png`; Pace "Ch.3 (Loper 2015)" in `15`/`16`. These are pedagogical UX touches.

8. **System Budgets at root level.** The stronghold mentions per-node budgets (Mass/Power) but the `06-...overview.png` screenshot shows them rolled up at the system root (`Mass: 800 kg / Power: 18 W`), suggesting budgets are aggregable up the tree. Worth noting in `04-node-model.md`.

These corrections are intentionally small and additive. None invalidate the stronghold's core verdict (different product, structural parallels, MBSE-specific). They expand the surface area for synthesis.

---

## Verification surface notes

Observations from the verification-related captures, folded in here for reference. Opinions belong in `08-borrow-list.md`.

**Requirements table (screenshot 28):** the row visible carries five header columns: `REQ ID`, `REQUIREMENT`, `METHOD`, `STATUS`, `DESCRIPTION`. The captured row is `REQ-001 / USV Transit Speed / Test / Draft / Hahaha`. Method and Status are pill-shaped badges in light neutral. A partially clipped section header below reads `UNVERIFIED REQUIREMENTS (5)` in a red or warning-tone serif, suggesting a categorical split between verified and unverified buckets.

**Verification methods enum.** The four classical V&V method values are exposed across the captured surfaces and the export schema: `Test`, `Analysis`, `Demonstration`, `Inspection`. The fixture verification record uses method `test`. The methods are surfaced as both pill badges (in the requirements table) and as columns in tooling-side views (per the docs).

**Verification statuses enum.** Five states observed across screenshots and docs: `Passed`, `Planned`, `Draft`, `Failed`, `Blocked`. The fixture single record carries `Draft`. The status taxonomy is lifecycle-shaped (Draft and Planned are pre-run states; Passed and Failed are post-run; Blocked is a wait state).

**Trace-link agent run (screenshot 29):** an in-flight modal at the `ROUTE` stage of the four-step pipeline (`ROUTE → CONTEXT → GENERATE → VALIDATE`). The agent prompt is `Suggest trace links for requirement REQ-004 ('Communications Loss Hold and Return-to-Home Response') on node Offshore Survey USV-ROV (SYS-o0bcqk). Find components that satisfy this requirement and create 'satisfies' trace links.` The progress copy reads `Engineering your changes... (4.1s)` with hint `typically 30-60s`. The `satisfies` link type is one of four documented (`satisfies`, `verifies`, `derives`, `depends_on`). This is a separate AI specialist run from requirement generation.

**Traceability matrix (screenshot 49):** three coverage axes shown as parallel tabs: `Architecture (0% traced)`, `Behavior (0% covered)`, `Verification (11% verified, active tab)`. Status summary line reads `0 fully traced  1 partial  8 gaps  (9 requirements total)`. Table columns: `REQ ID`, `REQUIREMENT`, `NODE`, `SATISFIED BY`, `VERIFIED BY`, `STATUS`. REQ-001 row carries `VER-fp596o` in the VERIFIED BY column with status `Partial`; the other 8 requirements show `-` in both link columns and status `Gap`. Two filter-bar buttons: `+ Add Link` (manual) and `+ Suggest Links` (AI, highlighted orange).

**Quality Check panel (screenshot 61):** dedicated quality-aggregation surface with a `34` numeric badge plus aggregate counts `0 errors, 21 warnings, 6 info`. Toggle between `Entire Model` (active) and `This Node`. Filter chips: `Errors`, `Warnings`, `Info`. Dropdown `All Categories`. Search field. Two sibling section headers visible: `COMPLETENESS` and `TRACEABILITY`. Five `COMPLETENESS` findings captured, all of shape "X has no artifacts; consider adding requirements or decomposing further" with subsystem id tags (`SUB-USV`, `SUB-PWR`, `SUB-ROV`, `SUB-COMMS`, `SUB-AUTO`) and per-row `Fix` actions. `Re-run` button at the bottom plus a `5:45:06 PM` last-run timestamp.

**Quality score badge (screenshot 60):** a tiny standalone badge captured in isolation. Diamond glyph, label `QUALITY` in muted warm-grey letter-spaced caps, value `34` in a darker weight. The same number appears in the persistent header chip (`Quality 73` in the pre-quality-check view, `34` in the captured Quality Check panel; the score updates with state).

**History panel (screenshot 47):** sibling tabs `All`, `User`, `AI`, `Genesis`. Six entries listed with relative timestamps. Tag taxonomy includes role labels: `AI`, `Brief`, `Requirements`, `Architect`, `Own Verification`. The `Genesis` filter tab is a third actor class alongside User and AI. `Export History` is a discrete export distinct from `Export Project`, suggesting history is a first-class portable artefact.

---

## Visualization surface notes

Observations from the visual-and-3D captures.

**2D Gallery render style picker (screenshot 41):** sub-tabs `2D Gallery` (active, brown underline) and `3D Viewer` (inactive). Page heading `Visualize ROV Vehicle`. Subtitle `Generate concept art, blueprints, and technical renders from your system description.` Six render-style cards in a 3-by-2 grid, each with an emoji glyph, a title, and an uppercase category tag:

- `Photorealistic Studio` (RENDER, camera-with-flash glyph)
- `Technical Blueprint` (TECHNICAL, drafting-triangle glyph)
- `Concept Art` (ARTISTIC, palette glyph)
- `Clay Render` (RENDER, amphora glyph)
- `Isometric Overview` (SCHEMATIC, blue-diamond glyph)
- `Exploded View` (TECHNICAL, collision glyph)

**3D Viewer empty state (screenshot 42):** sub-tabs `2D Gallery` (inactive) and `3D Viewer` (active). Centred placeholder card with diamond glyph, heading `No 3D model generated`, body `Create an interactive 3D mesh from your system description`, primary button `Generate 3D` (white text on brown rounded pill). Below: section heading `Generate a 3D model of ROV Vehicle` (serif), subtitle `Create an interactive 3D mesh from your system description and concept imagery.`

**3D mesh generation pipeline (screenshot 43):** five-stage progress indicator with explicit named stages. Three states rendered: green filled dot (done), brown/rust filled dot (active), hollow ring (pending). Stages visible:

1. `Preparing context` (done)
2. `Loading concept image` (done)
3. `Waiting for specialist` (active, bold)
4. `Validating code` (pending)
5. `Building geometry` (pending)

Top of panel: a circular spinner (mostly grey, rust arc segment) plus a large monospace timer reading `0:09`.

**3D Viewer rendered model (screenshot 62):** active `3D Viewer` tab with checkmark and orange underline. Centred low-poly tugboat model (grey superstructure, brown waterline hull, tan/orange cabin roof, tall mast with stacked navigation lights) on cream viewport background. Floating toolbar in top-right: panel/layout toggle, lighting/material toggle, reset/orbit, bounding box, vertical divider, then `glTF` badge with download glyph plus stats `14,784 verts · 4,928 faces · 37 mats`. Cream rounded outer frame around the viewport.

**Block diagram canvas (screenshot 38):** interactive draggable canvas with a status indicator `Interactive · Drag blocks to arrange` (green dot). Toolbar in top-right with zoom-in, zoom-out, fit/reset, grid icon, and view-mode toggles. Six subsystem cards (`SUB-USV`, `SUB-PWR`, `SUB-ROV`, `SUB-LARS`, `SUB-COMMS`, `SUB-AUTO`) in two rows linked by orthogonal connectors with port markers. Minimap in bottom-right (3-by-2 grid of tan rectangles). Below the canvas, an `INTERFACE CONTROL DOCUMENT  ·  2 connections` table with columns `ID`, `CONNECTION`, `PROTOCOL`, `SIGNALS` rendering the same data as the export's `interfaces[]` array.

**Subsystem card layout (screenshot 34):** six tiles in a 3-by-2 grid under a `DOMAIN TECHNOLOGIES` heading with a right-aligned italic descriptor `Subsystems from the same engineering domain` and a numeric count badge `6`. Each tile has a teal top border and an eyebrow `SUBSYSTEM` label in teal small caps above a bold black title. Selection state uses a rust outline ring (Launch & Recovery System highlighted in the capture).

**Assets library (screenshot 52):** `Assets` page with metadata `1 file  903.3 KB`, primary button `Upload Files` (dark amber), grid/list view toggle (list active). Filter chips: `All` (selected, peach), `Visuals`, `Documents`, `Diagrams`, `Simulation`, `Reference`. Search field. Single asset card showing a photographic image of an offshore survey vessel.

**Visualisation as derived artefact:** across screenshots 41/42/43/62, every visual artefact is generated from "your system description" (the prose the user authored) plus optional concept imagery. The 3D mesh pipeline takes the description plus a concept image, runs Claude Vision, generates MeshBuilder code, runs static analysis, executes in-browser, renders via Three.js, and exports glTF. The pipeline is named per stage at the user surface; the underlying mechanism is closed but observable through the staged progress.

---

## Causality pyramid and dendrite notes

Observations from the causality-and-tree captures.

**Causality view at the system level (screenshot 30):** active tab `Causality 4` (numeric counter on the tab) with siblings `Overview, Brief, Visuals, Requirements, Architecture, Causality, Completeness, Narrative, Dendritic, Verification`. Page header: `SYSTEM` chip, title `Offshore Survey USV-ROV`, identifier `SYS-offset56` (legible from screenshot OCR; canonical id in export is `SYS-o0bcqk`, the rendering may abbreviate or vary). Description paragraph plus `Show more`. Centred `SYSTEM` card. `DOMAIN TECHNOLOGIES` group heading with right-aligned italic gloss `Subsystems that the core engineering domain...` and four `SUBSYSTEM` cards in row 1 (`USV Platform`, `Power Generation & Distribution`, `ROV Vehicle`, `Launch & Recovery System`) plus two more in row 2 (`Communications & Data Link`, `Autonomy, Control & Mission Payload`). `PARTS & MATERIALS` group heading begins at the bottom with four red-tinted `GAP` tiles.

**Causality lanes (screenshot 31):** a scrolled view inside the body of the ROV decomposition. Three section headers visible:

1. `PARTS & ASSEMBLIES` (top, partially clipped). Six pinkish/red part cards, all in `GAP / <name> / needs decomposition` state.
2. `INSTRUMENTS & CONNECTIONS` with right-side caption `interfaces, protocols, and data channels`. Two interface cards (cream/tan) carrying `INTERFACE / <name> / <protocol>` content.
3. `KNOWLEDGE FOUNDATION` with right-side caption `Requirements, constraints, and understanding`. Nine constraint cards (cream/tan), each with a top-tag taxonomy label (`PERFORMANCE`, `ENVIRONMENTAL`, `FUNCTIONAL`, `SAFETY`) and a category footer.

This screenshot establishes the **five-tier pyramid** as a layered navigation surface: System (capstone) at the top, then Domain Technologies, Parts and Materials, Instruments and Connections, Knowledge Foundation. The lanes are rendered horizontally as section bands; the user scrolls vertically through the pyramid.

**Capstone causal-position panel (screenshot 37):** right side rail. Section header `CAUSAL POSITION` with horizontal divider. Tag pill (orange/cream) reading `◇ System capstone` (monospace). Green/sage callout block `Enabled by: USV Platform, Power Generation & Distribution, ROV Vehicle, Launch & Recovery System, Communications & Data Link, Autonomy, Control & Mission Payload`. Section `DEPENDENCIES` with right-aligned count `6`. Four KPI tiles in a 2-by-2 grid: `6 / DESCENDANTS`, `9 / REQUIREMENTS`, `2 / INTERFACES`, `6 / GAPS BELOW`. Pull quote (cream background, left orange rule, italic serif): `Once all of the lower levels are in place, it is only a matter of time before someone places the capstone on the pyramid.` Attribution `Harney, Technology Evaluation, Ch. 1`. Two action buttons: `▲ Refocus pyramid on this node`, `◆ Analyze Causality`.

**Subsystem causal-position panel (screenshot 36):** for SUB-LARS. Same section structure but values shift. `Prerequisite for: Offshore Survey USV-ROV` (pink/red info card). Yellow warning card with triangle icon reading "No children, pyramid layer incomplete". Stat grid: `0 / DESCENDANTS`, `3 / REQUIREMENTS`, `0 / INTERFACES`, `0 / GAPS BELOW`. Pull quote: `A technology domain performs no other purpose than to be a collection of parts, techniques, tools, and heuristics.` Attribution `Harney, Technology Evaluation, Ch. 1`. Action buttons: `▲ Refocus pyramid on this node`, `◆ Decompose with AI`, `◆ Analyze Causality`.

**Gap card with AI tooltip (screenshot 33):** close-up of a single gap-flagged node card with a hovering tooltip. Pale red/pink card background, dark red top border. Warning triangle glyph, red bold uppercase `GAP` label, italic dark red title (`Launch & Recovery System`), monospace muted subtext `needs decomposition`. Dark grey rounded tooltip overlapping the card with white text: `Click to decompose Launch & Recovery System with AI`. Establishes the `GAP / needs decomposition` lifecycle state plus the inline AI-decomposition affordance.

**Dendrite view (screenshot 55):** active tab `Dendrite` (with a small numeric badge). Left panel header row: `7 active`, `8 pruned`, `depth 1`. Search field with `Pinned (0)` pill, `Expand All` and `Collapse` buttons. Tree rooted at `Offshore Survey USV-ROV` with five subsystem children visible. Each tree row carries a description preview plus a tag row (`ENGINEERING DECISION` faintly visible under each). Selected node `Launch & Recovery System` (highlighted with left accent). Right detail panel: breadcrumb `Offshore Survey USV-ROV / Launch & Recovery System`, title, subtitle, tag chips row `ACTIVE PATH`, `ENGINEERING`, `DECISION`, `DEPTH 1`. Section `DESCRIPTION` with body paragraph and `Show more`. The `pruned` count plus `pinned` plus `depth` triplet suggests the dendrite is a graph-pruning view that surfaces decision history (active path versus pruned alternatives).

**Systemigram with narrative (screenshot 56):** active tab `Narrative`. Page heading `Offshore Survey USV-ROV`, subheading `Systemigram - Narrative Analysis`. Highlighted narrative quote block (cream/peach background): `The Power Generation & Distribution energises and commands the ROV Vehicle, whose video and sonar returns the Autonomy, Control & Mission Payload edge-processes and compresses, which the Communications & Data Link relays as processed survey data to the remote operations centre ashore.` Diagram canvas with `Reset View` button. Active nodes (solid peach borders) connected by curved labelled arrows: `Power Generation & Distribution / Energy Source` → `ROV Vehicle / Survey Payload` → `Autonomy, Control & Mission Payload / Decision Engine` → `Communications & Data Link / Data Relay`. Dashed-border placeholder nodes at the bottom (`Offshore Survey USV-ROV / System`, `USV Platform / Vessel Hull`, `Launch & Recovery System / Deployment Mechanism`) represent context nodes outside the mainstay path. Footer caption inside canvas: `Reads top-left -> bottom-right`.

**Pyramid tier vocabulary (consolidated across 30, 31, 36, 37):**

- **Capstone (System):** the root. Marked with a `System capstone` tag pill plus an `Enabled by:` list.
- **Domain Technologies (Subsystems):** the immediate children. Section heading `DOMAIN TECHNOLOGIES`.
- **Parts and Materials:** leaf components. Section heading `PARTS & MATERIALS`. Renders as red-tinted GAP tiles when undecomposed.
- **Instruments and Connections (Interfaces):** interface cards. Section heading `INSTRUMENTS & CONNECTIONS` with caption `interfaces, protocols, and data channels`.
- **Knowledge Foundation (Requirements and constraints):** constraint cards categorised by type (PERFORMANCE, ENVIRONMENTAL, FUNCTIONAL, SAFETY). Section heading `KNOWLEDGE FOUNDATION` with caption `Requirements, constraints, and understanding`.

The pyramid is displayed lane-by-lane (horizontal bands stacked vertically) rather than as a triangular shape. The triangle is metaphor; the rendering is a horizontal-banded stack with section captions.

---

## Observation: software-domain PRD ingestion (single sample)

Logged 2026-04-28 from a user experiment captured at `screenshots/65-software-domain-prd-decomposition.png`.

**What the user did.** Fed a software-domain PRD into getcairn.dev. The PRD described **OpenSpine**, a self-hostable event-driven runtime substrate for governed agents with capabilities including event ingestion, source verification, identity resolution, deterministic routing, authority composition, task grant issuance, gate-mediated effects, and auditing. The first product was named **Lyra**, a Telegram-controlled personal assistant whose first guarded workflow is selected-thread email reply drafting against a Gmail mailbox. This is unambiguously a software / agent-architecture domain, not the hardware MBSE space getcairn.dev's marketing positions.

**What the UI returned.** The platform accepted the input without complaint and ran it through the same `Round 1 of 3 / Shape the starting model` flow used for hardware domains. Same UI surfaces visible: `CURRENT UNDERSTANDING` block, `REFINED DESCRIPTION` with a 72% confidence pill, decomposition questions phrased in software-domain terms (one captured question began "What is the intended deployment target and operator profile" with multiple-choice options plus a free-text fallback). The interview shape carries no domain awareness in the UI chrome itself.

**The user's framing (verbatim):** "I guess that's what I meant about our cairn potentially being something flexible simply by the architecture and the fact that it can use AI to help fill the architecture."

**Caveats.**

- Sample size is one. A single experiment cannot establish that the platform is genuinely domain-flexible. It can only establish that it accepted the input.
- The decomposition format may still be hardware-flavoured under the hood. The captured screenshot only shows Round 1 of 3. Whether downstream specialists (Architect, Requirements, Interfaces, Behavior) carry the software domain through into the resulting model, or whether they normalise toward subsystem-with-mass-and-power-budget shapes, is not visible from this single mid-interview frame.
- The 72% confidence pill is moderate, not high. The platform may be flagging that the input is unusual for its trained behaviour.
- The platform's two-node-type schema (`system`, `subsystem`) is flat by design. That flatness may be deliberate (domain richness deferred to AI specialists at inference) or accidental (a hardware-shaped product that happens to absorb arbitrary text into a fixed schema). This single screenshot does not distinguish between those two possibilities.

**Action.** Log as hypothesis. Do not treat as evidence that our framework (kernel-fence plus AI assistance plus typed artefact taxonomy) is more or less flexible than getcairn.dev's. Validating the underlying claim would need at least three distinct-domain experiments on getcairn.dev (hardware system, software substrate, organisational process) plus a comparison of the resulting model shapes to see whether the decomposition truly generalises or homogenises.

Cross-references: section 12 of [07-ontology-comparison.md](./07-ontology-comparison.md) carries the structural framing; the L-priority entry in [08-borrow-list.md](./08-borrow-list.md) ("Domain-flex via AI normalisation, not schema enrichment") logs the hypothesis as a tracking item rather than an adoption candidate.
