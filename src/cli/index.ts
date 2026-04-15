import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { buildOntology } from "../graph/builder.js";
import { CairnError, StructuralError } from "../parser/errors.js";
import { parseCairnDsl } from "../parser/parser.js";
import { closestNodeIds, dependencyOrder, dependents, depends, getNode, neighbourhood } from "../query/index.js";
import { renderJson } from "./output/json.js";
import { renderNeighbourhood, renderNode, renderNodeList, renderOrder } from "./output/human.js";

interface GlobalOptions {
  file: string;
  json: boolean;
}

interface ParsedArgs {
  command: string;
  operands: string[];
  global: GlobalOptions;
  local: Record<string, string | boolean>;
}

main(process.argv.slice(2));

function main(argv: string[]): void {
  try {
    const args = parseArgs(argv);
    const ontology = loadOntology(args.global.file);

    switch (args.command) {
      case "get": {
        const node = requiredOperand(args, "<node>");
        const result = getNode(ontology, node);
        if (!result) throw notFound(ontology, node);
        output(args.global, "get", result, () => renderNode(result));
        return;
      }
      case "neighbourhood": {
        const result = neighbourhood(ontology, requiredOperand(args, "<node>"));
        output(args.global, "neighbourhood", result, () => renderNeighbourhood(result));
        return;
      }
      case "dependents": {
        const result = dependents(ontology, requiredOperand(args, "<node>"), Boolean(args.local.transitive));
        output(args.global, "dependents", result, () => renderNodeList("Dependents", result));
        return;
      }
      case "depends": {
        const result = depends(ontology, requiredOperand(args, "<node>"), Boolean(args.local.transitive));
        output(args.global, "depends", result, () => renderNodeList("Depends", result));
        return;
      }
      case "order": {
        const result = dependencyOrder(ontology, {
          from: typeof args.local.from === "string" ? args.local.from : undefined,
          scope: typeof args.local.scope === "string" ? args.local.scope : undefined,
        });
        output(args.global, "order", result, () => renderOrder(result));
        return;
      }
      default:
        throw new StructuralError(`Unknown command: ${args.command}`);
    }
  } catch (error) {
    reportError(error);
  }
}

function parseArgs(argv: string[]): ParsedArgs {
  const global: GlobalOptions = { file: "./cairn.dsl", json: false };
  const local: Record<string, string | boolean> = {};
  const operands: string[] = [];
  let command: string | undefined;

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index]!;
    if (arg === "--json") {
      global.json = true;
    } else if (arg === "--file") {
      global.file = argv[++index] ?? "";
    } else if (arg.startsWith("--file=")) {
      global.file = arg.slice("--file=".length);
    } else if (arg === "--transitive") {
      local.transitive = true;
    } else if (arg === "--from") {
      local.from = argv[++index] ?? "";
    } else if (arg.startsWith("--from=")) {
      local.from = arg.slice("--from=".length);
    } else if (arg === "--scope") {
      local.scope = argv[++index] ?? "";
    } else if (arg.startsWith("--scope=")) {
      local.scope = arg.slice("--scope=".length);
    } else if (arg.startsWith("--")) {
      throw new StructuralError(`Unknown option: ${arg}`);
    } else if (!command) {
      command = arg;
    } else {
      operands.push(arg);
    }
  }

  if (!command) throw new StructuralError("Missing command");
  if (!global.file) throw new StructuralError("--file requires a path");
  return { command, operands, global, local };
}

function requiredOperand(args: ParsedArgs, label: string): string {
  const value = args.operands[0];
  if (!value) throw new StructuralError(`${args.command} requires ${label}`);
  return value;
}

function loadOntology(file: string) {
  const source = resolve(file);
  const content = readFileSync(source, "utf8");
  return buildOntology(parseCairnDsl(content, source), source);
}

function output(options: GlobalOptions, command: string, result: unknown, human: () => string): void {
  process.stdout.write(options.json ? `${renderJson(command, result)}\n` : `${human()}\n`);
}

function notFound(ontology: ReturnType<typeof loadOntology>, input: string): StructuralError {
  const suggestions = closestNodeIds(ontology, input);
  return new StructuralError(`Node not found: ${input}${suggestions.length ? `\nDid you mean: ${suggestions.join(", ")}` : ""}`);
}

function reportError(error: unknown): never {
  if (error instanceof CairnError) {
    process.stderr.write(`${error.format()}\n`);
  } else if (error instanceof Error) {
    process.stderr.write(`${error.message}\n`);
  } else {
    process.stderr.write(`${String(error)}\n`);
  }
  process.exit(1);
}
