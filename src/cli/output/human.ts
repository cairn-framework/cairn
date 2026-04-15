import type { EdgeWithNode, NeighbourhoodResult, OrderTier } from "../../query/index.js";
import type { NodeMetadata } from "../../graph/ontology.js";

export function renderNode(node: NodeMetadata): string {
  return [
    `${node.kind}: ${node.name}`,
    `ID: ${node.id}`,
    `Description: ${node.description}`,
    `Tags: ${node.tags.length ? node.tags.map((tag) => `@${tag}`).join(" ") : "none"}`,
    `Paths: ${node.paths.length ? node.paths.join(", ") : "none"}`,
    `State: ${node.state}`,
    renderArtefacts(node),
  ].join("\n");
}

export function renderNeighbourhood(result: NeighbourhoodResult): string {
  return [
    renderNode(result.node),
    "",
    "Inbound",
    renderEdges(result.inbound, "source"),
    "",
    "Outbound",
    renderEdges(result.outbound, "target"),
  ].join("\n");
}

export function renderNodeList(title: string, entries: EdgeWithNode[]): string {
  return [title, renderEdges(entries, "node")].join("\n");
}

export function renderOrder(tiers: OrderTier[]): string {
  return tiers
    .map((tier) => [`Tier ${tier.tier}`, ...tier.nodes.map((node) => `- ${node.id} (${node.name})`)].join("\n"))
    .join("\n\n");
}

function renderArtefacts(node: NodeMetadata): string {
  const entries = Object.entries(node.artefacts);
  if (entries.length === 0) return "Artefacts: none";
  return ["Artefacts:", ...entries.flatMap(([key, values]) => values.map((value) => `- ${key}: ${value}`))].join("\n");
}

function renderEdges(entries: EdgeWithNode[], label: "source" | "target" | "node"): string {
  if (entries.length === 0) return "- none";
  return entries
    .map((entry) => {
      const prefix = entry.depth === 1 ? "direct" : `depth ${entry.depth}`;
      return `- ${entry.node.id} (${entry.node.name}) [${prefix}] - ${entry.edge.description}`;
    })
    .join("\n");
}
