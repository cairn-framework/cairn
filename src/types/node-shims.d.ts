declare module "node:fs" {
  export function readFileSync(path: string, encoding: "utf8"): string;
}

declare module "node:path" {
  export function resolve(...paths: string[]): string;
}

declare const process: {
  argv: string[];
  stdout: { write(value: string): void };
  stderr: { write(value: string): void };
  exit(code?: number): never;
};
