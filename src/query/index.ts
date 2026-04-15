import { StructuralError } from "../parser/errors.js";
import type { GraphEdge, NodeMetadata, Ontology } from "../graph/ontology.js";

export interface EdgeWithNode {
  edge: GraphEdge;
  node: NodeMetadata;
  depth: number;
}

export interface NeighbourhoodResult {
  node: NodeMetadata;
  inbound: EdgeWithNode[];
  outbound: EdgeWithNode[];
}

export interface OrderTier {
  tier: number;
  nodes: NodeMetadata[];
}

export function resolveNodeId(ontology: Ontology, input: string): string | undefined {
  return ontology.nodes.has(input) ? input : ontology.nameToId.get(input);
}

export function getNode(ontology: Ontology, input: string): NodeMetadata | undefined {
  const id = resolveNodeId(ontology, input);
  return id ? ontology.nodes.get(id) : undefined;
}

export function neighbourhood(ontology: Ontology, input: string): NeighbourhoodResult {
  const node = requireNode(ontology, input);
  return {
    node,
    inbound: (ontology.inbound.get(node.id) ?? []).map((edge) => ({
      edge,
      node: ontology.nodes.get(edge.source)!,
      depth: 1,
    })),
    outbound: (ontology.outbound.get(node.id) ?? []).map((edge) => ({
      edge,
      node: ontology.nodes.get(edge.target)!,
      depth: 1,
    })),
  };
}

export function dependents(ontology: Ontology, input: string, transitive = false): EdgeWithNode[] {
  return walkEdges(ontology, requireNode(ontology, input).id, "inbound", transitive);
}

export function depends(ontology: Ontology, input: string, transitive = false): EdgeWithNode[] {
  return walkEdges(ontology, requireNode(ontology, input).id, "outbound", transitive);
}

export function dependencyOrder(ontology: Ontology, options: { from?: string; scope?: string } = {}): OrderTier[] {
  let ids = [...ontology.nodes.keys()];
  if (options.scope) ids = ids.filter((id) => id.startsWith(options.scope!));
  if (options.from) {
    const from = requireNode(ontology, options.from).id;
    const ancestors = new Set([from, ...depends(ontology, from, true).map((entry) => entry.node.id)]);
    ids = ids.filter((id) => ancestors.has(id));
  }

  const inScope = new Set(ids);
  const remaining = new Set(ids);
  const completed = new Set<string>();
  const tiers: OrderTier[] = [];

  while (remaining.size > 0) {
    const ready = [...remaining].filter((id) => (ontology.outbound.get(id) ?? []).every((edge) => !inScope.has(edge.target) || completed.has(edge.target)));
    if (ready.length === 0) {
      throw new StructuralError(`Cycle detected among nodes: ${[...remaining].sort().join(", ")}`);
    }
    ready.sort();
    tiers.push({ tier: tiers.length, nodes: ready.map((id) => ontology.nodes.get(id)!) });
    for (const id of ready) {
      remaining.delete(id);
      completed.add(id);
    }
  }

  return tiers;
}

export function closestNodeIds(ontology: Ontology, input: string, limit = 3): string[] {
  return [...ontology.nodes.keys()]
    .map((id) => ({ id, score: editDistance(input, id) }))
    .sort((a, b) => a.score - b.score || a.id.localeCompare(b.id))
    .slice(0, limit)
    .map((entry) => entry.id);
}

function requireNode(ontology: Ontology, input: string): NodeMetadata {
  const node = getNode(ontology, input);
  if (!node) throw new StructuralError(`Node not found: ${input}`);
  return node;
}

function walkEdges(ontology: Ontology, startId: string, direction: "inbound" | "outbound", transitive: boolean): EdgeWithNode[] {
  const results: EdgeWithNode[] = [];
  const seen = new Set<string>();
  const queue = (ontology[direction].get(startId) ?? []).map((edge) => ({ edge, depth: 1 }));

  while (queue.length > 0) {
    const { edge, depth } = queue.shift()!;
    const nextId = direction === "inbound" ? edge.source : edge.target;
    if (seen.has(nextId)) continue;
    seen.add(nextId);
    results.push({ edge, node: ontology.nodes.get(nextId)!, depth });
    if (transitive) {
      queue.push(...(ontology[direction].get(nextId) ?? []).map((nextEdge) => ({ edge: nextEdge, depth: depth + 1 })));
    }
  }

  return results;
}

function editDistance(a: string, b: string): number {
  const dp = Array.from({ length: a.length + 1 }, () => Array<number>(b.length + 1).fill(0));
  for (let i = 0; i <= a.length; i += 1) dp[i]![0] = i;
  for (let j = 0; j <= b.length; j += 1) dp[0]![j] = j;
  for (let i = 1; i <= a.length; i += 1) {
    for (let j = 1; j <= b.length; j += 1) {
      dp[i]![j] = Math.min(
        dp[i - 1]![j]! + 1,
        dp[i]![j - 1]! + 1,
        dp[i - 1]![j - 1]! + (a[i - 1] === b[j - 1] ? 0 : 1),
      );
    }
  }
  return dp[a.length]![b.length]!;
}
