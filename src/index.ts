export { parseCairnDsl } from "./parser/parser.js";
export { buildOntology } from "./graph/builder.js";
export * from "./query/index.js";
export type { CairnAst, CairnNode, Edge } from "./parser/ast.js";
export type { Ontology, NodeMetadata } from "./graph/ontology.js";
