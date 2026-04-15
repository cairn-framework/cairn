import { describe, expect, it } from "vitest";
import { buildOntology } from "../../src/graph/builder.js";
import { parseCairnDsl } from "../../src/parser/parser.js";

const validDsl = `
System SaaS "Core" id "saas" {
  Container API "Backend" id "saas.api" {
    Module Auth "Auth" id "saas.api.auth" { path "./auth" }
    Module Billing "Billing" id "saas.api.billing" { path "./billing" }
  }
  Module DB "Database" id "saas.db" { path "./db" }
}
saas.api.auth -> saas.db "Reads"
saas.api.billing -> saas.db "Writes"
`;

describe("graph builder", () => {
  it("builds node, edge, parent, child, and name lookup indices", () => {
    const ontology = buildOntology(parseCairnDsl(validDsl));
    expect(ontology.nodes.get("saas.api.auth")?.name).toBe("Auth");
    expect(ontology.nameToId.get("Auth")).toBe("saas.api.auth");
    expect(ontology.parents.get("saas.api.auth")).toBe("saas.api");
    expect(ontology.children.get("saas.api")).toEqual(["saas.api.auth", "saas.api.billing"]);
    expect(ontology.inbound.get("saas.db")).toHaveLength(2);
    expect(ontology.outbound.get("saas.api.auth")).toHaveLength(1);
  });

  it("detects duplicate IDs", () => {
    const dsl = 'System SaaS "Core" id "saas" { Module A "A" id "saas.a" {} Module B "B" id "saas.a" {} }';
    expect(() => buildOntology(parseCairnDsl(dsl))).toThrow("Duplicate ID 'saas.a'");
  });

  it("detects duplicate leaf paths", () => {
    const dsl = 'System SaaS "Core" id "saas" { Module A "A" id "saas.a" { path "./same" } Module B "B" id "saas.b" { path "./same" } }';
    expect(() => buildOntology(parseCairnDsl(dsl))).toThrow("Duplicate path './same'");
  });

  it("detects unknown edge endpoints", () => {
    const dsl = 'System SaaS "Core" id "saas" { Module A "A" id "saas.a" {} } saas.a -> saas.missing "Nope"';
    expect(() => buildOntology(parseCairnDsl(dsl))).toThrow("Edge references unknown target ID 'saas.missing'");
  });

  it("detects invalid ID formats", () => {
    const dsl = 'System SaaS "Core" id "saas" { Module A "A" id "FooBar" {} }';
    expect(() => buildOntology(parseCairnDsl(dsl))).toThrow("Invalid ID 'FooBar'");
  });
});
