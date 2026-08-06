/**
 * The creation journey as a state machine.
 *
 * Four moments, and nothing else: describe, the map forms, the grill drains
 * the doubts, run. State lives in one JSON file beside the target project, so
 * closing the browser loses nothing: the service and its harness keep going,
 * and the next connection is handed the whole state. A service restart is
 * narrower. A finished journey is read back, but a step that was in flight
 * dies with the process and has to be asked for again.
 */
import { EventEmitter } from "node:events";
import { existsSync } from "node:fs";
import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { composeMap, decisionCount, hasBlueprint } from "./cairn.mjs";
import { extractJson, runHarness } from "./harness.mjs";
import { describePrompt, grillPrompt, settlePrompt } from "./prompts.mjs";

/**
 * Why run is not wired.
 *
 * `dec.webui-write-authority` clause 4 rules run in as a recorded ruling the
 * driver obeys (`cairn ruling run <plan>`), but that record is proposed, not
 * signed. Until the maintainer signs it the verb does not exist, so this
 * console records no ruling and dispatches nothing. Wiring it is not this
 * constant: the route, the click handler, and a dispatch path that does not
 * exist yet have to move together, after the verb ships.
 */
const RUN_NOT_WIRED = {
  caveat: "not wired yet · run needs a ruling this console is not yet allowed to record",
  reason: "Run would record a ruling (`cairn ruling run`) that the driver then obeys. That verb is proposed in `dec.webui-write-authority` clause 4 and is not signed yet, so nothing is dispatched and no ruling is written.",
};

const MAX_ACTIVITY = 8;

const empty = () => ({
  stage: "empty",
  description: null,
  summary: null,
  questions: [],
  answers: {},
  settleNote: null,
  settleWarning: null,
  activity: [],
  busy: null,
  error: null,
});

export class Journey extends EventEmitter {
  constructor(dataDir) {
    super();
    this.projectDir = join(dataDir, "project");
    this.statePath = join(dataDir, "journey.json");
    this.state = empty();
    this.inFlight = null;
  }

  async open() {
    await mkdir(this.projectDir, { recursive: true });
    if (existsSync(this.statePath)) {
      try {
        this.state = { ...empty(), ...JSON.parse(await readFile(this.statePath, "utf8")), busy: null, error: null };
      } catch {
        this.state = empty();
      }
    }
    return this;
  }

  async persist() {
    const { activity: _activity, busy: _busy, error: _error, ...durable } = this.state;
    await writeFile(this.statePath, `${JSON.stringify(durable, null, 2)}\n`, "utf8");
  }

  /** Pushes a plain-language line and tells every attached browser. */
  note(line) {
    this.state.activity = [...this.state.activity, line].slice(-MAX_ACTIVITY);
    this.emit("change");
  }

  set(patch) {
    this.state = { ...this.state, ...patch };
    this.emit("change");
  }

  /** State plus the map, which is read fresh from cairn on every request. */
  async snapshot() {
    let map = null;
    let mapError = null;
    try {
      map = await composeMap(this.projectDir);
    } catch (cause) {
      mapError = cause.message;
    }
    const answeredCount = this.state.questions.filter((q) => this.state.answers[q.id]).length;
    return {
      ...this.state,
      map,
      mapError,
      answered: answeredCount,
      run: RUN_NOT_WIRED,
    };
  }

  /** Serialises the harness steps: one journey, one thing happening at a time. */
  async step(name, work) {
    if (this.inFlight) throw Object.assign(new Error(`Already ${this.state.busy}.`), { status: 409 });
    const stageBefore = this.state.stage;
    this.set({ busy: name, error: null, activity: [] });
    this.inFlight = (async () => {
      try {
        await work();
      } catch (cause) {
        // A failed step must not park the journey on a stage it never reached,
        // or the console goes on describing work that did not happen.
        this.set({ stage: stageBefore, error: cause.message });
      } finally {
        this.set({ busy: null });
        await this.persist();
      }
    })();
    this.inFlight.finally(() => {
      this.inFlight = null;
    });
    return this.inFlight;
  }

  async blueprintText() {
    return readFile(join(this.projectDir, "cairn.blueprint"), "utf8");
  }

  /** Moment 1 and 2: the words become the blueprint, and the map forms. */
  describe(description) {
    return this.step("describing", async () => {
      this.set({
        stage: "describing",
        description,
        summary: null,
        questions: [],
        answers: {},
        settleNote: null,
        settleWarning: null,
      });
      const reply = await runHarness({
        projectDir: this.projectDir,
        prompt: describePrompt(description),
        step: "Reading your description",
        onActivity: (line) => this.note(line),
      });
      const { summary } = extractJson(reply, "Reading your description");
      if (!hasBlueprint(this.projectDir)) {
        throw new Error("The harness finished without writing a map. Nothing was changed.");
      }
      this.set({ summary: summary ?? null });
      await this.grillNow();
    });
  }

  /** Moment 3, first half: the doubts surface as plain questions. */
  async grillNow() {
    this.set({ stage: "grilling" });
    const reply = await runHarness({
      projectDir: this.projectDir,
      prompt: grillPrompt(this.state.description, await this.blueprintText()),
      step: "Working out what to ask you",
      onActivity: (line) => this.note(line),
    });
    const { questions } = extractJson(reply, "Working out what to ask you");
    // The harness fills this in, so the shape is checked here rather than
    // trusted: a question with no answers to pick would render as a dead end.
    const usable = Array.isArray(questions) && questions.every((q) => typeof q?.id === "string" && typeof q?.question === "string" && Array.isArray(q?.options) && q.options.length >= 2 && q.options.every((o) => typeof o?.id === "string" && typeof o?.label === "string"));
    if (!usable || questions.length === 0) {
      throw new Error("The grill came back with nothing it could put to you. Nothing was changed.");
    }

    // The console prefixes the marked question with "the one call worth arguing
    // with", so exactly one may carry the mark. Rather than fail an otherwise
    // good grill over a flag, keep the first and clear the rest: the definite
    // claim then matches what is on screen.
    let marked = false;
    const normalised = questions.map((question) => {
      const loadBearing = question.loadBearing === true && !marked;
      if (loadBearing) marked = true;
      return { ...question, loadBearing };
    });
    this.set({ stage: "questions", questions: normalised, answers: {} });
  }

  /** Records one selected answer, and settles as soon as the last one lands. */
  async answer(questionId, optionId) {
    const question = this.state.questions.find((q) => q.id === questionId);
    if (!question) throw Object.assign(new Error(`No question ${questionId}.`), { status: 404 });
    if (!question.options.some((o) => o.id === optionId)) {
      throw Object.assign(new Error(`No answer ${optionId} for ${questionId}.`), { status: 404 });
    }
    this.set({ answers: { ...this.state.answers, [questionId]: optionId } });
    await this.persist();

    const outstanding = this.state.questions.filter((q) => !this.state.answers[q.id]);
    if (outstanding.length === 0 && this.state.stage === "questions") {
      // Started, not awaited: settling takes minutes, and this request only
      // acknowledges the answer. Progress and failure both arrive by stream.
      this.settle().catch((cause) => this.set({ error: cause.message }));
    }
  }

  /** Moment 3, second half: the answers become recorded decisions. */
  settle() {
    if (this.state.questions.length === 0 || !hasBlueprint(this.projectDir)) {
      throw Object.assign(new Error("There is nothing to settle yet."), { status: 400 });
    }
    return this.step("settling", async () => {
      this.set({ stage: "settling", settleWarning: null });
      const answered = this.state.questions.map((question) => ({
        question: question.question,
        answer: question.options.find((o) => o.id === this.state.answers[question.id])?.label ?? "",
        node: question.node,
      }));
      const before = await decisionCount(this.projectDir);
      const reply = await runHarness({
        projectDir: this.projectDir,
        prompt: settlePrompt(this.state.description, await this.blueprintText(), answered),
        step: "Writing your answers in",
        onActivity: (line) => this.note(line),
      });
      const { note } = extractJson(reply, "Writing your answers in");

      // A decision carries authority over the build, so one recorded in the
      // person's name that they never made is a fault, not a bonus, and one
      // missing means a call they did make is not written down. The prompt asks
      // for exactly one per answer; this measures, because a prompt is a
      // request and not a guarantee. It reports rather than deletes: removing
      // an artefact the person has not seen would trade one silent act for
      // another.
      const written = (await decisionCount(this.projectDir)) - before;
      const expected = answered.length;
      let settleWarning = null;
      if (written > expected) {
        settleWarning = {
          title: "More was decided than you decided.",
          detail: `You answered ${expected} ${expected === 1 ? "question" : "questions"}, and ${written} calls were written down. Whatever went beyond your answers was not yours to make. All of them are in meta/decisions/ and worth reading before you build.`,
        };
      } else if (written < expected) {
        settleWarning = {
          title: "Not everything you decided was written down.",
          detail: `You answered ${expected} ${expected === 1 ? "question" : "questions"}, but only ${written} ${written === 1 ? "call was" : "calls were"} written down. Check meta/decisions/ before you build: at least one of your answers is not recorded anywhere.`,
        };
      }
      this.set({ stage: "ready", settleNote: note ?? null, settleWarning });
    });
  }

  /** Clears the target project and the journey, for a fresh run through. */
  async reset() {
    if (this.inFlight) throw Object.assign(new Error(`Already ${this.state.busy}.`), { status: 409 });
    await rm(this.projectDir, { recursive: true, force: true });
    await rm(this.statePath, { force: true });
    await mkdir(this.projectDir, { recursive: true });
    this.state = empty();
    this.emit("change");
  }
}
