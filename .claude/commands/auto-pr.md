# /auto-pr — Review, fix, and merge an open PR

Review an open pull request, fix any quality gate failures, and merge when clean.

## Usage

```
/auto-pr <pr-number>
```

## Workflow

### Phase 1: Assess

Run the assessment script to check out the PR and run all gates:

```bash
scripts/auto-pr.sh <pr-number>
```

This will:
- Check out the PR branch
- Run cargo fmt, clippy, test
- Run cairn lint and cairn hook all
- Run the dogfood gate

If all gates pass, the script will merge the PR automatically (squash + delete branch).

### Phase 2: Review and Fix (if gates failed)

If any gate fails:

1. **Inspect failures**: Read the gate output to understand what failed.
2. **Review the diff**: Run `git diff <base-branch>..HEAD` to see the full change set.
3. **Fix issues**:
   - `cargo fmt` failures → run `cargo fmt`
   - `cargo clippy` failures → fix the code; add `#[allow(...)]` with `// Reason:` comments only when justified
   - `cargo test` failures → fix the underlying code or test
   - `cairn lint` findings → fix blueprint/contract issues
   - `cairn hook all` findings → fix structural errors or update interface hashes with `cairn scan`
   - `dogfood` failures → usually cairn hook all or lint; fix the underlying issue
4. **Commit fixes**: `git add -A && git commit -m "fix: <concise description>"`
5. **Push**: `git push`
6. **Re-assess**: Run `scripts/auto-pr.sh <pr-number> --fix` to re-run gates (does not merge).
7. **Loop**: Repeat Phase 2 until all gates pass.

### Phase 3: Merge

Once all gates pass, run:

```bash
scripts/auto-pr.sh <pr-number>
```

This will merge the PR with squash and delete the branch.

## Safety rules (hard stops)

- **Do NOT proceed** if the PR modifies `.github/workflows/`, `Cargo.toml`, `.cflx.jsonc`, or `scripts/dogfood.sh`. Escalate to the user.
- **Do NOT proceed** if the PR has >30 files changed or >1000 total lines changed. Escalate to the user.
- **Do NOT proceed** if the PR is a draft or from a fork.
- **Do NOT proceed** if you cannot understand or fix a test failure. Escalate to the user.

## Gate reference

| Gate | Command | Fix |
|------|---------|-----|
| Format | `cargo fmt --check` | `cargo fmt` |
| Clippy | `cargo clippy --lib --tests` | Fix code; allow attrs with `// Reason:` |
| Tests | `cargo test` | Fix code or test |
| Lint | `cairn lint` | Fix blueprint/contracts |
| Hooks | `cairn hook all` | Fix structural issues; `cairn scan` to update hashes |
| Dogfood | `bash scripts/dogfood.sh` | Fix underlying lint/hook issue |

## Notes

- The script stashes any local changes before checking out the PR and restores them afterward.
- If you need to make multiple fix commits, squash them into one before the final merge: `git rebase -i <base-branch>`.
- Always verify `git status` is clean before merging.
