// Cairn webui visual-eval harness (autoresearch workload).
//
// Pipeline: serve the live webui against frozen API fixtures -> drive headless
// Chrome through CDP across a fixed set of viewport/interaction scenarios ->
// capture a screenshot of each -> run a deterministic, DOM-grounded visual eval
// plus a pixel-level blank/clutter check on each screenshot -> aggregate into a
// single UX-defect score (lower is better).
//
// Output: `METRIC <name>=<value>` lines on stdout, a JSON report at
// harness/out/report.json, and PNG screenshots at harness/out/screenshots/.
// Deterministic: no live network (web fonts blocked, data frozen), fixed
// viewports, reduced-motion emulated, animations disabled before capture.

import { copyFileSync, cpSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { setTimeout as sleep } from "node:timers/promises";

import { CDP, launchChrome, newPageSession } from "./lib/cdp.mjs";
import { decodePng, imageStats } from "./lib/png.mjs";
import { auditPage, parsePalette } from "./lib/audit.mjs";
import { startReplayServer } from "./server.mjs";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const FIXTURES = join(ROOT, "harness/fixtures");
const OUT = join(ROOT, "harness/out");
const SHOTS = join(OUT, "screenshots");

const FONT_BLOCKLIST = [
  "*fonts.googleapis.com*",
  "*fonts.gstatic.com*",
  "*://fonts.*",
];

// Defect weights. Accessibility (contrast) and broken responsive layout
// (overflow/offscreen) dominate; palette drift is a light nudge; a blank or
// landmark-missing render is treated as a severe regression (anti-gaming: you
// cannot win by deleting the UI).
const WEIGHTS = {
  contrast: 3,
  svgContrast: 3,
  overflow: 5,
  offscreen: 2,
  clipped: 2,
  tap: 1,
  palette: 1,
  blank: 50,
  landmark: 40,
  action: 40,
};

const SCENARIOS = [
  { name: "overview-desktop", width: 1440, height: 900, mobile: false },
  { name: "inspector-desktop", width: 1440, height: 900, mobile: false, action: "selectNode" },
  { name: "findings-desktop", width: 1440, height: 900, mobile: false, action: "openFindings" },
  { name: "tablet-portrait", width: 834, height: 1112, mobile: true, checkTap: true },
  { name: "mobile-portrait", width: 390, height: 844, mobile: true, checkTap: true },
  { name: "command-palette", width: 1440, height: 900, mobile: false, action: "openPalette" },
  { name: "blueprint-mode", width: 1440, height: 900, mobile: false, select: "cairn.root", action: "openBlueprint" },
  // Functional-state coverage: paths the clean self-scan never produces. The
  // "demo" server serves harness/fixtures-demo (findings + ghost/orphaned).
  { name: "decision-lineage", width: 1440, height: 900, mobile: false, select: "cairn.root", action: "openDecision" },
  { name: "findings-populated", width: 1440, height: 900, mobile: false, server: "demo", action: "openFindings", requireFindings: true },
  { name: "node-finding", width: 1440, height: 900, mobile: false, server: "demo", action: "openFinding", requireFindings: true },
  // "cairn.sse" is a fixture-only ghost node id used to keep the ghost-state
  // scenario meaningful after a live graph refresh.
  { name: "ghost-node", width: 1440, height: 900, mobile: false, server: "demo", select: "cairn.sse" },
];

const READY_EXPR =
  "(function(){try{return !!(document.querySelector('.instrument-shell')&&document.querySelector('.graph-svg')&&document.querySelectorAll('.node-module').length>0);}catch(e){return false;}})()";

const KILL_ANIM_EXPR =
  "(function(){var s=document.createElement('style');s.textContent='*,*::before,*::after{transition:none!important;animation:none!important;animation-duration:0s!important;animation-delay:0s!important;}';document.head.appendChild(s);return true;})()";

const ACTIONS = {
  selectNode: {
    // Click a node that is NOT currently selected and demand the selection
    // actually moves to it, so a pre-selected default cannot satisfy the check.
    fire: "(function(){var mods=[...document.querySelectorAll('.node-module')];var target=mods.find(function(n){return !n.classList.contains('selected');})||mods[0];if(!target){return false;}window.__evalSelectTarget=target.getAttribute('title')||'';target.click();return !!window.__evalSelectTarget;})()",
    settled: "(function(){var expected=String(window.__evalSelectTarget||'');var selected=document.querySelector('.node-module.selected');return !!expected&&!!selected&&selected.getAttribute('title')===expected&&!!document.querySelector('.evidence-rail .node-depth-plate .node-id');})()",
  },
  openFindings: {
    // Findings is the default channel, so prove the switching machinery in two
    // observed phases: switch away and require the other tab to become active,
    // then switch back and require findings active with a rendered body.
    fire: "(function(){var tabs=[...document.querySelectorAll('.channel-bar .channel-tab')];var findings=tabs.find((n)=>String(n.textContent||'').toLowerCase().includes('findings'));var other=tabs.find((n)=>n!==findings);if(!findings||!other){return false;}window.__evalFindingsPhase=1;other.click();return true;})()",
    settled: "(function(){var tabs=[...document.querySelectorAll('.channel-bar .channel-tab')];var findings=tabs.find((n)=>String(n.textContent||'').toLowerCase().includes('findings'));var other=tabs.find((n)=>n!==findings);if(!findings||!other){return false;}var phase=Number(window.__evalFindingsPhase||0);if(phase===1){if(!other.classList.contains('active')){return false;}window.__evalFindingsPhase=2;findings.click();return false;}if(phase===2){return findings.classList.contains('active')&&!!document.querySelector('.channel-bar .channel-body')&&!!(document.querySelector('.channel-item')||document.querySelector('.channel-empty'));}return false;})()",
  },
  openPalette: {
    fire: "(function(){var input=document.querySelector('.query-input');if(!input){return false;}input.focus();input.value='kernel';input.dispatchEvent(new Event('input',{bubbles:true}));input.dispatchEvent(new KeyboardEvent('keydown',{bubbles:true,key:'Enter',code:'Enter',keyCode:13,which:13}));return true;})()",
    settled: "(function(){var input=document.querySelector('.query-input');if(!input||input.value!=='kernel'){return false;}var matchesText=Array.from(document.querySelectorAll('.query-chip')).map((n)=>String(n.textContent||'').trim()).find((text)=>/\\b\\d+\\b/.test(text)&&text.includes(':'));if(!matchesText){return false;}var matches=Number((matchesText.match(/(\\d+)/)||['',-1])[1]);var base=Number(window.__evalExpectedPaletteBaseCount);var count=document.querySelectorAll('.node-module').length;return Number.isFinite(matches)&&matches>=0&&count>0&&count===matches&&(Number.isFinite(base)?count<=base&&count>0:count>=0);})()",
  },
  openFinding: {
    fire: "(function(){var expected=String(window.__evalExpectedFindingNode||'').trim();var rows=[...document.querySelectorAll('.channel-item')];if(!expected||!rows.length){return false;}var target=rows.find((n)=>String(n.textContent||'').includes(expected));if(!target){return false;}var action=target.querySelector('.query-action');if(!action){return false;}action.click();return true;})()",
    settled: "(function(){var expected=String(window.__evalExpectedFindingNode||'');var selected=document.querySelector('.node-module.selected');return !!expected&&!!selected&&selected.getAttribute('title')===expected;})()",
  },
  openBlueprint: {
    fire: "(function(){var rail=document.querySelector('.evidence-rail');if(!rail){return false;}var t=[...rail.querySelectorAll('.rail-tab')].find((n)=>String(n.textContent||'').toLowerCase().includes('blueprint'));if(!t){return false;}t.click();return true;})()",
    settled: "(function(){return !!document.querySelector('.evidence-rail .blueprint-plate');})()",
  },
  openDecision: {
    fire: "(function(){var rail=document.querySelector('.evidence-rail');if(!rail){return false;}var t=[...rail.querySelectorAll('.rail-tab')].find((n)=>String(n.textContent||'').toLowerCase().includes('lineage'));if(!t){return false;}t.click();return true;})()",
    settled: "(function(){var expected=String(window.__evalExpectedDecisionTitle||'').trim();var rail=document.querySelector('.evidence-rail');if(!rail){return false;}var stages=Array.from(rail.querySelectorAll('.lineage-stage'));if(stages.length<2){return false;}var choices=Array.from(stages[1].querySelectorAll('.query-chip'));if(!choices.length){return false;}if(!expected){window.__evalExpectedDecisionTitle=String(choices[0].textContent||'').trim();if(!window.__evalExpectedDecisionTitle){return false;}choices[0].click();return false;}var preview=document.querySelector('.lineage-plate .lineage-title');if(preview&&String(preview.textContent||'').trim()===expected){return true;}var target=choices.find((n)=>String(n.textContent||'').trim()===expected);if(!target){return false;}target.click();return false;})()",
  },
};

function missingLandmarks(scenario, lm) {
  const miss = [];
  if (!lm.shell) miss.push("shell");
  if (!lm.graphCanvas) miss.push("graphCanvas");
  if (!lm.graphSvg) miss.push("graphSvg");
  if (!lm.queryRail) miss.push("queryRail");
  if (!lm.nodeModules) miss.push("nodeModules");
  if (scenario.action === "selectNode" || scenario.select) {
    if (!lm.selectedNode) miss.push("selectedNode");
  }
  if (scenario.action === "openFindings" || scenario.action === "openFinding") {
    if (!lm.channelBar) miss.push("channelBar");
    if (scenario.requireFindings && !(lm.channelItems > 0)) miss.push("channelItems");
  }
  if (scenario.action === "openPalette") {
    if (!lm.queryRail) miss.push("queryRail");
    if (!lm.queryInput) miss.push("queryInput");
    if (!lm.queryInputValue) miss.push("queryInputValue");
  }
  if (scenario.action === "openBlueprint") {
    if (!lm.evidenceRail) miss.push("evidenceRail");
    if (!lm.blueprintPlate) miss.push("blueprintPlate");
  }
  if (scenario.action === "openDecision") {
    if (!lm.evidenceRail) miss.push("evidenceRail");
    if (!lm.lineagePlate) miss.push("lineagePlate");
  }
  return miss;
}


async function main() {
  mkdirSync(SHOTS, { recursive: true });
  const demoLint = String(readFileSync(join(ROOT, "harness/fixtures-demo/api/lint"), "utf8"));
  let demoFindingNode = "";
  try {
    const parsed = JSON.parse(demoLint);
    const firstFindingWithNode = Array.isArray(parsed?.findings) ? parsed.findings.find((finding) => typeof finding?.node === "string" && finding.node.trim().length) : null;
    if (firstFindingWithNode && typeof firstFindingWithNode.node === "string") {
      demoFindingNode = firstFindingWithNode.node.trim();
    }
  } catch {
    demoFindingNode = "";
  }

  const scenarios = SCENARIOS.map((scenario) =>
    scenario.action === "openFinding" ? { ...scenario, expectedFindingNode: demoFindingNode || undefined } : scenario,
  );

  const palette = [...parsePalette(readFileSync(join(ROOT, "docs/design-system/tokens.css"), "utf8"))];

  const replay = await startReplayServer({ root: ROOT, fixturesDir: FIXTURES, port: 0 });
  // Build the demo fixture set (committed overlay applied over the base) into
  // the gitignored OUT dir and serve it on a second port. Functional-state
  // scenarios (findings populated, ghost/orphaned) run against this server.
  const DEMO_FIXTURES = join(OUT, "demo-fixtures");
  cpSync(FIXTURES, DEMO_FIXTURES, { recursive: true });
  for (const f of ["graph", "status", "lint"]) {
    copyFileSync(join(ROOT, "harness/fixtures-demo/api", f), join(DEMO_FIXTURES, "api", f));
  }
  const demoReplay = await startReplayServer({ root: ROOT, fixturesDir: DEMO_FIXTURES, port: 0 });
  const chrome = await launchChrome();
  let client;
  let sessionId;
  try {
    client = await CDP.connect(chrome.wsUrl);
    ({ sessionId } = await newPageSession(client));
    await client.send("Page.enable", {}, sessionId);
    await client.send("Runtime.enable", {}, sessionId);
    await client.send("Network.enable", {}, sessionId);
    await client.send("Network.setBlockedURLs", { urls: FONT_BLOCKLIST }, sessionId);
    await client.send("Emulation.setEmulatedMedia", {
      features: [{ name: "prefers-reduced-motion", value: "reduce" }],
    }, sessionId);

    const evalJs = async (expression) => {
      const r = await client.send(
        "Runtime.evaluate",
        { expression, returnByValue: true, awaitPromise: true },
        sessionId,
      );
      if (r.exceptionDetails) {
        throw new Error(
          `page eval threw: ${r.exceptionDetails.exception?.description || r.exceptionDetails.text}`,
        );
      }
      return r.result.value;
    };
    const waitUntil = async (expr, timeout = 8000, interval = 100) => {
      const end = Date.now() + timeout;
      while (Date.now() < end) {
        if (await evalJs(expr)) return true;
        await sleep(interval);
      }
      return false;
    };

    const results = [];
    for (const scenario of scenarios) {
      await client.send("Emulation.setDeviceMetricsOverride", {
        width: scenario.width,
        height: scenario.height,
        deviceScaleFactor: 1,
        mobile: scenario.mobile,
        screenWidth: scenario.width,
        screenHeight: scenario.height,
      }, sessionId);

      const srv = scenario.server === "demo" ? demoReplay : replay;

      // The SPA persists node selection in localStorage; clear per-origin state
      // before each navigation so scenarios are isolated and order-independent.
      try {
        await client.send("Storage.clearDataForOrigin", {
          origin: srv.url,
          storageTypes: "local_storage,session_storage",
        }, sessionId);
      } catch {
        /* Storage domain unavailable; best effort */
      }

      await client.send("Page.navigate", { url: `${srv.url}/` }, sessionId);
      let ready = await waitUntil(READY_EXPR, 10000);

      if (ready && scenario.select) {
        await evalJs(
          `(function(){localStorage.setItem('cairn:ui:selection',${JSON.stringify(scenario.select)});localStorage.setItem('cairn:v2:selection',${JSON.stringify(scenario.select)});location.reload();return true;})()`,
        );
        ready = await waitUntil(READY_EXPR, 12000);
        if (ready) {
          const selectedExpr = `(function(){var expected=${JSON.stringify(scenario.select)};var selected=document.querySelector('.node-module.selected');return !!selected && String(selected.getAttribute('title')||'')===String(expected);})()`;
          ready = await waitUntil(selectedExpr, 12000);
        }
      }

      let actionPassed = true;
      let actionFailed = false;
      if (ready) {
        await evalJs(KILL_ANIM_EXPR);
        if (scenario.action) {
          await evalJs(
            "delete window.__evalExpectedFindingNode; delete window.__evalExpectedPaletteBaseCount; delete window.__evalExpectedDecisionTitle;",
          );
          if (scenario.action === "openFinding") {
            await evalJs(`window.__evalExpectedFindingNode=${JSON.stringify(scenario.expectedFindingNode || "")};`);
          } else if (scenario.action === "openPalette") {
            await evalJs("window.__evalExpectedPaletteBaseCount=document.querySelectorAll('.node-module').length;");
          }

          const action = ACTIONS[scenario.action];
          const fired = !!(await evalJs(action.fire));
          const settled = fired ? await waitUntil(action.settled, 6000) : false;
          actionPassed = fired && settled;
          actionFailed = !actionPassed;
          await sleep(350); // settle layout/render before capture
        } else {
          await sleep(350); // settle layout/render before capture
        }
      }

      const audit = await evalJs(
        `(${auditPage.toString()})(${JSON.stringify({ palette, checkTap: !!scenario.checkTap })})`,
      );

      const shot = await client.send("Page.captureScreenshot", {
        format: "png",
        captureBeyondViewport: false,
      }, sessionId);
      const png = Buffer.from(shot.data, "base64");
      const shotPath = join(SHOTS, `${scenario.name}.png`);
      writeFileSync(shotPath, png);
      let stats = { mean: 0, std: 0, distinctColors: 0 };
      try {
        stats = imageStats(decodePng(png));
      } catch (e) {
        stats.error = String(e.message || e);
      }
      const blank = ready ? stats.std < 3 : true;

      const miss = ready
        ? [...missingLandmarks(scenario, audit.landmarks), ...(actionFailed ? ["actionFailed"] : [])]
        : ["render-failed"];
      const overflowFlag = audit.overflow > 2 ? 1 : 0;
      const tap = scenario.checkTap ? audit.tap : 0;
      const score =
        WEIGHTS.contrast * audit.contrast +
        WEIGHTS.svgContrast * audit.svgContrast +
        WEIGHTS.overflow * overflowFlag +
        WEIGHTS.offscreen * audit.offscreen +
        WEIGHTS.clipped * audit.clipped +
        WEIGHTS.tap * tap +
        WEIGHTS.palette * audit.palette +
        WEIGHTS.blank * (blank ? 1 : 0) +
        WEIGHTS.landmark * miss.length;

      results.push({
        scenario: scenario.name,
        viewport: `${scenario.width}x${scenario.height}`,
        ready,
        score,
        overflowFlag,
        blank,
        missingLandmarks: miss,
        metrics: {
          contrast: audit.contrast,
          svgContrast: audit.svgContrast,
          offscreen: audit.offscreen,
          clipped: audit.clipped,
          tap,
          palette: audit.palette,
          overflowPx: audit.overflow,
          textElements: audit.textElements,
        },
        landmarks: audit.landmarks,
        imageStats: stats,
        screenshot: `harness/out/screenshots/${scenario.name}.png`,
        detail: audit.detail,
      });
    }

    const agg = (key) => results.reduce((s, r) => s + (r.metrics[key] || 0), 0);
    const total = results.reduce((s, r) => s + r.score, 0);
    const secondary = {
      contrast_violations: agg("contrast"),
      svg_contrast: agg("svgContrast"),
      offscreen_elements: agg("offscreen"),
      clipped_text: agg("clipped"),
      tiny_tap_targets: agg("tap"),
      palette_violations: agg("palette"),
      overflow_scenarios: results.reduce((s, r) => s + r.overflowFlag, 0),
      blank_screens: results.reduce((s, r) => s + (r.blank ? 1 : 0), 0),
      missing_landmarks: results.reduce((s, r) => s + r.missingLandmarks.length, 0),
      scenarios_ready: results.reduce((s, r) => s + (r.ready ? 1 : 0), 0),
      scenarios_total: results.length,
    };

    writeFileSync(
      join(OUT, "report.json"),
      JSON.stringify({ generated: new Date().toISOString(), weights: WEIGHTS, total, secondary, scenarios: results }, null, 2),
    );

    console.log(`METRIC ux_defect_score=${total}`);
    for (const [k, v] of Object.entries(secondary)) console.log(`METRIC ${k}=${v}`);

    if (secondary.scenarios_ready === 0) {
      console.error("harness: no scenario rendered; check fixtures and chrome");
      return 1;
    }
    return 0;
  } finally {
    try {
      if (client) client.close();
    } catch {
      /* ignore */
    }
    chrome.close();
    await replay.close();
    await demoReplay.close();
  }
}

main()
  .then((code) => process.exit(code))
  .catch((err) => {
    console.error("harness failed:", err?.stack || err);
    process.exit(1);
  });
