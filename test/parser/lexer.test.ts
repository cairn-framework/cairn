import { describe, expect, it } from "vitest";
import { lex } from "../../src/parser/lexer.js";

describe("lexer", () => {
  it("tokenizes keywords, identifiers, strings, IDs, tags, arrows, braces, and comments", () => {
    const tokens = lex('System SaaS "Core" id "saas" @app { # comment\nsaas.api -> saas.db "Reads" }');
    expect(tokens.map((token) => token.type)).toEqual([
      "identifier",
      "identifier",
      "string",
      "identifier",
      "string",
      "tag",
      "braceOpen",
      "identifier",
      "arrow",
      "identifier",
      "string",
      "braceClose",
      "eof",
    ]);
  });

  it("tracks line and column positions", () => {
    const tokens = lex('System SaaS "Core"\nModule Auth "Auth"');
    expect(tokens[3]?.position).toEqual({ line: 2, column: 1 });
  });
});
