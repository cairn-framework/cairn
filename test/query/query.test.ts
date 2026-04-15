import { describe, expect, it } from "vitest";
import { buildOntology } from "../../src/graph/builder.js";
import { parseCairnDsl } from "../../src/parser/parser.js";
import { dependencyOrder, dependents, depends, getNode, neighbourhood } from "../../src/query/index.js";

const dsl = `
System SaaS "Core" id "saas" {
  Module DB "Database" id "saas.db" {}
  Module Crypto "Crypto" id "saas.crypto" {}
  Module Auth "Auth" id "saas.api.auth" {}
  Module Billing "Billing" id "saas.api.billing" {}
  Module Admin "Admin" id "saas.api.admin" {}
}
saas.api.auth -> saas.db "Reads users"
saas.api.auth -> saas.crypto "Hashes passwords"
saas.api.billing -> saas.db "Writes ledger"
saas.api.admin -> saas.api.auth "Manages sessions"
`;

describe("queries", () => {
  const ontology = buildOntology(parseCairnDsl(dsl));

  it("gets nodes by ID or name", () => {
    expect(getNode(ontology, "saas.api.auth")?.name).toBe("Auth");
    expect(getNode(ontology, "Auth")?.id).toBe("saas.api.auth");
  });

  it("returns neighbourhood with inline connected node metadata", () => {
    const result = neighbourhood(ontology, "saas.api.auth");
    expect(result.inbound.map((entry) => entry.node.id)).toEqual(["saas.api.admin"]);
    expect(result.outbound.map((entry) => entry.node.id)).toEqual(["saas.db", "saas.crypto"]);
  });

  it("returns direct and transitive dependents", () => {
    expect(dependents(ontology, "saas.db").map((entry) => entry.node.id)).toEqual(["saas.api.auth", "saas.api.billing"]);
    expect(dependents(ontology, "saas.db", true).map((entry) => entry.node.id)).toEqual([
      "saas.api.auth",
      "saas.api.billing",
      "saas.api.admin",
    ]);
  });

  it("returns direct dependencies", () => {
    expect(depends(ontology, "saas.api.auth").map((entry) => entry.node.id)).toEqual(["saas.db", "saas.crypto"]);
  });

  it("returns dependency-tier order", () => {
    expect(dependencyOrder(ontology).map((tier) => tier.nodes.map((node) => node.id))).toEqual([
      ["saas", "saas.crypto", "saas.db"],
      ["saas.api.auth", "saas.api.billing"],
      ["saas.api.admin"],
    ]);
  });

  it("filters order by scope", () => {
    expect(dependencyOrder(ontology, { scope: "saas.api." }).map((tier) => tier.nodes.map((node) => node.id))).toEqual([
      ["saas.api.auth", "saas.api.billing"],
      ["saas.api.admin"],
    ]);
  });

  it("detects cycles in order", () => {
    const cycle = 'System S "S" id "s" { Module A "A" id "s.a" {} Module B "B" id "s.b" {} } s.a -> s.b "a" s.b -> s.a "b"';
    expect(() => dependencyOrder(buildOntology(parseCairnDsl(cycle)))).toThrow("Cycle detected among nodes");
  });
});
