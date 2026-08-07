/**
 * Cairn CLI adapter for the guided-console prototype.
 *
 * Every fact this console puts on screen comes from a real `cairn`
 * invocation against the target project: node states from `cairn context`,
 * wave order from `cairn frontier` tiers, layer glosses from `cairn get`.
 * Nothing here is synthesised, and no graph fact is computed twice.
 */
import { execFile } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

const CAIRN_BIN = process.env.CAIRN_PROTOTYPE_CAIRN_BIN || "cairn";

/**
 * Runs a cairn subcommand in the target project and parses its JSON.
 *
 * Bounded, because these run on the frame-building path while the harness is
 * working the same directory. An unbounded call that never returns would stall
 * every later frame behind it.
 */
async function cairnJson(projectDir, args) {
  const { stdout } = await execFileAsync(CAIRN_BIN, [...args, "--json"], {
    cwd: projectDir,
    maxBuffer: 64 * 1024 * 1024,
    timeout: 20_000,
  });
  return JSON.parse(stdout);
}

/**
 * How many decisions the project carries, read through cairn rather than by
 * counting files, so it is the same number cairn itself would report.
 */
export async function decisionCount(projectDir) {
  if (!hasBlueprint(projectDir)) return 0;
  const ctx = await cairnJson(projectDir, ["context"]);
  return ctx.artefact_counts?.decisions ?? 0;
}

/** True once the decode step has written a blueprint to the target project. */
export function hasBlueprint(projectDir) {
  return existsSync(join(projectDir, "cairn.blueprint"));
}

/** Reports the cairn binary this prototype drives, for the working drawer. */
export async function cairnVersion() {
  const { stdout } = await execFileAsync(CAIRN_BIN, ["--version"]);
  return stdout.trim();
}

const plural = (n, one, many) => (n === 1 ? one : many);

/**
 * Blueprint node names are single tokens (`SumStore`), because the grammar
 * takes no spaces there. The console reads in plain language, so the display
 * name is derived: split the words, sentence-case them, and leave acronyms
 * whole. Display only. The id and the declared name are untouched.
 */
function plainName(name) {
  const spaced = name
    .replace(/[-_]+/g, " ")
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1 $2");
  const words = spaced.split(/\s+/).filter(Boolean);
  return words.map((word, index) => (index === 0 || word === word.toUpperCase() ? word : word.toLowerCase())).join(" ");
}

/** Joins plain names the way a sentence would: "a, b and c". */
function sentenceList(names) {
  if (names.length <= 1) return names[0] ?? "";
  return `${names.slice(0, -1).join(", ")} and ${names[names.length - 1]}`;
}

/**
 * A band gloss is a label, not a sentence. The register reference sets it at a
 * handful of words beside the layer name ("Interface · what you touch"), while
 * a blueprint description is a full sentence written for a reader who has
 * stopped to read it. So take the leading clause, and keep it only while it
 * still reads as a label: past that the layer name stands alone, because a
 * sentence cut mid-phrase reads worse than no gloss at all. Nothing is lost,
 * the description stays where it was declared.
 */
const GLOSS_WORDS = 8;

function glossOf(description) {
  if (!description) return null;
  const lead = description.split(/[:;(]/)[0].trim().replace(/[.,]+$/, "");
  if (lead === "") return null;
  return lead.split(/\s+/).length > GLOSS_WORDS ? null : lead;
}

/**
 * Writes the wave note in plain language, derived from the graph rather
 * than authored: wave 1 has nothing under it, later waves name what they
 * are waiting on.
 */
function waveNote(waveNumber, parts, nameOf) {
  if (waveNumber === 1) {
    return parts.length === 1 ? "Nothing rests under it, so it goes first." : "Nothing rests under them, so they go first, side by side.";
  }
  const blockers = [...new Set(parts.flatMap((part) => part.blocking))]
    .map((id) => nameOf(id))
    .filter(Boolean)
    .sort();
  // A later tier with nothing blocking it has already had its dependencies
  // built, so cairn calls it startable now. Saying it waits on the wave before
  // would invent a dependency the graph does not hold.
  if (blockers.length === 0) return parts.length === 1 ? "Nothing is holding it up." : "Nothing is holding these up.";
  return `${plural(parts.length, "Waits", "Wait")} for ${sentenceList(blockers)}.`;
}

/**
 * Assembles the one view the console renders: layers top to bottom, parts
 * with their wave number and build state, and the waves as sentences.
 *
 * Layer order is derived, not declared: the layer whose parts are built
 * last sits at the top, which is how a reader expects to see it.
 */
export async function composeMap(projectDir) {
  if (!hasBlueprint(projectDir)) return null;

  const [ctx, front] = await Promise.all([cairnJson(projectDir, ["context"]), cairnJson(projectDir, ["frontier"])]);

  const nodes = new Map(ctx.nodes.map((n) => [n.id, n]));
  const nameOf = (id) => (nodes.has(id) ? plainName(nodes.get(id).name) : id);

  const tiers = new Map();
  for (const entry of [...front.ready, ...front.blocked]) {
    tiers.set(entry.node, { wave: entry.tier + 1, blocking: entry.blocking ?? [] });
  }

  const system = ctx.nodes.find((n) => n.kind === "system");
  const containers = ctx.nodes.filter((n) => n.kind === "container");

  // Only containers need a description: the console shows layer glosses, and
  // nothing reads the system's own.
  const descriptions = new Map(
    await Promise.all(
      containers.map(async (container) => {
        const detail = await cairnJson(projectDir, ["get", container.id]);
        return [container.id, detail.description ?? null];
      }),
    ),
  );

  const partOf = (id) => {
    const n = nodes.get(id);
    const tier = tiers.get(id);
    return {
      id,
      name: n ? plainName(n.name) : id,
      state: n?.state ?? "unknown",
      wave: tier?.wave ?? null,
      blocking: tier?.blocking ?? [],
    };
  };

  const claimed = new Set();
  const layers = containers.map((container) => {
    const parts = (container.children ?? [])
      .filter((id) => nodes.get(id)?.kind === "module")
      .map((id) => {
        claimed.add(id);
        return partOf(id);
      });
    return {
      id: container.id,
      name: plainName(container.name),
      gloss: glossOf(descriptions.get(container.id)),
      parts,
    };
  });

  // A module declared outside every container is shown, but it is not a layer:
  // cairn declared no container for it, so counting it as one would credit the
  // graph with structure it does not carry.
  const loose = ctx.nodes.filter((n) => n.kind === "module" && !claimed.has(n.id)).map((n) => partOf(n.id));
  const declaredLayers = layers.length;
  if (loose.length > 0) {
    layers.push({ name: "In no layer", gloss: null, parts: loose });
  }

  const depth = (layer) => Math.max(0, ...layer.parts.map((p) => p.wave ?? 0));
  layers.sort((a, b) => depth(b) - depth(a));

  const allParts = layers.flatMap((layer) => layer.parts);
  const waveNumbers = [...new Set(allParts.map((p) => p.wave).filter((w) => w !== null))].sort((a, b) => a - b);
  const waves = waveNumbers.map((n) => {
    const parts = allParts.filter((p) => p.wave === n);
    return {
      n,
      parts: parts.map((p) => p.name),
      note: waveNote(n, parts, nameOf),
    };
  });

  const ghost = allParts.filter((p) => p.state === "ghost").length;

  return {
    system: system ? { name: plainName(system.name) } : null,
    // Ids and blocker lists are consumed above, in the notes; the console reads
    // only what it renders.
    layers: layers.map((layer) => ({
      name: layer.name,
      gloss: layer.gloss,
      parts: layer.parts.map((part) => ({ name: part.name, state: part.state, wave: part.wave })),
    })),
    waves,
    counts: {
      parts: allParts.length,
      ghost,
      built: allParts.length - ghost,
      layers: declaredLayers,
      edges: ctx.edge_count ?? 0,
      decisions: ctx.artefact_counts?.decisions ?? 0,
    },
  };
}
