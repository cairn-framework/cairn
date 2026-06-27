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
    let node = el;
    while (node && node !== document.documentElement) {
      const c = parseColor(getComputedStyle(node).backgroundColor);
      if (c && c.a >= 0.999) return c;
      node = node.parentElement;
    }
    const bodyC = parseColor(getComputedStyle(document.body).backgroundColor);
    if (bodyC && bodyC.a >= 0.999) return bodyC;
    return { r: 20, g: 19, b: 16, a: 1 }; // stone-1 fallback
  }
  function isVisible(el, cs, rect) {
    if (cs.display === "none" || cs.visibility === "hidden") return false;
    if (parseFloat(cs.opacity) === 0) return false;
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
        const bg = effectiveBg(el);
        const ratio = contrastRatio(fg, bg);
        const fontPx = parseFloat(cs.fontSize);
        const bold = (parseInt(cs.fontWeight, 10) || 400) >= 700;
        const large = fontPx >= 24 || (fontPx >= 18.66 && bold);
        const min = large ? 3.0 : 4.5;
        if (ratio < min - 0.05) {
          contrastSigs.add(`${norm(fg)}|${norm(bg)}|${large ? "L" : "N"}|${ratio.toFixed(2)}`);
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
  // HTML pass above skips. The graph is the heart of the tool, so check each
  // rendered <text> against the rect that paints behind it (the node card),
  // falling back to the canvas background for edge labels and dividers.
  const svgContrastSigs = new Set();
  const canvasEl = document.querySelector(".graph-canvas");
  const canvasBg =
    (canvasEl && parseColor(getComputedStyle(canvasEl).backgroundColor)) || { r: 20, g: 19, b: 16, a: 1 };
  for (const t of document.querySelectorAll("svg text")) {
    const text = (t.textContent || "").trim();
    if (!text) continue;
    const cs = getComputedStyle(t);
    if (cs.display === "none" || cs.visibility === "hidden" || parseFloat(cs.opacity) === 0) continue;
    const rect = t.getBoundingClientRect();
    if (rect.width <= 0 || rect.height <= 0) continue;
    const fg = parseColor(cs.fill);
    if (!fg || fg.a <= 0) continue;
    let bg = null;
    let g = t.parentElement;
    let hops = 0;
    while (g && hops < 4 && !bg) {
      for (const r of g.children) {
        if (r.tagName.toLowerCase() !== "rect") continue;
        const rc = parseColor(getComputedStyle(r).fill);
        if (!rc || rc.a < 0.5) continue;
        const rr = r.getBoundingClientRect();
        if (rr.left <= rect.left + 1 && rr.right >= rect.right - 1 && rr.top <= rect.top + 1 && rr.bottom >= rect.bottom - 1) {
          bg = rc;
        }
      }
      g = g.parentElement;
      hops += 1;
    }
    if (!bg) bg = canvasBg;
    const ratio = contrastRatio(fg, bg);
    const fontPx = parseFloat(cs.fontSize);
    const bold = (parseInt(cs.fontWeight, 10) || 400) >= 700;
    const large = fontPx >= 24 || (fontPx >= 18.66 && bold);
    const min = large ? 3.0 : 4.5;
    if (ratio < min - 0.05) {
      svgContrastSigs.add(`${norm(fg)}|${norm(bg)}|${large ? "L" : "N"}|${ratio.toFixed(2)}`);
    }
  }

  const landmarks = {
    inspector: !!document.querySelector(".inspector"),
    emptyInspector: !!document.querySelector(".empty-inspector"),
    graphSvg: !!document.querySelector(".graph-svg"),
    miniDots: document.querySelectorAll(".graph-minimap .mini-dot").length,
    statGrid: !!document.querySelector(".stat-grid"),
    insTitle: !!document.querySelector(".ins-title"),
    blueprintCard: !!document.querySelector(".blueprint-card"),
    drawerOpen: !!document.querySelector(".changes-drawer .drawer-body, .changes-drawer .drawer-empty"),
    cmdPalette: !!document.querySelector(".cmd-palette"),
    blueprintModal: !!document.querySelector(".blueprint-modal"),
    decisionDetail: !!document.querySelector(".decision-detail"),
    changeCards: document.querySelectorAll(".changes-drawer .change-card").length,
    proseNudge: !!document.querySelector(".prose-nudge"),
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
