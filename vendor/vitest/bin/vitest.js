#!/usr/bin/env node
import { mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { pathToFileURL } from "node:url";
import { createRequire } from "node:module";

const require = createRequire(join(process.cwd(), "package.json"));
const ts = require("typescript");
const root = process.cwd();
const outDir = join(root, "dist", "test");

globalThis.__vitestShim = { suites: [], current: [] };
rmSync(outDir, { recursive: true, force: true });

const testFiles = findTests(join(root, "test"));
for (const file of testFiles) {
  const rel = relative(join(root, "test"), file);
  const target = join(outDir, rel).replace(/\.ts$/, ".js");
  mkdirSync(dirname(target), { recursive: true });
  const source = readFileSync(file, "utf8");
  const output = ts.transpileModule(source, {
    compilerOptions: {
      module: ts.ModuleKind.ES2022,
      target: ts.ScriptTarget.ES2022,
      moduleResolution: ts.ModuleResolutionKind.NodeNext,
      esModuleInterop: true,
    },
    fileName: file,
  }).outputText;
  writeFileSync(target, output);
}

for (const file of findTests(outDir, ".js")) {
  await import(pathToFileURL(file).href);
}

let failures = 0;
for (const test of globalThis.__vitestShim.suites) {
  try {
    await test.fn();
    process.stdout.write(`PASS ${test.name}\n`);
  } catch (error) {
    failures += 1;
    process.stderr.write(`FAIL ${test.name}\n${error?.stack ?? error}\n`);
  }
}

process.stdout.write(`\n${globalThis.__vitestShim.suites.length - failures} passed, ${failures} failed\n`);
process.exit(failures === 0 ? 0 : 1);

function findTests(dir, ext = ".ts") {
  const found = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) found.push(...findTests(path, ext));
    if (entry.isFile() && entry.name.endsWith(`.test${ext}`)) found.push(path);
  }
  return found.sort();
}
