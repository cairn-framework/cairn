import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { buildOntology } from "../src/graph/builder.js";
import { parseCairnDsl } from "../src/parser/parser.js";
import { dependents, getNode, neighbourhood } from "../src/query/index.js";

const fixture = readFileSync("test/fixtures/cairn.dsl", "utf8");
const ontology = buildOntology(parseCairnDsl(fixture, "test/fixtures/cairn.dsl"), "test/fixtures/cairn.dsl");

describe("self-hosting fixture", () => {
  it("gets parser metadata", () => {
    expect(getNode(ontology, "cairn.kernel.parser")).toMatchObject({
      id: "cairn.kernel.parser",
      name: "Parser",
      description: "Parses .dsl files into a node graph",
    });
  });

  it("returns the five reconciliation connections", () => {
    const result = neighbourhood(ontology, "cairn.kernel.reconciliation");
    expect([...result.inbound, ...result.outbound].map((entry) => entry.node.id).sort()).toEqual([
      "cairn.kernel.artefacts",
      "cairn.kernel.changes",
      "cairn.kernel.hooks",
      "cairn.kernel.parser",
      "cairn.kernel.query",
    ]);
  });

  it("returns code reconciler as the reconciler interface dependent", () => {
    expect(dependents(ontology, "cairn.kernel.reconciler").map((entry) => entry.node.id)).toEqual(["cairn.reconcilers.code"]);
  });
});
