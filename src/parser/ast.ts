import type { SourcePosition } from "./errors.js";

export type NodeKind = "System" | "Container" | "Module" | "Actor";

export interface Edge {
  source: string;
  target: string;
  description: string;
  position: SourcePosition;
}

export interface CairnNode {
  kind: NodeKind;
  name: string;
  description: string;
  id?: string;
  tags: string[];
  paths: string[];
  artefacts: Record<string, string[]>;
  children: CairnNode[];
  position: SourcePosition;
}

export interface CairnAst {
  root: CairnNode;
  edges: Edge[];
}

export const nodeKeywords = ["System", "Container", "Module", "Actor"] as const;

export const artefactKeys = ["contract", "todos", "decisions", "research", "reviews", "sources"] as const;

export type ArtefactKey = (typeof artefactKeys)[number];
