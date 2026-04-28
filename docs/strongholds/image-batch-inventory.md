# Image Batch Inventory — getcairn.dev research net

**Generated:** 2026-04-28 post-compaction
**Purpose:** Authoritative path manifest for all session image inputs. Every analysis sub-agent reads from this list, ONE IMAGE PER CALL. Oversized images (>2000px in any dimension) **must be downscaled before reading** or the dim-limit error recurs.

## Why this file exists

The 2026-04-28 Recovery Nazgul (`a4de0475da9682b41`) failed twice with `image dimension limit (>2000px) on many-image request`. Root cause: 28 of 61 cache images exceed 2000px, and any read of an oversized image (even alone) trips the limit. Resize-before-read is mandatory.

## Path roots

- **Image cache (current session, sha-based numbering 1..63 with 19 and 57 missing):** `~/.claude/image-cache/3c554f47-4023-470a-b0ac-e0f4abe0fdbf/<n>.png`
- **Clipboard temp (timestamp-based, all today's screenshots before they were renumbered into the cache):** `/var/folders/r_/st8p53nx0kvgy3_5jx012p940000gn/T/clipboard-2026-04-28-<HHMMSS>-<sha8>.png`
- **Older session image-cache (pre-compaction Round 1):** `~/.claude/image-cache/8c0c44ea-5c4b-4d33-ac18-3b91cdd60526/` (5 files, pre-checked in)
- **Resize landing zone (to be created by stage scout):** `docs/research/getcairn-dev/screenshots/_resized/<n>.png` (max-dim 1800px, no upscale)

## Already-staged screenshots (do NOT reanalyse)

Located at `docs/research/getcairn-dev/screenshots/01-..-27-...png`. These map to cache images 1-27 and are already covered by docs `01-product-overview.md` through `06-command-palette.md`. Two `XX-unidentified-*.png` are dead-ends (terminal + GitHub PR UI screenshots, not getcairn.dev surface).

## Cache image audit (all 61 files)

Format: `# dims  bytes  flag  best-guess content`. OVERSIZED = >2000px on width or height. Content guesses are derived from the conversation summary; sub-agents must verify on read.

| # | Dimensions | Bytes | Flag | Content guess |
|---|---|---|---|---|
| 1 | 2576x1233 | 2491631 | OVERSIZED | round 3 starting model / shape |
| 2 | 2575x1384 | 2575394 | OVERSIZED | round 3 comms safe-state Q |
| 3 | 2576x1225 | 2258151 | OVERSIZED | ready to build / project genesis |
| 4 | 1192x1092 | 639993 | | build progress checklist |
| 5 | 2576x1273 | 851565 | OVERSIZED | command palette context-aware |
| 6 | 2576x1194 | 594108 | OVERSIZED | project root system tree |
| 7 | 2576x873 | 294149 | OVERSIZED | interfaces visuals attachments |
| 8 | 2576x1255 | 432165 | OVERSIZED | subsystem empty state |
| 9 | 634x1456 | 152934 | | node detail properties+budgets |
| 10 | 632x1396 | 128172 | | node detail decompose+generate |
| 11 | 2576x60 | 41773 | OVERSIZED | status bar quality+pending review |
| 12 | 2576x1369 | 443887 | OVERSIZED | causality pyramid |
| 13 | 1992x1196 | 164728 | | completeness three-axis radar |
| 14 | 1986x1542 | 228669 | | completeness fix-with-ai banner |
| 15 | 2576x1380 | 594975 | OVERSIZED | completeness PGD side panel |
| 16 | 634x1460 | 134753 | | completeness side panel pace quote |
| 17 | 492x466 | 51057 | | system tree completeness % |
| 18 | 1258x630 | 90144 | | generation pipeline route|context|generate|validate |
| 20 | 2214x208 | 71904 | OVERSIZED | command palette AI suggested followups |
| 21 | 1128x674 | 102922 | | round 3 architecture signals detail |
| 22 | 2575x1407 | 717495 | OVERSIZED | completeness PGD alt |
| 23 | 620x1486 | 172954 | | AI reasoning panel token+cost |
| 24 | 1384x1276 | 235844 | | prefilled followup card |
| 25 | 1760x92 | 24899 | | command palette prefilled from review |
| 26 | 2058x318 | 58834 | OVERSIZED | review proposed changes pipeline trace |
| 27 | 2082x364 | 128697 | OVERSIZED | pipeline trace zoom named models |
| 28 | 2036x174 | 34496 | OVERSIZED | requirements-stage specialised pipeline |
| 29 | 1108x658 | 104515 | | unknown - new |
| 30 | 2144x1304 | 253852 | OVERSIZED | unknown - new |
| 31 | 1854x1450 | 337794 | | interfaces ICD tab |
| 32 | 1676x1398 | 341930 | | interfaces signal model IN/OUT |
| 33 | 746x256 | 40962 | | unknown - small panel |
| 34 | 1696x472 | 100941 | | unknown - banner/header |
| 35 | 600x1480 | 166842 | | node detail panel budgets |
| 36 | 600x1512 | 144786 | | node detail panel properties+ai |
| 37 | 588x1094 | 121162 | | node detail panel suggestion chips |
| 38 | 2574x1320 | 414124 | OVERSIZED | interfaces alt view |
| 39 | 1974x1472 | 352620 | | node detail causal position |
| 40 | 2110x892 | 178776 | OVERSIZED | interfaces protocols HVDC+fibre |
| 41 | 1646x1118 | 147573 | | visuals 2D gallery |
| 42 | 2114x1436 | 145945 | OVERSIZED | visuals 3D viewer pipeline |
| 43 | 528x396 | 37430 | | unknown - small badge |
| 44 | 480x518 | 60598 | | requirements panel filters |
| 45 | 1480x1366 | 196903 | | NEW THIS TURN - to identify |
| 46 | 496x216 | 21123 | | NEW THIS TURN - small badge/chip |
| 47 | 1228x1566 | 259682 | | NEW THIS TURN |
| 48 | 2575x1061 | 312320 | OVERSIZED | NEW THIS TURN |
| 49 | 2574x1027 | 514501 | OVERSIZED | NEW THIS TURN |
| 50 | 2574x712 | 109284 | OVERSIZED | NEW THIS TURN |
| 51 | 2575x850 | 196854 | OVERSIZED | NEW THIS TURN |
| 52 | 2574x558 | 182186 | OVERSIZED | NEW THIS TURN |
| 53 | 1606x1064 | 128851 | | NEW THIS TURN |
| 54 | 984x684 | 86167 | | NEW THIS TURN |
| 55 | 2152x1078 | 331878 | OVERSIZED | NEW THIS TURN |
| 56 | 2210x1488 | 252035 | OVERSIZED | NEW THIS TURN |
| 58 | 748x1718 | 239540 | | usage/cost panel ($1.13, 26 calls) |
| 59 | 748x1718 | 239540 | | usage/cost panel duplicate of 58 |
| 60 | 208x82 | 7381 | | quality 34 button chip |
| 61 | 758x1706 | 242896 | | quality check panel (errors/warnings) |
| 62 | 2575x1408 | 287294 | OVERSIZED | 3D viewer glTF tugboat |
| 63 | 2575x1408 | 287294 | OVERSIZED | 3D viewer glTF tugboat duplicate of 62 |

Note: images 19 and 57 are absent from the cache (skipped or de-duped at capture).

## Clipboard temp paths (timestamped capture trail)

55 files in `/var/folders/r_/st8p53nx0kvgy3_5jx012p940000gn/T/`. These mirror the cache 1:1 in capture order; sub-agents should prefer the cache numeric paths since those are stable.

```
clipboard-2026-04-28-150703-C006DCBD.png
clipboard-2026-04-28-163055-90FBB273.png
clipboard-2026-04-28-170712-D02ED926.png
clipboard-2026-04-28-171754-1589C945.png
clipboard-2026-04-28-171815-38CB1CB7.png
clipboard-2026-04-28-171823-833C186F.png
clipboard-2026-04-28-171828-7CB42D77.png
clipboard-2026-04-28-172008-BF525B5B.png
clipboard-2026-04-28-172322-D6C7DA90.png
clipboard-2026-04-28-172334-EB4D82BD.png
clipboard-2026-04-28-172341-93B180AA.png
clipboard-2026-04-28-172352-750E1B43.png
clipboard-2026-04-28-172639-F689E4E9.png
clipboard-2026-04-28-172847-4333FC6C.png
clipboard-2026-04-28-173229-D4D1F916.png
clipboard-2026-04-28-173237-3D997724.png
clipboard-2026-04-28-173304-9A611C3B.png
clipboard-2026-04-28-173311-6E624F29.png
clipboard-2026-04-28-173316-5797DF95.png
clipboard-2026-04-28-173412-FBAB3317.png
clipboard-2026-04-28-173437-7532833C.png
clipboard-2026-04-28-173617-A5A7C4A3.png
clipboard-2026-04-28-173720-116A6F5C.png
clipboard-2026-04-28-173729-4066C922.png
clipboard-2026-04-28-173744-A0E82618.png
clipboard-2026-04-28-173809-F3C5D6EA.png
clipboard-2026-04-28-173822-F533107D.png
clipboard-2026-04-28-173831-6D086C44.png
clipboard-2026-04-28-173840-105642D6.png
clipboard-2026-04-28-173851-66E8EC7E.png
clipboard-2026-04-28-173907-648198C4.png
clipboard-2026-04-28-173922-1AFD4480.png
clipboard-2026-04-28-173944-A3D7874D.png
clipboard-2026-04-28-174024-D30D396A.png
clipboard-2026-04-28-174034-6ABB9B9B.png
clipboard-2026-04-28-174042-33C22FD9.png
clipboard-2026-04-28-174102-E15306D8.png
clipboard-2026-04-28-174134-16FBD0F2.png
clipboard-2026-04-28-174200-CCB4BFC4.png
clipboard-2026-04-28-174233-F6A1EC34.png
clipboard-2026-04-28-174247-7963B6DD.png
clipboard-2026-04-28-174255-B04ADC6C.png
clipboard-2026-04-28-174305-8C0E30FF.png
clipboard-2026-04-28-174311-8555B95C.png
clipboard-2026-04-28-174327-4B237A33.png
clipboard-2026-04-28-174339-0FF93824.png
clipboard-2026-04-28-174401-92F12E5B.png
clipboard-2026-04-28-174422-3A0D6FC7.png
clipboard-2026-04-28-174733-E95F63E2.png
clipboard-2026-04-28-175108-535F81CC.png
clipboard-2026-04-28-175127-FF04B106.png
clipboard-2026-04-28-175132-344925FF.png
clipboard-2026-04-28-175142-F8FA3462.png
clipboard-2026-04-28-175204-84651D06.png
clipboard-2026-04-28-175233-221CDE82.png
```

## Processing protocol — strict

1. **Stage scout (one-shot bash):** for every cache image with width or height > 2000, run `sips --resampleHeightWidthMax 1800 <src>.png --out screenshots/_resized/<n>.png`. For non-oversized images, hard-link or copy unchanged into `_resized/<n>.png` so analysers always reach the same path.
2. **Per-image analysis scouts:** dispatch ONE scout per image. Each scout reads exactly ONE file from `screenshots/_resized/<n>.png` and writes findings to `docs/research/getcairn-dev/screenshots/_analysis/<n>.md`. Scouts MUST NOT call Glob, MUST NOT batch reads, MUST NOT view multiple PNGs in one Read call.
3. **Synthesis:** after all analyses, a Nazgul reads only the markdown analysis files (no images) and authors the missing 4 docs (07-ontology-comparison, 08-borrow-list, 09-design-influence, 10-source-attribution) plus the new sections owed by the conversation summary.

## Analysis priority order

**Tier 1 — newest, unprocessed, high-value (dispatch first):**
- 31, 32, 38, 40 (interfaces / ICD / signal model)
- 35, 36, 37, 39 (node detail panel — budgets, ai chips, causal position)
- 41, 42 (visuals 2D gallery + 3D viewer pipeline)
- 44 (requirements panel filters)
- 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56 (NEW THIS TURN — content unknown, identify first)
- 58, 59, 61 (usage/cost + quality-check panel — operational metering signals)
- 62, 63 (3D glTF viewer asset — capability proof)

**Tier 2 — verification page (already partially documented):**
- 28, 29, 30 (verification methods page from prior summary)
- 33, 34, 43 (small fragments — may be dead ends)

**Tier 3 — low-priority / duplicates:**
- 60 (single QUALITY 34 chip — fold into 61 analysis)
- 58 ↔ 59 duplicate, 62 ↔ 63 duplicate — analyse once

## Missing doc files to author after analysis

- `07-ontology-comparison.md` — their methods/links ↔ our evidence/provenance edges
- `08-borrow-list.md` — concrete affordances worth porting
- `09-design-influence.md` — visual + voice cues to absorb
- `10-source-attribution.md` — what we saw, in their UI, and where

## Completion gate

Doc set is "production-ready" only when:
- All Tier 1 + Tier 2 images have analysis files in `_analysis/`
- 4 missing doc files exist with no placeholder phrases (TBD/TODO/PLACEHOLDER/etc.)
- `working-notes.md` updated with Verification, Visualization, Causality-Pyramid sections folded in
- Em-dash audit clean (CLAUDE.md ban)
