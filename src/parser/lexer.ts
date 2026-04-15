import { ParseError, type SourcePosition } from "./errors.js";

export type TokenType =
  | "identifier"
  | "string"
  | "tag"
  | "arrow"
  | "braceOpen"
  | "braceClose"
  | "bracketOpen"
  | "bracketClose"
  | "comma"
  | "eof";

export interface Token {
  type: TokenType;
  value: string;
  position: SourcePosition;
}

const isIdentifierChar = (char: string): boolean => /[A-Za-z0-9_.-]/.test(char);

export function lex(input: string, source = "<input>"): Token[] {
  const tokens: Token[] = [];
  let index = 0;
  let line = 1;
  let column = 1;

  const position = (): SourcePosition => ({ line, column });
  const current = (): string => input[index] ?? "";
  const next = (): string => input[index + 1] ?? "";
  const advance = (): string => {
    const char = input[index++] ?? "";
    if (char === "\n") {
      line += 1;
      column = 1;
    } else {
      column += 1;
    }
    return char;
  };
  const push = (type: TokenType, value: string, tokenPosition: SourcePosition): void => {
    tokens.push({ type, value, position: tokenPosition });
  };

  while (index < input.length) {
    const char = current();

    if (char === " " || char === "\t" || char === "\r" || char === "\n") {
      advance();
      continue;
    }

    if (char === "#") {
      while (index < input.length && current() !== "\n") advance();
      continue;
    }

    const tokenPosition = position();

    if (char === "{") {
      advance();
      push("braceOpen", char, tokenPosition);
      continue;
    }
    if (char === "}") {
      advance();
      push("braceClose", char, tokenPosition);
      continue;
    }
    if (char === "[") {
      advance();
      push("bracketOpen", char, tokenPosition);
      continue;
    }
    if (char === "]") {
      advance();
      push("bracketClose", char, tokenPosition);
      continue;
    }
    if (char === ",") {
      advance();
      push("comma", char, tokenPosition);
      continue;
    }
    if (char === "-" && next() === ">") {
      advance();
      advance();
      push("arrow", "->", tokenPosition);
      continue;
    }
    if (char === '"') {
      advance();
      let value = "";
      while (index < input.length && current() !== '"') {
        if (current() === "\n") {
          throw new ParseError("Unterminated string literal", { source, position: tokenPosition });
        }
        if (current() === "\\") {
          advance();
          const escaped = advance();
          value += escaped === "n" ? "\n" : escaped;
        } else {
          value += advance();
        }
      }
      if (current() !== '"') {
        throw new ParseError("Unterminated string literal", { source, position: tokenPosition });
      }
      advance();
      push("string", value, tokenPosition);
      continue;
    }
    if (char === "@") {
      advance();
      let value = "";
      while (isIdentifierChar(current())) value += advance();
      if (!value) throw new ParseError("Expected tag name after @", { source, position: tokenPosition });
      push("tag", value, tokenPosition);
      continue;
    }
    if (isIdentifierChar(char)) {
      let value = "";
      while (isIdentifierChar(current())) value += advance();
      push("identifier", value, tokenPosition);
      continue;
    }

    throw new ParseError(`Unexpected character '${char}'`, { source, position: tokenPosition });
  }

  tokens.push({ type: "eof", value: "", position: position() });
  return tokens;
}
