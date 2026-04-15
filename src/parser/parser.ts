import { artefactKeys, nodeKeywords, type CairnAst, type CairnNode, type Edge, type NodeKind } from "./ast.js";
import { lex, type Token, type TokenType } from "./lexer.js";
import { ParseError } from "./errors.js";

const validNodeKeywords = nodeKeywords.join(", ");

export function parseCairnDsl(input: string, source = "<input>"): CairnAst {
  return new Parser(lex(input, source), source).parse();
}

class Parser {
  private index = 0;

  constructor(
    private readonly tokens: Token[],
    private readonly source: string,
  ) {}

  parse(): CairnAst {
    const root = this.parseNode(["System"]);
    const edges: Edge[] = [];

    while (!this.check("eof")) {
      if (this.check("identifier") && this.checkNext("arrow")) {
        edges.push(this.parseEdge());
        continue;
      }

      const token = this.peek();
      if (this.check("identifier")) {
        throw new ParseError(`Unknown keyword '${token.value}'. Valid declarations: ${validNodeKeywords}`, {
          source: this.source,
          position: token.position,
        });
      }
      throw this.expected("edge or end of file");
    }

    return { root, edges };
  }

  private parseNode(allowedKinds: NodeKind[]): CairnNode {
    const keyword = this.consume("identifier", "node keyword");
    if (!nodeKeywords.includes(keyword.value as NodeKind)) {
      throw new ParseError(`Unknown keyword '${keyword.value}'. Valid declarations: ${validNodeKeywords}`, {
        source: this.source,
        position: keyword.position,
      });
    }
    if (!allowedKinds.includes(keyword.value as NodeKind)) {
      throw new ParseError(`${keyword.value} is not valid in this position`, {
        source: this.source,
        position: keyword.position,
      });
    }

    const name = this.consume("identifier", "node name");
    const description = this.consume("string", "node description");
    const node: CairnNode = {
      kind: keyword.value as NodeKind,
      name: name.value,
      description: description.value,
      tags: [],
      paths: [],
      artefacts: {},
      children: [],
      position: keyword.position,
    };

    this.parseInlineNodeMetadata(node);

    if (this.match("braceOpen")) {
      while (!this.check("braceClose") && !this.check("eof")) {
        if (this.check("identifier") && nodeKeywords.includes(this.peek().value as NodeKind)) {
          node.children.push(this.parseNode(["Container", "Module", "Actor"]));
        } else {
          this.parseNodeAttribute(node);
        }
      }
      this.consume("braceClose", "}");
    }

    return node;
  }

  private parseInlineNodeMetadata(node: CairnNode): void {
    while (!this.check("braceOpen") && !this.check("braceClose") && !this.check("eof")) {
      if (this.check("tag")) {
        node.tags.push(this.advance().value);
        continue;
      }
      if (this.check("identifier") && this.peek().value === "id") {
        this.advance();
        node.id = this.consume("string", "id value").value;
        continue;
      }
      break;
    }
  }

  private parseNodeAttribute(node: CairnNode): void {
    const key = this.consume("identifier", "attribute name");
    if (key.value === "id") {
      node.id = this.consume("string", "id value").value;
      return;
    }
    if (key.value === "path") {
      node.paths = this.parseStringOrList();
      return;
    }
    if (artefactKeys.includes(key.value as never)) {
      node.artefacts[key.value] = this.parseStringOrList();
      return;
    }
    throw new ParseError(`Unknown attribute '${key.value}'`, { source: this.source, position: key.position });
  }

  private parseStringOrList(): string[] {
    if (this.match("bracketOpen")) {
      const values: string[] = [];
      while (!this.check("bracketClose") && !this.check("eof")) {
        values.push(this.consume("string", "string list item").value);
        if (!this.match("comma") && !this.check("bracketClose")) {
          throw this.expected(", or ]");
        }
      }
      this.consume("bracketClose", "]");
      return values;
    }
    return [this.consume("string", "string value").value];
  }

  private parseEdge(): Edge {
    const source = this.consume("identifier", "edge source");
    this.consume("arrow", "->");
    const target = this.consume("identifier", "edge target");
    const description = this.consume("string", "edge description");
    return {
      source: source.value,
      target: target.value,
      description: description.value,
      position: source.position,
    };
  }

  private peek(): Token {
    return this.tokens[this.index]!;
  }

  private check(type: TokenType): boolean {
    return this.peek().type === type;
  }

  private checkNext(type: TokenType): boolean {
    return this.tokens[this.index + 1]?.type === type;
  }

  private advance(): Token {
    return this.tokens[this.index++]!;
  }

  private match(type: TokenType): boolean {
    if (!this.check(type)) return false;
    this.advance();
    return true;
  }

  private consume(type: TokenType, label: string): Token {
    if (this.check(type)) return this.advance();
    throw this.expected(label);
  }

  private expected(label: string): ParseError {
    const token = this.peek();
    return new ParseError(`Expected ${label}, found ${token.type}${token.value ? ` '${token.value}'` : ""}`, {
      source: this.source,
      position: token.position,
    });
  }
}
