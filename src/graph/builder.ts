import type { CairnAst, CairnNode } from "../parser/ast.js";
import { StructuralError } from "../parser/errors.js";
import { toNodeMetadata, type Ontology } from "./ontology.js";

const idPattern = /^[a-z][a-z0-9-]*(\.[a-z][a-z0-9-]*)*$/;

export function buildOntology(ast: CairnAst, source = "<input>"): Ontology {
  const ontology: Ontology = {
    nodes: new Map(),
    nameToId: new Map(),
    outbound: new Map(),
    inbound: new Map(),
    parents: new Map(),
    children: new Map(),
  };
  const paths = new Map<string, { id: string; line: number; column: number }>();

  const visit = (node: CairnNode, parentId?: string): void => {
    if (!node.id) {
      throw new StructuralError(`${node.kind} '${node.name}' is missing required field: id`, { source, position: node.position });
    }
    if (!node.description) {
      throw new StructuralError(`${node.kind} '${node.name}' is missing required field: description`, { source, position: node.position });
    }
    if (!idPattern.test(node.id)) {
      throw new StructuralError(`Invalid ID '${node.id}'. Expected pattern: ${idPattern.source}`, {
        source,
        position: node.position,
      });
    }
    const existing = ontology.nodes.get(node.id);
    if (existing) {
      throw new StructuralError(
        `Duplicate ID '${node.id}' first declared at ${existing.position.line}:${existing.position.column}, repeated at ${node.position.line}:${node.position.column}`,
        { source, position: node.position },
      );
    }

    const metadata = toNodeMetadata(node, parentId);
    ontology.nodes.set(metadata.id, metadata);
    ontology.nameToId.set(metadata.name, metadata.id);
    ontology.outbound.set(metadata.id, []);
    ontology.inbound.set(metadata.id, []);
    if (parentId) {
      ontology.parents.set(metadata.id, parentId);
      ontology.children.set(parentId, [...(ontology.children.get(parentId) ?? []), metadata.id]);
    }

    if (node.children.length === 0) {
      for (const path of node.paths) {
        const previous = paths.get(path);
        if (previous) {
          throw new StructuralError(
            `Duplicate path '${path}' claimed by '${previous.id}' at ${previous.line}:${previous.column} and '${node.id}'`,
            { source, position: node.position },
          );
        }
        paths.set(path, { id: node.id, ...node.position });
      }
    }

    for (const child of node.children) visit(child, node.id);
  };

  visit(ast.root);

  for (const edge of ast.edges) {
    if (!ontology.nodes.has(edge.source)) {
      throw new StructuralError(`Edge references unknown source ID '${edge.source}'`, { source, position: edge.position });
    }
    if (!ontology.nodes.has(edge.target)) {
      throw new StructuralError(`Edge references unknown target ID '${edge.target}'`, { source, position: edge.position });
    }
    ontology.outbound.get(edge.source)!.push(edge);
    ontology.inbound.get(edge.target)!.push(edge);
  }

  return ontology;
}
