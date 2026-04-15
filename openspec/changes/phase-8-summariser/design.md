# Design: Phase 8 Summariser

## References

- `docs/spec.md` section 13 for summariser behavior.
- `docs/spec.md` section 11 for interface-hook integration.
- `docs/spec.md` section 15 for later brownfield use.

## Backend Interface

The summariser SHALL use a trait:

```rust
pub trait SummariserBackend {
    fn id(&self) -> SummariserBackendId;
    fn generate(&self, request: SummariserRequest<'_>) -> Result<SummariserDraft, SummariserError>;
}
```

Supported backend modes:

- `disabled`.
- `local_command`.
- `hosted_api`.

Hosted adapters SHALL receive credentials through environment variables or external config references, never through committed files.

`cairn.config.yaml` SHALL configure the summariser with this provider-neutral shape:

```yaml
summariser:
  mode: disabled # disabled | local_command | hosted_api
  timeout_ms: 30000
  max_prompt_bytes: 24000
  max_sample_bytes_per_file: 4000
  local_command:
    argv: ["cairn-summariser"]
  hosted_api:
    provider: "example"
    model: "example-model"
    credential_env: "CAIRN_SUMMARISER_API_KEY"
```

The local command backend SHALL send one JSON `SummariserRequest` to stdin and read one JSON `SummariserResponse` from stdout. Stderr SHALL be captured as diagnostic text only. Non-zero exit, timeout, invalid JSON, or response validation failure SHALL produce a backend failure without creating or modifying a draft.

`SummariserRequest` JSON SHALL include `schema_version`, `request_id`, `draft_type`, `target_node`, `ontology_facts`, `contract_excerpt`, `interface_findings`, `docstring_findings`, `project_context`, `rules`, and `code_samples`. `code_samples` SHALL obey `max_sample_bytes_per_file`; the full serialized request SHALL obey `max_prompt_bytes` after truncation.

`SummariserResponse` JSON SHALL include `schema_version`, `draft_text`, optional `summary`, and optional `metadata`. `draft_text` SHALL be the only field that can become contract or docstring prose; metadata SHALL be stored with the draft for audit only.

## Draft Storage

Drafts SHALL be stored under `.cairn/state/summariser/<draft-id>.json` with:

- Draft ID.
- Target node ID.
- Draft type.
- Backend ID.
- Created timestamp.
- Prompt input references.
- Generated text.
- Status: `pending`, `accepted`, `edited`, or `discarded`.

Generated contract text SHALL be staged as draft state until a resolution action occurs.

## Resolution Actions

- `accept`: replace the target contract with generated text and record the current interface hash.
- `edit`: create or update an editable draft file under `.cairn/state/summariser/editable/<draft-id>.md` and leave the contract unchanged.
- `accept --edited`: replace the target contract with the editable draft file content and record the current interface hash.
- `discard`: mark the draft discarded and leave the underlying contradiction unresolved.

The summariser SHALL never apply output during generation.

## CLI

Commands:

- `cairn summarise <node>`
- `cairn drafts`
- `cairn draft show <draft-id>`
- `cairn draft accept <draft-id>`
- `cairn draft accept <draft-id> --edited`
- `cairn draft edit <draft-id>`
- `cairn draft discard <draft-id>`

Commands SHALL support JSON output.

## MCP Tools

Phase 8 SHALL register summariser commands in the shared query tool registry so `cairn-mcp` exposes them through MCP. `cairn_drafts` and `cairn_draft_show` SHALL be `read_only` tools. `cairn_summarise`, `cairn_draft_accept`, `cairn_draft_edit`, and `cairn_draft_discard` SHALL be `mutating` tools and SHALL follow the MCP mutating-tool gate from Phase 7.

## Testing

Tests SHALL use deterministic fake backends. Coverage SHALL include disabled mode, config parsing, local command stdin/stdout JSON protocol, timeout, non-zero exit, invalid response handling, prompt input construction and truncation, draft persistence, accept, edit-file creation, edited accept, discard, interface hash recording, and MCP registry exposure for read-only and mutating draft tools.
