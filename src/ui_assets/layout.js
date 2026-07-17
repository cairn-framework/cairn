/* Layout: hinge placement of System -> Container -> Module on the graph
 * canvas. Consumes artefact counts to derive the provenance/authority
 * splay used by graph-canvas.js.
 */
import { balanceFromCount } from "./utils.js";

// ==========================================================================
// Layout: hinge placement of System -> Container -> Module
// ==========================================================================

const LAYOUT = Object.freeze({
  originX: 900,
  startY: 160,
  groupGap: 72,
  moduleGap: 30,
  moduleWidth: 250,
  moduleHeight: 108,
  containerWidth: 280,
  containerHeight: 74,
  systemWidth: 260,
  systemHeight: 66,
  splayMax: 220,
});

function buildLayout(graph, artefactCounts) {
  if (!graph || !Array.isArray(graph.nodes)) return { nodes: [], totalHeight: 0 };
  const systems = graph.nodes.filter((n) => n.kind === "system");
  const containers = graph.nodes.filter((n) => n.kind === "container");
  const modules = graph.nodes.filter((n) => n.kind === "module");
  const actors = graph.nodes.filter((n) => n.kind === "actor");
  const system = systems[0] || null;

  const laid = [];
  const { originX, startY, moduleGap, groupGap } = LAYOUT;
  let y = startY;

  if (system) {
    laid.push({
      id: system.id,
      kind: "system",
      data: system,
      x: originX,
      y: 60,
      width: LAYOUT.systemWidth,
      height: LAYOUT.systemHeight,
    });
  }

  const placeModulesFor = (containerId) => {
    const children = modules
      .filter((m) => m.parent === containerId)
      .slice()
      .sort((a, b) => {
        const rank = (state) => (state === "ghost" ? 0 : state === "orphaned" ? 1 : 2);
        const rr = rank(a.state) - rank(b.state);
        if (rr !== 0) return rr;
        return (a.name || a.id).localeCompare(b.name || b.id);
      });
    for (const m of children) {
      const counts = artefactCounts.get(m.id) || null;
      const prov = counts ? balanceFromCount(counts.provenance) : 0;
      const auth = counts ? balanceFromCount(counts.authority) : 0;
      const balance = (auth - prov) / 5;
      const x = originX + balance * LAYOUT.splayMax;
      laid.push({
        id: m.id,
        kind: "module",
        data: m,
        counts,
        x,
        y,
        width: LAYOUT.moduleWidth,
        height: LAYOUT.moduleHeight,
      });
      y += LAYOUT.moduleHeight + moduleGap;
    }
  };

  const topContainers = containers.filter((c) => c.parent === (system ? system.id : null) || !c.parent);
  const orphanedContainers = containers.filter((c) => !topContainers.includes(c));

  for (const container of [...topContainers, ...orphanedContainers]) {
    laid.push({
      id: container.id,
      kind: "container",
      data: container,
      x: originX,
      y,
      width: LAYOUT.containerWidth,
      height: LAYOUT.containerHeight,
    });
    y += LAYOUT.containerHeight + 28;
    placeModulesFor(container.id);
    y += groupGap;
  }

  const placedIds = new Set(laid.map((n) => n.id));
  const strayModules = modules.filter((m) => !placedIds.has(m.id));
  if (strayModules.length > 0) {
    laid.push({
      id: "__stray__",
      kind: "divider",
      data: { name: "Uncontained" },
      x: originX,
      y,
      width: LAYOUT.containerWidth,
      height: LAYOUT.containerHeight,
    });
    y += LAYOUT.containerHeight + 28;
    for (const m of strayModules) {
      const counts = artefactCounts.get(m.id) || null;
      laid.push({
        id: m.id,
        kind: "module",
        data: m,
        counts,
        x: originX,
        y,
        width: LAYOUT.moduleWidth,
        height: LAYOUT.moduleHeight,
      });
      y += LAYOUT.moduleHeight + moduleGap;
    }
    y += groupGap;
  }

  for (const a of actors) {
    laid.push({
      id: a.id,
      kind: "actor",
      data: a,
      counts: null,
      x: originX,
      y,
      width: LAYOUT.moduleWidth,
      height: LAYOUT.moduleHeight,
    });
    y += LAYOUT.moduleHeight + moduleGap;
  }

  return { nodes: laid, totalHeight: y + 40 };
}

function ownershipPath(from, to) {
  const fx = from.x;
  const fy = from.y + from.height / 2;
  const tx = to.x;
  const ty = to.y - to.height / 2;
  const midY = fy + (ty - fy) * 0.55;
  return `M ${fx} ${fy} C ${fx} ${midY}, ${tx} ${midY}, ${tx} ${ty}`;
}
// Approximate midpoint of an ownership bezier curve for label placement.
function edgeMidpoint(from, to) {
  const fx = from.x;
  const fy = from.y + from.height / 2;
  const tx = to.x;
  const ty = to.y - to.height / 2;
  const midY = fy + (ty - fy) * 0.55;
  // Cubic bezier at t=0.5:
  // x = 0.5*fx + 0.5*tx
  // y = 0.125*fy + 0.75*midY + 0.125*ty
  return {
    x: (fx + tx) / 2,
    y: 0.125 * fy + 0.75 * midY + 0.125 * ty,
  };
}

export { LAYOUT, buildLayout, ownershipPath, edgeMidpoint };
