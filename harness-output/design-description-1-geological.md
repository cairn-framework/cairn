# Design description, iteration 1 — Direction A: Strata Survey (geological / cairn-stones)

Rejected sibling concepts (one line each):
- "Trail map": a hiking-map pastiche with contour lines everywhere; rejected as decoration without information.
- "Rock garden": literal illustrated stones as nodes; rejected as theme-park kitsch violating the anti-goals.

## Concept

The explorer is a geological survey plate. A codebase accretes like sediment; cairn is the survey that marks what is really there. The screen reads like a museum specimen drawer crossed with a field notebook: warm mineral paper, engraved labels, strata bands, and survey-marker symbols. Calm, archival, authoritative.

## Layout geometry

A thin survey-header band across the top carries the expedition facts: the map name, the count of formations (nodes) and contacts (edges), the survey hash, and a clean-bill seal when nothing drifts. Below, the plate splits roughly two thirds to one third: the survey plate (graph canvas) left, the specimen card (inspector) right. The specimen card is a physically distinct sheet, slightly lighter, as if a card were laid on the plate. A shallow findings ledger runs beneath the plate, ruled like a ledger book. The command surface sits at the top of the plate: a small brass-line search field and layer toggles styled as map-legend entries.

## Graph canvas as strata

Nodes are not circles; they are cut stone blocks: small horizontal slabs with slightly irregular corner radii, wider for containers, widest for the system. Containment is expressed as strata: the system is the bedrock band, containers are beds within it, modules are stones resting in their bed. Dependency edges are thin engraved contact lines with a subtle downhill direction tick, like a dip arrow on a geological map. The selected node gets a survey benchmark mark: a small triangle-and-dot monument symbol beside it. State language: synced stones sit flush; a ghost stone would be drawn as a dashed outline only, an outline of a stone that is not there; an orphaned stone sits tilted out of its bed with a hazard-ochre underline. A small legend explains the three states with miniature stones.

## Colour

Warm mineral neutrals: limestone paper, deeper shale for engraved text, a putty mid-tone for beds. One structural accent: oxidised copper green for survey marks, selection, and interactive affordances. One alarm hue: hazard ochre reserved exclusively for drift (ghost, orphaned, warnings). Never pure black or white; the darkest ink is warm shale, the lightest ground is limestone. Info findings are ink, not colour.

## Typography

Engraved-plate serif for identity and node names, the voice of a specimen label: high contrast, slightly condensed. A quiet grotesque for body and metadata. Tabular figures for all counts. Small caps with generous letterspacing for zone titles (SURVEY, SPECIMEN, LEDGER), like map lettering.

## Spatial rhythm

Strata logic: horizontal bands of unequal thickness, thicker where information accretes. Ruled hairlines separate bands, like ledger rules. Negative space is the paper itself; it should feel like unmarked ground, not emptiness. Density lives in the specimen card: tight rows of facts, ruled every row.

## Motion

Minimal and lapidary: selection marks fade in like an ink stamp settling; no bounce, no slide theatrics.

## Mood

The user should feel like they are reading a trustworthy survey of terrain that existed before them and will outlast them. Calm confidence, archival weight, no dashboard anxiety.
