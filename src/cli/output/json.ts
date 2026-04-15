export function renderJson(command: string, result: unknown): string {
  return JSON.stringify({
    schema_version: "cairn.query.v1",
    command,
    result,
  });
}
