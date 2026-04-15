import { describe, expect, it } from "vitest";
import { buildOntology } from "../../src/graph/builder.js";
import { parseCairnDsl } from "../../src/parser/parser.js";

const fixture = `
System SaaS "Core product" id "saas" @product {
  Container API "Backend" id "saas.api" {
    Module Auth "JWT auth" id "saas.api.auth" @auth {
      path ["./apps/api/auth", "./apps/api/auth-ts"]
      contract "./meta/contracts/auth.md"
      todos "./meta/todos/auth/"
      decisions "./meta/decisions/auth/"
      research "./meta/research/auth/"
      reviews "./meta/reviews/auth/"
    }
  }
  Module DB "Postgres" id "saas.db" {
    path "./infra/db"
  }
}
saas.api.auth -> saas.db "Reads users"
`;

describe("parser", () => {
  it("parses top-level system, nested containers/modules, tags, artefacts, and edges", () => {
    const ast = parseCairnDsl(fixture);
    const api = ast.root.children[0]!;
    const auth = api.children[0]!;

    expect(ast.root.id).toBe("saas");
    expect(api.kind).toBe("Container");
    expect(auth.kind).toBe("Module");
    expect(auth.tags).toEqual(["auth"]);
    expect(auth.paths).toEqual(["./apps/api/auth", "./apps/api/auth-ts"]);
    expect(auth.artefacts.contract).toEqual(["./meta/contracts/auth.md"]);
    expect(ast.edges).toEqual([
      expect.objectContaining({ source: "saas.api.auth", target: "saas.db", description: "Reads users" }),
    ]);
  });

  it("reports missing required IDs with node source position", () => {
    const ast = parseCairnDsl('System SaaS "Core" id "saas" { Module Auth "JWT auth" { path "./auth" } }');
    expect(() => buildOntology(ast, "missing.dsl")).toThrow("Module 'Auth' is missing required field: id");
  });

  it("reports unknown top-level keywords with valid alternatives", () => {
    expect(() => parseCairnDsl('Service Foo "Nope" id "foo"')).toThrow(
      "Unknown keyword 'Service'. Valid declarations: System, Container, Module, Actor",
    );
  });

  it("normalizes a single path into an array", () => {
    const ast = parseCairnDsl('System SaaS "Core" id "saas" { Module Auth "JWT auth" id "saas.auth" { path "./auth" } }');
    expect(ast.root.children[0]?.paths).toEqual(["./auth"]);
  });
});
