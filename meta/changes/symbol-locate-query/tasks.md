# Tasks: symbol-locate-query

- [x] Add `QueryRequest.symbol: Option<String>` and update every
      non-defaulted `QueryRequest { .. }` literal (CLI, MCP, tests) to set it
- [x] Add `locate` / `cairn_locate` to `query_api::registry::TOOL_REGISTRY`
      (bump size test 42 -> 43) and dispatch it in `execute_data_with_scan`
- [x] Implement `src/query_api/handlers/locate.rs::locate_json` (reverse
      index over `NodeRecord.symbols`, exact match, all collisions returned)
- [x] Wire MCP: `request_from_arguments` reads `"symbol"`; `input_schema`
      gains a `"LocateRequest"` case
- [x] Wire CLI: `locate` subcommand (`uses_shared_json`, `shared_request`
      symbol population, `render_loaded_project_command` dispatch,
      `render_locate` human renderer, `symbol_arg` helper, help entry)
- [x] Add `copy.toml` entries (`[locate]`, `[locate] no-matches and missing-symbol keys`,
      `[help.commands.locate]`)
- [x] Unit/integration tests: known symbol resolves to correct node/file/
      line; collision returns all matches; zero matches is a clean empty
      result
- [x] Regenerate `map.json` (`cairn scan`) and the `api_meta` wire snapshot;
      review the diff deliberately before accepting
- [x] Run the full gate suite (fmt, clippy, test, scan --strict) green
- [x] Manual acceptance transcript: real symbol, collision, no-match
- [x] Flip `meta/todos/todo.symbol-locate-query.md` to done with a dated
      Resolution section
