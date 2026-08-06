/**
 * Harness adapter for the guided-console prototype.
 *
 * The prototype does not embed a model. It drives the maintainer's own
 * harness as a child process in non-interactive mode, reads its JSONL
 * event stream, and turns that stream into plain-language activity the
 * console can show while work is in flight.
 *
 * Nothing is shared between steps except the files in the target project.
 * That is deliberate: the blueprint and the artefacts are the state, which
 * is the same claim cairn makes about a codebase.
 */
import { spawn } from "node:child_process";
import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

const DEFAULT_COMMAND = "omp";
const DEFAULT_ARGS = "-p --mode json --no-session --no-skills";

/** The harness wiring in force, surfaced in the working drawer. */
export function harnessSpec() {
  return {
    command: process.env.CAIRN_PROTOTYPE_HARNESS || DEFAULT_COMMAND,
    args: (process.env.CAIRN_PROTOTYPE_HARNESS_ARGS ?? DEFAULT_ARGS).split(/\s+/).filter(Boolean),
  };
}

/** Maps one harness event to a plain sentence, or null when it says nothing. */
function plainLine(event) {
  if (event.type === "tool_execution_start") {
    const command = event.args?.command ?? "";
    switch (event.toolName) {
      case "write":
      case "edit":
        return "writing the map";
      case "read":
      case "glob":
      case "grep":
        return "reading what is already there";
      case "bash":
        if (/cairn\s+(scan|lint)/.test(command)) return "checking the map holds";
        if (/cairn\s+decision/.test(command)) return "recording your answers";
        return "working";
      default:
        return "working";
    }
  }
  if (event.type === "message_update" && event.assistantMessageEvent?.type === "thinking_start") {
    return "thinking it through";
  }
  if (event.type === "turn_start") return "thinking it through";
  return null;
}

/** Pulls the final assistant text out of the event stream. */
function assistantText(event) {
  if (event.type !== "message_end" || event.message?.role !== "assistant") return null;
  const text = (event.message.content ?? [])
    .filter((part) => part.type === "text")
    .map((part) => part.text)
    .join("");
  return text.trim() === "" ? null : text;
}

/**
 * Extracts the JSON object a step's prompt contract asked for, tolerating a
 * code fence or a stray sentence around it. Throws with the raw reply when
 * there is no object to find, so the failure is visible instead of guessed at.
 */
export function extractJson(text, step) {
  const start = text.indexOf("{");
  const end = text.lastIndexOf("}");
  if (start === -1 || end <= start) {
    throw new Error(`${step}: the harness did not reply with JSON. It said: ${text.slice(0, 400)}`);
  }
  try {
    return JSON.parse(text.slice(start, end + 1));
  } catch (cause) {
    throw new Error(`${step}: the harness reply was not valid JSON (${cause.message}). It said: ${text.slice(0, 400)}`);
  }
}

/**
 * Runs one harness step against the target project.
 *
 * `onActivity` receives deduplicated plain-language lines as they happen.
 * Resolves with the harness's final reply text.
 */
export async function runHarness({ projectDir, prompt, step, onActivity }) {
  const { command, args } = harnessSpec();
  const scratch = await mkdtemp(join(tmpdir(), "cairn-console-"));
  const promptPath = join(scratch, "prompt.md");
  await writeFile(promptPath, prompt, "utf8");

  try {
    return await new Promise((resolve, reject) => {
      const child = spawn(command, [...args, "--cwd", projectDir, `@${promptPath}`], {
        cwd: projectDir,
        stdio: ["ignore", "pipe", "pipe"],
      });

      let pending = "";
      let stderr = "";
      let reply = null;
      let lastLine = null;

      child.stdout.setEncoding("utf8");
      child.stdout.on("data", (chunk) => {
        pending += chunk;
        const lines = pending.split("\n");
        pending = lines.pop() ?? "";
        for (const line of lines) {
          if (line.trim() === "") continue;
          let event;
          try {
            event = JSON.parse(line);
          } catch {
            continue;
          }
          const text = assistantText(event);
          if (text !== null) reply = text;
          const plain = plainLine(event);
          if (plain !== null && plain !== lastLine) {
            lastLine = plain;
            onActivity?.(plain);
          }
        }
      });

      child.stderr.setEncoding("utf8");
      child.stderr.on("data", (chunk) => {
        stderr = (stderr + chunk).slice(-4000);
      });

      child.on("error", (cause) => {
        reject(new Error(`${step}: could not start the harness (${command}): ${cause.message}`));
      });

      child.on("close", (code) => {
        if (code !== 0) {
          reject(new Error(`${step}: the harness exited with code ${code}. ${stderr.trim().slice(-600)}`));
          return;
        }
        if (reply === null) {
          reject(new Error(`${step}: the harness finished without a reply. ${stderr.trim().slice(-600)}`));
          return;
        }
        resolve(reply);
      });
    });
  } finally {
    await rm(scratch, { recursive: true, force: true });
  }
}
