// Unit coverage for the parts of `auditPage` whose arithmetic decides whether a
// reported contrast ratio matches the pixels a reader actually sees.
//
// `auditPage` normally runs inside Chrome. Here it runs against a minimal DOM
// double that implements only the surface the audit touches, so the composited
// contrast arithmetic is asserted without a browser and without fixtures.
//
// Run: node --test harness/lib/

import assert from "node:assert/strict";
import { after, before, test } from "node:test";

import { auditPage } from "./audit.mjs";

/** Element double. Only the members `auditPage` reads are implemented. */
class El {
  constructor(tag, { classes = [], style = {}, text = "", rect = null, attrs = {} } = {}) {
    this.nodeType = 1;
    this.tagName = tag.toUpperCase();
    this.className = classes.join(" ");
    this.classSet = new Set(classes);
    this.style = style;
    this.text = text;
    this.attrs = attrs;
    this.parentElement = null;
    this.children = [];
    this.rect = rect || { left: 0, top: 0, right: 100, bottom: 20, width: 100, height: 20 };
    this.scrollWidth = this.rect.width;
    this.clientWidth = this.rect.width;
    this.scrollHeight = this.rect.height;
    this.clientHeight = this.rect.height;
  }

  append(...kids) {
    for (const kid of kids) {
      kid.parentElement = this;
      this.children.push(kid);
    }
    return this;
  }

  get childNodes() {
    const nodes = this.text ? [{ nodeType: 3, textContent: this.text }] : [];
    return nodes.concat(this.children);
  }

  get textContent() {
    return this.text + this.children.map((c) => c.textContent).join("");
  }

  getBoundingClientRect() {
    return this.rect;
  }

  getAttribute(name) {
    return name in this.attrs ? this.attrs[name] : null;
  }

  closest(selector) {
    let node = this;
    while (node) {
      if (matchesToken(node, selector)) return node;
      node = node.parentElement;
    }
    return null;
  }

  descendants() {
    return this.children.flatMap((c) => [c, ...c.descendants()]);
  }
}

/** Match one compound token: `tag`, `.cls`, or `tag.cls.cls`. */
function matchesToken(el, token) {
  const clean = token.replace(/:[a-z-]+(\([^)]*\))?/g, "").trim();
  if (!clean) return false;
  const parts = clean.split(".");
  const tag = parts.shift();
  if (tag && el.tagName !== tag.toUpperCase()) return false;
  return parts.every((cls) => el.classSet.has(cls));
}

/**
 * Match the LAST compound token of a descendant selector. `auditPage` only uses
 * descendant selectors for landmark presence, which these tests do not assert,
 * so ignoring the ancestor part is safe and keeps the double small.
 */
function matches(el, selector) {
  const tokens = selector.trim().split(/\s+/);
  return matchesToken(el, tokens[tokens.length - 1]);
}

function makeDocument(body) {
  const all = body.descendants();
  const query = (selector) => {
    if (selector === "body *") return all;
    if (selector === "svg text") return all.filter((el) => el.tagName === "TEXT" && el.closest("svg"));
    return all.filter((el) => matches(el, selector));
  };
  const documentElement = new El("html");
  documentElement.append(body);
  documentElement.scrollWidth = 1440;
  documentElement.scrollHeight = 900;
  return {
    documentElement,
    scrollingElement: documentElement,
    body,
    querySelector: (selector) => query(selector)[0] || null,
    querySelectorAll: (selector) => query(selector),
  };
}

const DEFAULT_STYLE = {
  display: "block",
  visibility: "visible",
  opacity: "1",
  color: "rgb(232,228,218)",
  backgroundColor: "rgba(0, 0, 0, 0)",
  fill: "rgb(232,228,218)",
  fontSize: "13px",
  fontWeight: "400",
  overflowX: "visible",
  overflowY: "visible",
  textOverflow: "clip",
  borderTopWidth: "0px",
  borderRightWidth: "0px",
  borderBottomWidth: "0px",
  borderLeftWidth: "0px",
};

const CHALK = "rgb(232,228,218)"; // --ink-char
const STONE_3 = "rgb(46,45,41)"; // --stone-3, the node card surface
const STONE_1 = "rgb(33,31,28)"; // --stone-1, audit fallback
const STONE_2 = "rgb(40,39,35)"; // --stone-2, the graph canvas

const LIGHT_INK_AGED = "rgb(98,94,84)";
const LIGHT_STONE_2 = "rgb(231,226,216)";
const LIGHT_GHOST_WASH = "rgba(108,101,89,0.2)";
const PALETTE = [CHALK, STONE_3, STONE_1, STONE_2, LIGHT_INK_AGED, LIGHT_STONE_2, LIGHT_GHOST_WASH];

let savedGlobals;

before(() => {
  savedGlobals = {
    window: globalThis.window,
    document: globalThis.document,
    getComputedStyle: globalThis.getComputedStyle,
  };
  globalThis.window = { innerWidth: 1440, innerHeight: 900 };
  globalThis.getComputedStyle = (el) => ({ ...DEFAULT_STYLE, ...el.style });
});

after(() => {
  globalThis.window = savedGlobals.window;
  globalThis.document = savedGlobals.document;
  globalThis.getComputedStyle = savedGlobals.getComputedStyle;
});

/** A chalk label on a node card, inside a shell faded to `shellOpacity`. */
function renderDomTree(shellOpacity, {
  labelColour = CHALK,
  cardBackground = STONE_3,
  canvasBackground = STONE_2,
} = {}) {
  const label = new El("p", {
    classes: ["node-id"],
    style: { color: labelColour },
    text: "cairn.ui",
  });
  const card = new El("button", {
    classes: ["node-module"],
    style: { backgroundColor: cardBackground },
  }).append(label);
  const shell = new El("div", {
    classes: ["node-shell", "dimmed"],
    style: { opacity: String(shellOpacity) },
  }).append(card);
  const canvas = new El("div", {
    classes: ["graph-canvas"],
    style: { backgroundColor: canvasBackground },
  }).append(shell);
  return new El("body", { style: { backgroundColor: canvasBackground } }).append(canvas);
}

function renderNestedDomTree() {
  const body = renderDomTree(0.5);
  const canvas = body.children[0];
  const innerShell = canvas.children[0];
  const outerGroup = new El("div", { style: { opacity: "0.5" } }).append(innerShell);
  outerGroup.parentElement = canvas;
  canvas.children = [outerGroup];
  return body;
}

function renderOwnBackgroundOpacityDomTree() {
  const body = renderDomTree(1);
  const label = body.children[0].children[0].children[0].children[0];
  label.style.color = "rgb(0,0,0)";
  label.style.backgroundColor = "rgb(255,255,255)";
  label.style.opacity = "0.5";
  return body;
}

function renderSvgTree(groupOpacity, {
  labelColour = CHALK,
  labelFillOpacity = "1",
  rectFill = STONE_3,
  canvasBackground = STONE_2,
} = {}) {
  const rect = new El("rect", {
    style: { fill: rectFill },
    rect: { left: 0, top: 0, right: 120, bottom: 40, width: 120, height: 40 },
  });
  const label = new El("text", {
    style: { fill: labelColour, fillOpacity: labelFillOpacity },
    text: "cairn.ui",
    rect: { left: 10, top: 10, right: 90, bottom: 26, width: 80, height: 16 },
  });
  const group = new El("g", { style: { opacity: String(groupOpacity) } }).append(rect, label);
  const svg = new El("svg", { classes: ["graph-svg"] }).append(group);
  const canvas = new El("div", {
    classes: ["graph-canvas"],
    style: { backgroundColor: canvasBackground },
  }).append(svg);
  return new El("body", { style: { backgroundColor: canvasBackground } }).append(canvas);
}

function run(body) {
  globalThis.document = makeDocument(body);
  return auditPage({ palette: PALETTE, checkTap: false });
}

function ratioOf(signature) {
  return Number(signature.split("|")[3]);
}

test("DOM pass composites ancestor opacity into the reported ratio", () => {
  const audit = run(renderDomTree(0.55));
  assert.equal(audit.contrast, 1, "faded chalk on a node card is a contrast defect");
  // CSS group opacity flattens both the chalk foreground and --stone-3 card
  // onto the --stone-2 canvas. The resulting pixels measure 4.42:1, matching
  // the oracle that exposed this defect. Measuring either declared colour
  // instead reports 10.86 and makes the defect disappear.
  assert.ok(
    Math.abs(ratioOf(audit.detail.contrast[0]) - 4.42) < 0.02,
    `expected ~4.42:1, got ${audit.detail.contrast[0]}`,
  );
  assert.match(audit.detail.contrast[0], /^rgb\(146,143,136\)\|rgb\(43,42,38\)\|N\|/);
});

test("DOM pass leaves an unfaded ancestor chain alone", () => {
  const audit = run(renderDomTree(1));
  assert.equal(audit.contrast, 0, "chalk on --stone-3 is 10.86:1 and passes");
});

test("DOM pass composites a translucent card over its backdrop", () => {
  const audit = run(renderDomTree(1, {
    labelColour: LIGHT_INK_AGED,
    cardBackground: LIGHT_GHOST_WASH,
    canvasBackground: LIGHT_STONE_2,
  }));
  assert.equal(audit.contrast, 1, "light-theme ghost text is a contrast defect");
  assert.match(audit.detail.contrast[0], /^rgb\(98,94,84\)\|rgb\(206,201,191\)\|N\|3\.92$/);
});

test("DOM pass skips text faded to nothing by an ancestor", () => {
  const audit = run(renderDomTree(0));
  assert.equal(audit.contrast, 0, "invisible text is not a contrast defect");
  assert.equal(audit.textElements, 0);
});

test("DOM pass fails closed on nested opacity groups", () => {
  const audit = run(renderNestedDomTree());
  assert.ok(
    audit.detail.contrast.some((signature) => signature.startsWith("nested-opacity|")),
    "nested opacity must not produce a false passing ratio",
  );
});

test("DOM pass fails closed on own opacity with a painted background", () => {
  const audit = run(renderOwnBackgroundOpacityDomTree());
  assert.ok(
    audit.detail.contrast.some((signature) => signature.startsWith("own-background-opacity|")),
    "own opacity over an element background must not produce a false ratio",
  );
});

test("SVG pass composites ancestor opacity into the reported ratio", () => {
  const audit = run(renderSvgTree(0.55));
  assert.equal(audit.svgContrast, 1, "faded graph label is a contrast defect");
  assert.ok(
    Math.abs(ratioOf(audit.detail.svgContrast[0]) - 4.42) < 0.02,
    `expected ~4.42:1, got ${audit.detail.svgContrast[0]}`,
  );
  assert.match(audit.detail.svgContrast[0], /^rgb\(146,143,136\)\|rgb\(43,42,38\)\|N\|/);
});

test("SVG pass composites a translucent card fill over its backdrop", () => {
  const audit = run(renderSvgTree(1, {
    labelColour: LIGHT_INK_AGED,
    rectFill: LIGHT_GHOST_WASH,
    canvasBackground: LIGHT_STONE_2,
  }));
  assert.equal(audit.svgContrast, 1, "light-theme ghost SVG text is a contrast defect");
  assert.match(audit.detail.svgContrast[0], /^rgb\(98,94,84\)\|rgb\(206,201,191\)\|N\|3\.92$/);
});


test("SVG pass applies text fill-opacity to the foreground", () => {
  const audit = run(renderSvgTree(1, { labelFillOpacity: "0.2" }));
  assert.equal(audit.svgContrast, 1, "translucent SVG text is a contrast defect");
  assert.match(audit.detail.svgContrast[0], /^rgb\(83,82,76\)\|rgb\(46,45,41\)\|N\|1\.75$/);
});
test("SVG pass leaves an unfaded ancestor chain alone", () => {
  const audit = run(renderSvgTree(1));
  assert.equal(audit.svgContrast, 0, "chalk on --stone-3 is 10.86:1 and passes");
});
