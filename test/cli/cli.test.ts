import { describe, expect, it } from "vitest";
import { execFileSync } from "node:child_process";

const cli = ["dist/src/cli/index.js"];
const fixture = "test/fixtures/cairn.dsl";

describe("cli", () => {
  it("prints get output for a fixture node", () => {
    const output = execFileSync("node", [...cli, "get", "cairn.kernel.parser", "--file", fixture], { encoding: "utf8" });
    expect(output).toContain("Module: Parser");
    expect(output).toContain("ID: cairn.kernel.parser");
  });

  it("prints stable JSON output", () => {
    const output = execFileSync("node", [...cli, "get", "Parser", "--file", fixture, "--json"], { encoding: "utf8" });
    expect(JSON.parse(output)).toMatchObject({
      schema_version: "cairn.query.v1",
      command: "get",
      result: { id: "cairn.kernel.parser" },
    });
  });

  it("prints node-not-found suggestions to stderr and exits 1", () => {
    expect(() => execFileSync("node", [...cli, "get", "cairn.kernel.parse", "--file", fixture], { encoding: "utf8" })).toThrow(
      /Did you mean: cairn.kernel.parser/,
    );
  });
});
