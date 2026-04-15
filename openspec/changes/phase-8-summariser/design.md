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
- `edit`: write editable draft content to a temp or state file, accept the edited content on explicit command, and record the current interface hash.
- `discard`: mark the draft discarded and leave the underlying contradiction unresolved.

The summariser SHALL never apply output during generation.

## CLI

Commands:

- `cairn summarise <node>`
- `cairn drafts`
- `cairn draft show <draft-id>`
- `cairn draft accept <draft-id>`
- `cairn draft edit <draft-id>`
- `cairn draft discard <draft-id>`

Commands SHALL support JSON output.

## Testing

Tests SHALL use deterministic fake backends. Coverage SHALL include disabled mode, backend failure, prompt input construction, draft persistence, accept, edit, discard, and interface hash recording.
