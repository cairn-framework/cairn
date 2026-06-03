# Summariser

The Cairn summariser is an optional backend that proposes contract drafts when interface contradictions are detected. It is disabled by default. When enabled, it sends structured prompt inputs to a configured backend and stores the response as a pending draft. Drafts are never auto-applied. A human or agent must explicitly accept, edit, or discard each draft before it affects the project.

## Configuration

Add a `summariser:` section to `cairn.config.yaml` at your project root.

```yaml
summariser:
  mode: disabled           # disabled | local_command | hosted_api
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

### Fields

| Field | Default | Description |
|---|---|---|
| `mode` | `disabled` | Backend to use. `disabled` skips generation. `local_command` runs a local executable. `hosted_api` is a configuration boundary for future adapters. |
| `timeout_ms` | `30000` | Milliseconds to wait for a backend response. |
| `max_prompt_bytes` | `24000` | Maximum serialized request size. Larger requests drop code samples first, then truncate the contract excerpt, then the project context. |
| `max_sample_bytes_per_file` | `4000` | Maximum bytes per individual code sample. |
| `local_command.argv` | none | Command and arguments to invoke for `local_command` mode. |
| `hosted_api.provider` | none | Provider identifier (reserved for future adapters). |
| `hosted_api.model` | none | Model identifier (reserved for future adapters). |
| `hosted_api.credential_env` | none | Environment variable name holding the API key. Never commit keys to version control. |

### Security

Credentials for hosted backends MUST be supplied through environment variables or external secret stores. Cairn never reads credentials from committed files.

## Wire protocol

### Request

`local_command` backends receive one JSON object on stdin:

```json
{
  "schema_version": 1,
  "request_id": "uuid",
  "draft_type": "contract",
  "target_node": "app.auth",
  "map_facts": ["Module Auth ..."],
  "contract_excerpt": "Current contract text",
  "interface_findings": ["public fn login() added"],
  "docstring_findings": [],
  "project_context": "Project context from config",
  "rules": ["rule text"],
  "code_samples": [
    {"path": "src/auth.rs", "language": "rust", "content": "..."}
  ]
}
```

Empty collections are omitted from the JSON.

### Response

Backends write one JSON object to stdout:

```json
{
  "schema_version": 1,
  "draft_text": "# Auth\n\nUpdated contract prose.",
  "summary": "Optional human-readable summary.",
  "metadata": {}
}
```

Only `draft_text` is used for contract content. `summary` and `metadata` are stored for audit only.

## Draft storage

Drafts live under `.cairn/state/summariser/`:

```
.cairn/state/summariser/
  draft-<id>.json          # pending / accepted / discarded draft
  editable/
    draft-<id>.md          # editable version written by draft edit
```

Each draft records its status, target node, draft text, creation time, transition history, and optional backend metadata.

## Resolution actions

### `cairn draft accept <draft-id>`

Validates the draft text as a contract for the target node, replaces the target contract file, records the current interface hash, and transitions the draft to `accepted`.

Validation checks:
- The draft text has valid Markdown frontmatter.
- The `node` frontmatter value matches the draft's target node.
- The post-write project scan produces no errors.

If validation or the post-write scan fails, the original contract is restored, the draft stays `pending`, and the command exits with code `1`.

### `cairn draft accept <draft-id> --edited`

Same as `accept`, but reads the contract text from the editable file (`.cairn/state/summariser/editable/draft-<id>.md`) instead of the draft's stored `draft_text`. The same validation and rollback guarantees apply.

### `cairn draft edit <draft-id>`

Writes the draft text to an editable file without modifying the contract. Transitions the draft to `edited`. Use this when you want to revise the generated text before accepting it.

### `cairn draft discard <draft-id>`

Marks the draft as `discarded` and leaves the underlying contradiction unresolved. The contract is not modified.

## MCP exposure

When Cairn runs as an MCP server (`cairn-mcp`), summariser tools follow the same read-only versus mutating safety model as the rest of the tool registry.

### Read-only tools (always visible)

- `cairn_drafts` — list all drafts.
- `cairn_draft_show` — show a single draft by ID.

### Mutating tools (require `--allow-mutating-tools`)

- `cairn_summarise` — generate a new draft for a node.
- `cairn_draft_accept` — accept a draft (optionally with `--edited`).
- `cairn_draft_edit` — create an editable draft file.
- `cairn_draft_discard` — discard a draft.

Mutating tool calls must also pass `"mutating": true` in their arguments, in addition to the server being started with `--allow-mutating-tools`.

## Safety model

The summariser never modifies contract files during generation. A draft is created in `pending` state and remains inert until a resolution action runs. This three-action design (accept, edit, discard) prevents the summariser from degrading into an auto-applier. The human or agent retains explicit authority over every contract change.

Rollback on failed acceptance restores the original contract from a snapshot taken before the write attempt. The draft stays pending so the user can inspect the failure, edit the text, and retry.
