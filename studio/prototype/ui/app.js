/**
 * The guided console, client side.
 *
 * One render function over one server state. No framework, no local model of
 * the journey: the server holds it, the event stream pushes it, and this file
 * only decides how it reads. Every graph count came out of `cairn`; the
 * question and answer tallies come from the harness reply the server holds.
 *
 * The rule this file is written to: never put a claim on screen that the state
 * behind it does not support. Nothing is building here, so nothing says it is.
 */

const el = {
  stage: document.getElementById("stage"),
  talk: document.getElementById("talk"),
  outcome: document.getElementById("outcome"),
  composer: document.getElementById("composer"),
  say: document.getElementById("say"),
  reset: document.getElementById("reset"),
  chip: document.getElementById("project-chip"),
};

/**
 * The stage strip. Each stage reads one way while it is the current one and
 * another once it is behind you, so "questions answered" is never shown over
 * questions nobody has answered.
 */
const STAGES = [
  { key: "described", active: "describe", done: "described", reached: ["grilling", "questions", "settling", "ready"] },
  { key: "mapped", active: "mapping", done: "mapped", reached: ["questions", "settling", "ready"] },
  { key: "answered", active: "answer questions", done: "questions answered", reached: ["ready"] },
  { key: "run", active: "ready to run", done: "ready to run", reached: [] },
];

const NOW_STAGE = {
  empty: "described",
  describing: "described",
  grilling: "mapped",
  questions: "answered",
  settling: "answered",
  ready: "run",
};

const DOING_TITLE = {
  describing: "Reading what you wrote.",
  grilling: "Working out what to ask you.",
  settling: "Writing your answers in.",
};

let state = null;
let talkMark = "";

/* ---- tiny dom helpers -------------------------------------------------- */

function node(tag, className, text) {
  const n = document.createElement(tag);
  if (className) n.className = className;
  if (text !== undefined) n.textContent = text;
  return n;
}

function fill(parent, children) {
  parent.replaceChildren(...children.filter(Boolean));
}

const plural = (n, one, many) => `${n} ${n === 1 ? one : many}`;

function sentenceList(items) {
  if (items.length <= 1) return items[0] ?? "";
  return `${items.slice(0, -1).join(", ")} and ${items[items.length - 1]}`;
}

/** A closed technical disclosure. The primary layer never needs it opened. */
function disclosure(summaryText, body) {
  const wrap = node("details", "working");
  const head = node("summary", "working-head");
  head.append(node("span", "working-title", summaryText), node("span", "working-note", "not needed to steer"));
  wrap.append(head, body);
  return wrap;
}

/* ---- the bezel stage strip -------------------------------------------- */

function renderStage() {
  const now = NOW_STAGE[state.stage] ?? "described";
  const nowIndex = STAGES.findIndex((s) => s.key === now);
  const parts = [];
  STAGES.forEach((stage, index) => {
    if (index > 0) parts.push(node("span", "sep", "→"));
    const behind = stage.reached.includes(state.stage);
    if (index === nowIndex) parts.push(node("span", "now", stage.active));
    else parts.push(node("span", behind ? "" : "later", behind ? stage.done : stage.active));
  });
  fill(el.stage, parts);
}

/* ---- the conversation rail -------------------------------------------- */

function grillBlock() {
  const block = node("div", "grill");
  block.setAttribute("aria-label", "Open questions");

  const head = node("div", "grill-head");
  head.append(node("span", "grill-title", `Before we build · ${plural(state.questions.length, "question", "questions")}`), node("span", "grill-meta", `${state.answered} of ${state.questions.length} answered`));
  block.append(head);

  for (const question of state.questions) {
    const wrap = node("div", `grill-q${question.loadBearing ? " load-bearing" : ""}`);
    wrap.append(node("p", "q", question.question));

    const answers = node("div", "answers");
    for (const option of question.options) {
      const chosen = state.answers[question.id] === option.id;
      const button = node("button", `answer${chosen ? " chosen" : ""}`, option.label);
      button.type = "button";
      button.disabled = Boolean(state.busy) || state.stage === "ready";
      button.addEventListener("click", () => {
        post("/api/answer", { question: question.id, option: option.id });
      });
      answers.append(button);
    }
    wrap.append(answers);
    if (question.why) wrap.append(node("p", "grill-why", question.why));
    block.append(wrap);
  }
  return block;
}

function renderTalk() {
  const lines = [];

  if (state.description) {
    lines.push(node("div", "speaker", "You"), node("p", null, state.description));
  }

  if (state.summary) {
    lines.push(node("div", "speaker", "Cairn"), node("p", "by-cairn", state.summary));
    // The pointer, and only the pointer. The shape of the map is the
    // standfirst's line and how much of it stands is the map's own title, so
    // repeating either here put one number on screen three times over.
    if (state.map) {
      const quiet = node("p");
      quiet.append(node("span", "quiet", "The map on the right filled in as this was written."));
      lines.push(quiet);
    }
  }

  if (state.questions.length > 0) lines.push(grillBlock());

  // The harness writes settleNote to say the answers went in. When the
  // measured decision count disagrees, that sentence is not trustworthy, so
  // the warning in the outcome speaks instead of it.
  if (state.settleNote && !state.settleWarning) {
    lines.push(node("div", "speaker", "Cairn"), node("p", "by-cairn", state.settleNote));
  }

  if (lines.length === 0) {
    lines.push(node("div", "speaker", "Cairn"), node("p", "by-cairn", "Tell me what you want to build. I will lay it out, then ask you about anything your words leave open."));
  }

  const wasAt = el.talk.scrollTop;
  fill(el.talk, lines);

  // Re-rendering resets the scroll, so only move the reader when the
  // conversation actually gained something. Questions arriving means showing
  // them from the top, head and all; anything else means the newest line.
  const mark = `${state.questions.length}|${state.settleNote ? 1 : 0}|${state.summary ? 1 : 0}`;
  if (mark === talkMark) {
    el.talk.scrollTop = wasAt;
    return;
  }
  talkMark = mark;
  const grill = el.talk.querySelector(".grill");
  if (grill && !state.settleNote) grill.scrollIntoView({ block: "start" });
  else el.talk.scrollTop = el.talk.scrollHeight;
}

/* ---- the outcome view -------------------------------------------------- */

function runPlate() {
  const button = node("button", "run");
  button.type = "button";

  const left = node("div");
  left.append(node("span", "run-word", "Run"));

  const right = node("div");
  const unbuilt = state.map?.counts.ghost ?? 0;
  const waves = state.map?.waves.length ?? 0;
  // Only what the graph proves. Not "each wave waits for the one before it":
  // a wave whose dependencies are already built waits for nothing, and a
  // part-built map can open at wave 2 with no wave 1 in it at all.
  right.append(node("span", "run-sub", `Build the ${plural(unbuilt, "part", "parts")} still to do, in ${plural(waves, "wave", "waves")}. The order comes from what depends on what.`), node("div", "run-caveat", state.run.caveat));

  button.append(left, right);
  button.addEventListener("click", async () => {
    const res = await fetch("/api/run", { method: "POST" });
    const body = await res.json();
    showTrouble("Run is not wired.", body.reason ?? body.error);
  });
  return button;
}

function wavesPanel() {
  const panel = node("div", "waves");
  panel.append(node("div", "waves-title", "What will happen, in order"));
  // A wave number is a position in the whole build, not a count of what is
  // left. In a part-built map the earliest positions can already be built, so
  // the first row is numbered 2 or later. Say why, rather than leave a reader
  // hunting for a wave 1 that is not there. No wave 1 row means no unbuilt
  // part sits at that depth, which is only true when something built does.
  const first = state.map.waves[0]?.n ?? 1;
  if (first > 1) {
    panel.append(node("div", "waves-lead", `It starts at wave ${first}, because what would have come first is already built.`));
  }
  for (const wave of state.map.waves) {
    const row = node("div", "wave-row");
    row.append(node("span", "wave-n", String(wave.n)));
    const what = node("div", "wave-what");
    what.append(node("div", "wave-parts", sentenceList(wave.parts)), node("div", "wave-note", wave.note));
    row.append(what);
    panel.append(row);
  }
  return panel;
}

function miniMap() {
  const panel = node("div", "mini-map");
  panel.setAttribute("aria-label", "The map so far");
  // How much of the map stands is this panel's own fact, so it is stated here
  // rather than a third time in the conversation.
  const built = state.map.counts.built;
  const standing = built === 0 ? "all still to build" : `${built} of ${state.map.counts.parts} built`;
  panel.append(node("div", "waves-title", `The map so far · ${standing}`));

  for (const layer of state.map.layers) {
    const band = node("div", "mm-band");
    band.append(node("span", "mm-gloss", layer.gloss ? `${layer.name} · ${layer.gloss}` : layer.name));
    const row = node("div", "mm-row");
    for (const part of layer.parts) {
      const chip = node("span", `mm-node${part.state === "ghost" ? "" : " built"}`);
      if (part.wave !== null) chip.append(node("i", null, String(part.wave)));
      chip.append(document.createTextNode(part.name));
      row.append(chip);
    }
    band.append(row);
    panel.append(band);
  }
  // The legend explains every treatment on screen, so it only claims the ones
  // that are there: a solid chip exists once something is built.
  const builtClause = state.map.counts.built > 0 ? " · solid means built" : "";
  panel.append(node("div", "mm-foot", `numbers are waves · dashed means not built yet${builtClause} · nothing is building`));
  return panel;
}

const GLYPH = {
  map: '<rect x="2" y="2" width="10" height="2.6" rx="1" fill="none" stroke="var(--ink-3)" /><rect x="2" y="5.8" width="10" height="2.6" rx="1" fill="none" stroke="var(--ink-3)" /><rect x="2" y="9.6" width="10" height="2.6" rx="1" fill="none" stroke="var(--ink-3)" />',
  decisions: '<circle cx="5" cy="7" r="3" fill="var(--ink-3)" /><circle cx="10" cy="7" r="3" fill="none" stroke="var(--ink-3)" />',
  parallel: '<path d="M2 4.5h8M2 9.5h8" stroke="var(--ink-3)" /><path d="M9 2.5l3 2-3 2M9 7.5l3 2-3 2" fill="none" stroke="var(--ink-3)" />',
  pending: '<circle cx="7" cy="7" r="4.5" fill="none" stroke="var(--ink-3)" stroke-dasharray="2 2" />',
  // The host this runs on. Its own glyph, because the map glyph means the map:
  // one glyph, one meaning, or the strip stops being readable at a glance.
  wiring: '<rect x="1.5" y="3" width="11" height="8" rx="1" fill="none" stroke="var(--ink-3)" /><path d="M4 6h6M4 8.5h3.5" stroke="var(--ink-3)" />',
};

function workingItem(glyph, textNodes) {
  const item = node("span", "w-item");
  const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  svg.setAttribute("width", "14");
  svg.setAttribute("height", "14");
  svg.setAttribute("viewBox", "0 0 14 14");
  svg.setAttribute("aria-hidden", "true");
  svg.innerHTML = glyph;
  item.append(svg, ...textNodes);
  return item;
}

function workingDrawer() {
  const strip = node("div", "working-strip");
  const map = state.map;
  if (map) {
    strip.append(
      workingItem(GLYPH.map, [node("span", "mono", "cairn.blueprint"), document.createTextNode(` ${plural(map.counts.parts, "part", "parts")}, ${plural(map.counts.edges, "edge", "edges")}`)]),
      workingItem(GLYPH.parallel, [document.createTextNode(`${plural(map.counts.layers, "layer", "layers")}, ${plural(map.waves.length, "wave", "waves")} by dependency depth`)]),
    );
    // The count comes from cairn, and it counts decision files, so it is
    // labelled as what cairn counted rather than as rulings.
    if (map.counts.decisions > 0) {
      strip.append(workingItem(GLYPH.decisions, [document.createTextNode(`${plural(map.counts.decisions, "decision", "decisions")} recorded`)]));
    }
  }
  strip.append(workingItem(GLYPH.pending, [node("span", "mono", "cairn ruling run"), document.createTextNode(" sanctioned, not built yet")]));
  // Harness and project are one fact, where this is running, so they are one
  // item. The register reference strips this drawer to four.
  strip.append(workingItem(GLYPH.wiring, [node("span", "mono", state.wiring.harness), document.createTextNode(" · "), node("span", "mono", state.wiring.project)]));
  return disclosure("The working", strip);
}

function doingPanel() {
  const panel = node("div", "doing");
  panel.append(node("div", "doing-title", DOING_TITLE[state.stage] ?? "Working."));
  const list = node("ul", "doing-lines");
  for (const line of state.activity) list.append(node("li", null, line));
  if (state.activity.length === 0) list.append(node("li", null, "starting up"));
  panel.append(list, node("div", "doing-foot", "This takes a few minutes. You can close the tab; this keeps running until it finishes."));
  return panel;
}

/**
 * Trouble: a plain title in the primary layer, the raw detail behind a
 * disclosure. Harness output, file paths, and command names are the working,
 * not the message.
 */
function troublePanel(title, detail) {
  const panel = node("div", "trouble");
  panel.append(node("div", "trouble-title", title));
  if (detail) {
    const inner = node("details", "trouble-more");
    inner.append(node("summary", null, "what exactly went wrong"), node("p", "detail", detail));
    panel.append(inner);
  }
  return panel;
}

function showTrouble(title, detail) {
  el.outcome.prepend(troublePanel(title, detail));
}

function headlineFor() {
  const name = state.map?.system?.name;
  switch (state.stage) {
    case "empty":
      return "Describe what you want to build.";
    case "describing":
      return "Laying out what you asked for.";
    case "grilling":
      return name ? `Mapped your ${name}.` : "Mapped what you asked for.";
    case "questions": {
      const n = state.questions.length;
      return `${plural(n, "call", "calls")} ${n === 1 ? "is" : "are"} yours to make.`;
    }
    case "settling":
      return "Writing your answers into the plan.";
    case "ready":
      return name ? `Ready to build your ${name}.` : "Ready to build.";
    default:
      return "Build something.";
  }
}

function standfirstFor() {
  const map = state.map;
  if (!map || state.stage === "empty") return null;
  // A blueprint need not declare containers. "in 0 layers" says nothing, so
  // the clause is only there when the graph has layers to report.
  const layers = map.counts.layers > 0 ? ` in ${plural(map.counts.layers, "layer", "layers")}` : "";
  const shape = `${plural(map.counts.parts, "part", "parts")}${layers}, across ${plural(map.waves.length, "wave", "waves")}`;
  if (state.stage === "ready") {
    // No promise of work in the background: run refuses, so nothing is going.
    return `Every question is answered. ${shape}. Nothing is being built yet, because run is not wired.`;
  }
  if (state.stage === "questions") {
    return `${shape}. Answer what is below and the plan is settled.`;
  }
  return `${shape}. Nothing is built yet.`;
}

function renderOutcome() {
  const blocks = [node("h1", "headline", headlineFor())];

  const standfirst = standfirstFor();
  if (standfirst) blocks.push(node("p", "standfirst", standfirst));

  if (state.stage === "empty" && !state.busy) {
    blocks.push(node("p", "blank-note", "Write it on the left in your own words: what it does, and what you want to be able to do with it. You will get a plan you can read, a few questions, and then one action."));
  }

  if (state.error) blocks.push(troublePanel("That did not work.", state.error));

  // More or fewer calls recorded than the person made. Say so where they will
  // see it, above the plan those calls now govern.
  if (state.settleWarning) {
    blocks.push(troublePanel(state.settleWarning.title, state.settleWarning.detail));
  }

  if (state.busy) blocks.push(doingPanel());

  if (state.stage === "ready" && !state.busy) blocks.push(runPlate());

  if (state.map && state.stage !== "empty") {
    const grid = node("div", "plan-grid");
    if (state.map.waves.length > 0) grid.append(wavesPanel());
    grid.append(miniMap());
    blocks.push(grid);
  }

  if (state.mapError) blocks.push(troublePanel("Could not read the map.", state.mapError));

  blocks.push(workingDrawer());
  fill(el.outcome, blocks);
}

function render() {
  renderStage();
  renderTalk();
  renderOutcome();
  el.chip.textContent = state.map?.system?.name ? `prototype · ${state.map.system.name}` : "prototype";
  el.say.disabled = Boolean(state.busy);
  el.reset.disabled = Boolean(state.busy);
  // The composer only ever starts a fresh description. Say that, rather than
  // offering a conversation this prototype does not hold.
  el.say.placeholder = state.description ? "Describe it again to start over." : "Describe what you want built, in your own words.";
}

/* ---- talking to the server -------------------------------------------- */

async function post(path, body) {
  const res = await fetch(path, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body ?? {}),
  });
  if (!res.ok) {
    const problem = await res.json().catch(() => ({}));
    showTrouble("That did not work.", problem.error ?? problem.reason ?? `${res.status}`);
  }
  return res;
}

el.composer.addEventListener("submit", (event) => {
  event.preventDefault();
  const description = el.say.value.trim();
  if (description === "") return;
  el.say.value = "";
  post("/api/describe", { description });
});

el.say.addEventListener("keydown", (event) => {
  if (event.key === "Enter" && !event.shiftKey) {
    event.preventDefault();
    el.composer.requestSubmit();
  }
});

el.reset.addEventListener("click", () => {
  post("/api/reset");
});

/**
 * A step can run for minutes, and anything between here and the service may
 * drop an idle stream in that time. Always come back for it: the first frame
 * of a new connection is the whole state, so a reconnect is also a catch-up.
 */
function connect() {
  const stream = new EventSource("/api/events");
  stream.addEventListener("message", (event) => {
    state = JSON.parse(event.data);
    render();
  });
  stream.addEventListener("error", () => {
    stream.close();
    setTimeout(connect, 2000);
  });
}

connect();
