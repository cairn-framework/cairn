// Deterministic, DOM-grounded visual eval run inside the rendered page, plus the
// token-palette parser used to flag off-design-system colours.
//
// `auditPage` is shipped into Chrome via `auditPage.toString()`, so it must stay
// fully self-contained (only nested helpers, no imports/closures).

/**
 * Parse the allowed colour palette out of docs/design-system/tokens.css.
 * Returns a Set of normalised colour strings matching `auditPage`'s `norm()`
 * output: `rgb(r,g,b)` for opaque, `rgba(r,g,b,a)` for translucent.
 */
export function parsePalette(tokensCss) {
  const palette = new Set();
  // Strip CSS comments so prose hex in headers does not widen the palette.
  const css = tokensCss.replace(/\/\*[\s\S]*?\*\//g, "");

  for (const m of css.matchAll(/#([0-9a-fA-F]{3}|[0-9a-fA-F]{6})\b/g)) {
    const hex = m[1];
    let r;
    let g;
    let b;
    if (hex.length === 3) {
      r = parseInt(hex[0] + hex[0], 16);
      g = parseInt(hex[1] + hex[1], 16);
      b = parseInt(hex[2] + hex[2], 16);
    } else {
      r = parseInt(hex.slice(0, 2), 16);
      g = parseInt(hex.slice(2, 4), 16);
      b = parseInt(hex.slice(4, 6), 16);
    }
    palette.add(`rgb(${r},${g},${b})`);
  }

  for (const m of css.matchAll(/rgba?\(([^)]+)\)/g)) {
    const parts = m[1].split(",").map((s) => s.trim());
    if (parts.length < 3) continue;
    const r = Math.round(parseFloat(parts[0]));
    const g = Math.round(parseFloat(parts[1]));
    const b = Math.round(parseFloat(parts[2]));
    if (parts.length >= 4) {
      const a = Number(parseFloat(parts[3]).toFixed(3));
      palette.add(a >= 1 ? `rgb(${r},${g},${b})` : `rgba(${r},${g},${b},${a})`);
    } else {
      palette.add(`rgb(${r},${g},${b})`);
    }
  }
  return palette;
}

/**
 * Audit the currently-rendered document for visual/UX defects. All measurements
 * are derived from layout + computed style, so they are deterministic given a
 * fixed dataset and blocked web fonts. SVG-internal nodes (the pannable graph)
 * are excluded; the graph is only used for landmark presence checks.
 *
 * @param {{palette: string[], checkTap: boolean}} opts
 */
export function auditPage(opts) {
  const paletteSet = new Set(opts.palette);
  const checkTap = !!opts.checkTap;
  const vw = window.innerWidth;
  const vh = window.innerHeight;
  const docEl = document.scrollingElement || document.documentElement;

  function parseColor(str) {
    if (!str) return null;
    const m = str.match(/rgba?\(([^)]+)\)/);
    if (!m) return null;
    const p = m[1].split(",").map((s) => parseFloat(s));
    return { r: p[0], g: p[1], b: p[2], a: p[3] === undefined ? 1 : p[3] };
  }
  function norm(c) {
    if (!c) return null;
    const r = Math.round(c.r);
    const g = Math.round(c.g);
    const b = Math.round(c.b);
    if (c.a >= 1) return `rgb(${r},${g},${b})`;
    return `rgba(${r},${g},${b},${Number(c.a.toFixed(3))})`;
  }
  function allow(n) {
    return (
      n === "rgb(0,0,0)" ||
      n === "rgb(255,255,255)" ||
      n.startsWith("rgba(0,0,0,") ||
      n.startsWith("rgba(255,255,255,")
    );
  }
  function relLum(c) {
    const f = (v) => {
      const s = v / 255;
      return s <= 0.03928 ? s / 12.92 : Math.pow((s + 0.055) / 1.055, 2.4);
    };
    return 0.2126 * f(c.r) + 0.7152 * f(c.g) + 0.0722 * f(c.b);
  }
  function contrastRatio(a, b) {
    const l1 = relLum(a);
    const l2 = relLum(b);
    const hi = Math.max(l1, l2);
    const lo = Math.min(l1, l2);
    return (hi + 0.05) / (lo + 0.05);
  }
  function effectiveBg(el) {
    const layers = [];
    let node = el;
    while (node && node !== document.documentElement) {
      const colour = parseColor(getComputedStyle(node).backgroundColor);
      if (colour && colour.a > 0) layers.push(colour);
      node = node.parentElement;
    }
    let painted = { r: 20, g: 19, b: 16, a: 1 }; // stone-1 fallback
    for (let i = layers.length - 1; i >= 0; i -= 1) {
      painted = composite(layers[i], painted, layers[i].a);
    }
    return painted;
  }
  function backgroundWithin(el, stopExclusive) {
    const layers = [];
    let node = el;
    while (node && node !== stopExclusive) {
      const colour = parseColor(getComputedStyle(node).backgroundColor);
      if (colour && colour.a > 0) layers.push(colour);
      node = node.parentElement;
    }
    let painted = { r: 0, g: 0, b: 0, a: 0 };
    for (let i = layers.length - 1; i >= 0; i -= 1) {
      painted = composite(layers[i], painted, layers[i].a);
    }
    return painted;
  }
  // Return the text element's own opacity separately from ancestor opacity.
  // An ancestor fades the entire subtree, including its painted background, so
  // applying the combined alpha to the foreground alone does not describe the
  // pixels. `outermostFaded` identifies the backdrop outside that group.
  function opacityContext(el) {
    const own = parseFloat(getComputedStyle(el).opacity);
    let ancestors = 1;
    let fadedAncestors = 0;
    let outermostFaded = null;
    let node = el.parentElement;
    while (node && node.nodeType === 1) {
      const v = parseFloat(getComputedStyle(node).opacity);
      if (!Number.isNaN(v)) {
        ancestors *= v;
        if (v < 1) {
          outermostFaded = node;
          fadedAncestors += 1;
        }
      }
      node = node.parentElement;
    }
    return {
      own: Number.isNaN(own) ? 1 : own,
      ancestors,
      outermostFaded,
      fadedAncestors,
    };
  }
  // Porter-Duff source-over. Keeping alpha matters inside an opacity group:
  // flattening a translucent card to opaque before group compositing paints the
  // outside backdrop twice.
  function composite(fg, bg, alpha) {
    const a = alpha < 0 ? 0 : alpha > 1 ? 1 : alpha;
    const bgAlpha = bg.a === undefined ? 1 : bg.a;
    const outAlpha = a + bgAlpha * (1 - a);
    if (outAlpha === 0) return { r: 0, g: 0, b: 0, a: 0 };
    return {
      r: (fg.r * a + bg.r * bgAlpha * (1 - a)) / outAlpha,
      g: (fg.g * a + bg.g * bgAlpha * (1 - a)) / outAlpha,
      b: (fg.b * a + bg.b * bgAlpha * (1 - a)) / outAlpha,
      a: outAlpha,
    };
  }
  // Resolve the final foreground and background pixels. Element opacity applies
  // to the foreground. Ancestor opacity then flattens both it and the local
  // background onto the backdrop outside the outermost faded group.
  function renderedPair(el, fg, localBg) {
    const opacity = opacityContext(el);
    let paintedFg = composite(fg, localBg, fg.a * opacity.own);
    let paintedBg = localBg;
    if (opacity.ancestors < 1) {
      const outside = effectiveBg(opacity.outermostFaded?.parentElement);
      paintedFg = composite(paintedFg, outside, paintedFg.a * opacity.ancestors);
      paintedBg = composite(localBg, outside, localBg.a * opacity.ancestors);
    }
    const ownBg = parseColor(getComputedStyle(el).backgroundColor);
    const ownBackgroundOpacity = opacity.own < 1 && !!ownBg && ownBg.a > 0;
    return {
      fg: paintedFg,
      bg: paintedBg,
      alpha: opacity.own * opacity.ancestors,
      nestedOpacity: opacity.fadedAncestors > 1,
      ownBackgroundOpacity,
    };
  }
  function isVisible(el, cs, rect) {
    if (cs.display === "none" || cs.visibility === "hidden") return false;
    const opacity = opacityContext(el);
    if (opacity.own * opacity.ancestors === 0) return false;
    if (rect.width <= 0 || rect.height <= 0) return false;
    if (rect.bottom < 0 || rect.right < 0) return false;
    return true;
  }
  function sig(el) {
    const cls = typeof el.className === "string" ? el.className : "";
    return `${el.tagName.toLowerCase()}.${cls.trim().split(/\s+/).join(".")}`;
  }

  const contrastSigs = new Set();
  const clippedSigs = new Set();
  const offscreenSigs = new Set();
  const tapSigs = new Set();
  const paletteViol = new Set();
  let textElements = 0;

  const all = document.querySelectorAll("body *");
  for (const el of all) {
    if (el.closest("svg")) continue; // graph internals are intentionally large
    const cs = getComputedStyle(el);
    const rect = el.getBoundingClientRect();
    if (!isVisible(el, cs, rect)) continue;

    if (rect.right > vw + 2 && rect.left < vw - 2) offscreenSigs.add(sig(el));

    const colC = parseColor(cs.color);
    if (colC && colC.a > 0) {
      const n = norm(colC);
      if (n && !paletteSet.has(n) && !allow(n)) paletteViol.add(n);
    }
    const bgC = parseColor(cs.backgroundColor);
    if (bgC && bgC.a > 0) {
      const n = norm(bgC);
      if (n && !paletteSet.has(n) && !allow(n)) paletteViol.add(n);
    }
    for (const side of ["Top", "Right", "Bottom", "Left"]) {
      if (parseFloat(cs[`border${side}Width`]) > 0) {
        const bc = parseColor(cs[`border${side}Color`]);
        if (bc && bc.a > 0) {
          const n = norm(bc);
          if (n && !paletteSet.has(n) && !allow(n)) paletteViol.add(n);
        }
      }
    }

    let hasText = false;
    for (const child of el.childNodes) {
      if (child.nodeType === 3 && child.textContent.trim().length > 0) {
        hasText = true;
        break;
      }
    }
    if (hasText) {
      textElements += 1;
      const fg = parseColor(cs.color);
      if (fg && fg.a > 0) {
        const opacity = opacityContext(el);
        const localBg = opacity.ancestors < 1
          ? backgroundWithin(el, opacity.outermostFaded?.parentElement)
          : effectiveBg(el);
        const painted = renderedPair(el, fg, localBg);
        const ratio = contrastRatio(painted.fg, painted.bg);
        const fontPx = parseFloat(cs.fontSize);
        const bold = (parseInt(cs.fontWeight, 10) || 400) >= 700;
        const large = fontPx >= 24 || (fontPx >= 18.66 && bold);
        if (painted.nestedOpacity) {
          contrastSigs.add(`nested-opacity|${sig(el)}`);
        }
        if (painted.ownBackgroundOpacity) {
          contrastSigs.add(`own-background-opacity|${sig(el)}`);
        }
        const min = large ? 3.0 : 4.5;
        if (ratio < min - 0.05) {
          contrastSigs.add(`${norm(painted.fg)}|${norm(painted.bg)}|${large ? "L" : "N"}|${ratio.toFixed(2)}`);
        }
      }
      const clip = (v) => v === "hidden" || v === "clip";
      const clippedX = clip(cs.overflowX) && el.scrollWidth > el.clientWidth + 1;
      const clippedY = clip(cs.overflowY) && el.scrollHeight > el.clientHeight + 1;
      if ((clippedX || clippedY) && cs.textOverflow !== "ellipsis") {
        clippedSigs.add(sig(el));
      }
    }

    if (checkTap) {
      const tag = el.tagName.toLowerCase();
      const role = el.getAttribute("role");
      const interactive =
        tag === "a" || tag === "button" || tag === "input" || tag === "select" ||
        tag === "textarea" || role === "button";
      if (interactive && (rect.width < 44 || rect.height < 44)) {
        tapSigs.add(`${sig(el)}@${Math.round(rect.width)}x${Math.round(rect.height)}`);
      }
    }
  }

  // SVG graph-label contrast: graph nodes/edges paint text via `fill`, which the
  // HTML pass above skips. Check each rendered <text> against the rect behind it,
  // compositing translucent fills over the canvas.
  const svgContrastSigs = new Set();
  const canvasEl = document.querySelector(".graph-canvas");
  const canvasBg = canvasEl ? effectiveBg(canvasEl) : { r: 20, g: 19, b: 16, a: 1 };
  for (const t of document.querySelectorAll("svg text")) {
    const text = (t.textContent || "").trim();
    if (!text) continue;
    const cs = getComputedStyle(t);
    if (cs.display === "none" || cs.visibility === "hidden") continue;
    const opacity = opacityContext(t);
    if (opacity.own * opacity.ancestors === 0) continue;
    const rect = t.getBoundingClientRect();
    if (rect.width <= 0 || rect.height <= 0) continue;
    const fg = parseColor(cs.fill);
    if (!fg || fg.a <= 0) continue;
    const fillOpacity = parseFloat(cs.fillOpacity);
    fg.a *= Number.isNaN(fillOpacity) ? 1 : Math.max(0, Math.min(1, fillOpacity));
    if (fg.a <= 0) continue;
    let bg = null;
    let g = t.parentElement;
    let hops = 0;
    while (g && hops < 4 && !bg) {
      for (const r of g.children) {
        if (r.tagName.toLowerCase() !== "rect") continue;
        const rectStyle = getComputedStyle(r);
        const rc = parseColor(rectStyle.fill);
        if (!rc || rc.a <= 0) continue;
        const fillOpacity = parseFloat(rectStyle.fillOpacity);
        const rectOpacity = parseFloat(rectStyle.opacity);
        rc.a *= (Number.isNaN(fillOpacity) ? 1 : fillOpacity) *
          (Number.isNaN(rectOpacity) ? 1 : rectOpacity);
        if (rc.a <= 0) continue;
        const rr = r.getBoundingClientRect();
        if (rr.left <= rect.left + 1 && rr.right >= rect.right - 1 && rr.top <= rect.top + 1 && rr.bottom >= rect.bottom - 1) {
          bg = opacity.ancestors < 1 ? rc : composite(rc, canvasBg, rc.a);
        }
      }
      g = g.parentElement;
      hops += 1;
    }
    if (!bg) bg = opacity.ancestors < 1 ? { r: 0, g: 0, b: 0, a: 0 } : canvasBg;
    const painted = renderedPair(t, fg, bg);
    const ratio = contrastRatio(painted.fg, painted.bg);
    const fontPx = parseFloat(cs.fontSize);
    const bold = (parseInt(cs.fontWeight, 10) || 400) >= 700;
    const large = fontPx >= 24 || (fontPx >= 18.66 && bold);
    const min = large ? 3.0 : 4.5;
    if (ratio < min - 0.05) {
      svgContrastSigs.add(`${norm(painted.fg)}|${norm(painted.bg)}|${large ? "L" : "N"}|${ratio.toFixed(2)}`);
    }
    if (painted.nestedOpacity) {
      svgContrastSigs.add(`nested-opacity|${sig(t)}`);
    }
    if (painted.ownBackgroundOpacity) {
      svgContrastSigs.add(`own-background-opacity|${sig(t)}`);
    }
  }

  const dimmedModule = document.querySelector(".node-shell.dimmed .node-module");
  const selectedModule = document.querySelector(".node-shell.selected .node-module");
  let inkAged = null;
  if (document.createElement) {
    const probe = document.createElement("span");
    probe.style.color = "var(--ink-aged)";
    probe.style.display = "none";
    document.body.appendChild(probe);
    inkAged = parseColor(getComputedStyle(probe).color);
    probe.remove();
  }
  const selectedColour = selectedModule ? parseColor(getComputedStyle(selectedModule).color) : null;
  const dimmedTextNodes = dimmedModule
    ? [dimmedModule, ...(dimmedModule.querySelectorAll
      ? dimmedModule.querySelectorAll(".node-id, .node-name, .node-description")
      : [])]
    : [];
  const dimmedTextPasses = dimmedTextNodes.length > 0 && dimmedTextNodes.every((node) => {
    const colour = parseColor(getComputedStyle(node).color);
    const bg = effectiveBg(node);
    return !!colour && !!inkAged &&
      norm(colour) === norm(inkAged) &&
      contrastRatio(colour, bg) >= 4.5;
  });
  const dimmedNodeRecessed =
    dimmedTextPasses &&
    !!selectedColour &&
    !!inkAged &&
    norm(inkAged) !== norm(selectedColour);
  // Accessible-name contract: "<node id>, <state label>", where the label is
  // the rendered copy.toml string for the module's own state class (Synced,
  // Planned for ghost, Orphaned, Drift). Derived from the class so a wrong
  // or stale label fails; a copy-load failure yields a bare id and fails too.
  const stateNamedModules = Array.from(document.querySelectorAll(".node-module"));
  const nodeStateNamed =
    stateNamedModules.length > 0 &&
    stateNamedModules.every((mod) => {
      const id = mod.getAttribute("data-node-id") || "";
      const label = mod.getAttribute("aria-label") || "";
      const state = ["synced", "ghost", "orphaned", "drift"].find((cls) => mod.classList.contains(cls));
      if (!state) return false;
      const expected = state === "ghost" ? "planned" : state;
      return label.toLowerCase() === `${id.toLowerCase()}, ${expected}`;
    });

  // Wire-legibility helper (todo.console-wire-legibility): rendered, laid
  // out (offsetParent), non-empty, and not clipped by its own box in either
  // axis. Ellipsis clamps and clipped wraps fail the scroll/client compare.
  const visibleUntruncated = (el) =>
    !!el && !!el.offsetParent && !!(el.textContent || "").trim() && el.scrollWidth <= el.clientWidth + 1 && el.scrollHeight <= el.clientHeight + 1;

  const landmarks = {
    shell: !!document.querySelector(".instrument-shell"),
    statusBezel: !!document.querySelector(".status-bezel"),
    queryRail: !!document.querySelector(".query-rail"),
    queryInput: !!document.querySelector(".query-input"),
    queryInputValue: (document.querySelector(".query-input")?.value || "").trim(),
    graphCanvas: !!document.querySelector(".graph-canvas"),
    graphSvg: !!document.querySelector(".graph-svg"),
    nodeModules: document.querySelectorAll(".node-module").length,
    selectedNode: document.querySelectorAll(".node-module.selected").length,
    nodeStateNamed,
    dimmedNodeRecessed,
    evidenceRail: !!document.querySelector(".evidence-rail"),
    depthPlate: !!document.querySelector(".node-depth-plate"),
    blueprintPlate: !!document.querySelector(".blueprint-plate"),
    lineagePlate: !!document.querySelector(".lineage-plate"),
    lineageDecisions: document.querySelectorAll(".lineage-stage:nth-of-type(2) .query-chip").length,
    channelBar: !!document.querySelector(".channel-bar"),
    activeFindings: !!document.querySelector('.channel-tab.active'),
    channelItems: document.querySelectorAll(".channel-item").length,
    backlogTierSections: document.querySelectorAll(".channel-tier").length,
    backlogParentGroup: !!document.querySelector(".channel-group.has-parent .channel-group-header"),
    consoleLanes: document.querySelectorAll(".console-lane").length,
    consolePendingRows: document.querySelectorAll(".console-lane-pending .channel-item .channel-code").length,
    consoleFrontierEmpty: !!document.querySelector(".console-frontier-empty"),
    consoleRoadmapTier: !!document.querySelector(".console-lane-roadmap .channel-tier"),
    pendingRows: document.querySelectorAll(".channel-bar .pending-item").length,
    pendingDetail: !!document.querySelector(".channel-bar .pending-detail"),
    pendingPrompt: !!document.querySelector(".channel-bar .pending-detail-prompt"),
    pendingRubric: !!document.querySelector(".channel-bar .pending-detail-tier"),
    pendingEvidence: !!document.querySelector(".channel-bar .pending-detail-evidence"),
    pendingReopen: !!document.querySelector(".channel-bar .pending-detail-reopen code"),
    // Wire-legibility landmarks (todo.console-wire-legibility): evidence the
    // browser downloaded must be rendered, visible, and untruncated. When the
    // harness injects the fixture's expected title, the rendered text must
    // match it, so a placeholder cannot satisfy the check.
    nextRecommendedTitle: (() => {
      const el = document.querySelector(".status-bezel .status-next-title");
      if (!visibleUntruncated(el)) return false;
      const expected = String(window.__evalExpectedNextTitle || "").trim();
      return !expected || (el.textContent || "").trim() === expected;
    })(),
    nextRecommendedRule: visibleUntruncated(document.querySelector(".status-bezel .status-next-rule")),
    bezelCleanQualified: !!document.querySelector('.status-annunciator[data-drift-state="clean-qualified"]'),
    bezelSeveritySummary: visibleUntruncated(document.querySelector(".status-annunciator-summary")),
    // Backlog rows must print the todo title (prose with spaces) through the
    // prose-title class, not the todo.* filename stem, and every prose title
    // must be visually untruncated.
    backlogTitleProse: (() => {
      const prose = [...document.querySelectorAll(".channel-tier .channel-item .channel-code.channel-title-prose")].filter((el) => /\s/.test((el.textContent || "").trim()));
      return prose.length > 0 && prose.every(visibleUntruncated);
    })(),
    // A COLLAPSED pending row (no expanded detail inside it) must show the
    // ruling summary and the rubric tier, wrapped rather than truncated.
    pendingCollapsedSummary: [...document.querySelectorAll(".pending-item")]
      .filter((row) => !row.querySelector(".pending-detail"))
      .some((row) => {
        const summary = row.querySelector(".pending-collapsed .pending-detail-summary");
        const tier = row.querySelector(".pending-collapsed .pending-detail-tier");
        return visibleUntruncated(summary) && visibleUntruncated(tier);
      }),
  };

  return {
    vw,
    vh,
    scrollW: docEl.scrollWidth,
    scrollH: docEl.scrollHeight,
    overflow: Math.max(0, docEl.scrollWidth - vw),
    contrast: contrastSigs.size,
    clipped: clippedSigs.size,
    offscreen: offscreenSigs.size,
    tap: tapSigs.size,
    palette: paletteViol.size,
    textElements,
    svgContrast: svgContrastSigs.size,
    landmarks,
    detail: {
      contrast: [...contrastSigs],
      clipped: [...clippedSigs],
      offscreen: [...offscreenSigs],
      tap: [...tapSigs],
      palette: [...paletteViol],
      svgContrast: [...svgContrastSigs],
    },
  };
}
