import type { CairnNode, Edge, NodeKind } from "../parser/ast.js";
import type { SourcePosition } from "../parser/errors.js";

export interface NodeMetadata {
  kind: NodeKind;
  id: string;
  name: string;
  description: string;
  tags: string[];
  paths: string[];
  artefacts: Record<string, string[]>;
  state: "declared";
  parentId?: string;
  position: SourcePosition;
}

export interface GraphEdge extends Edge {}

export interface Ontology {
  nodes: Map<string, NodeMetadata>;
  nameToId: Map<string, string>;
  outbound: Map<string, GraphEdge[]>;
  inbound: Map<string, GraphEdge[]>;
  parents: Map<string, string>;
  children: Map<string, string[]>;
}

export function toNodeMetadata(node: CairnNode, parentId?: string): NodeMetadata {
  if (!node.id) throw new Error("Cannot convert node without ID");
  return {
    kind: node.kind,
    id: node.id,
    name: node.name,
    description: node.description,
    tags: [...node.tags],
    paths: [...node.paths],
    artefacts: structuredClone(node.artefacts),
    state: "declared",
    parentId,
    position: node.position,
  };
}
