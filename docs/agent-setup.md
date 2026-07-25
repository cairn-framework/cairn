# Set Up Cairn in This Repo (Agent Instructions)

**If you are a human:** paste this into your coding agent and press enter:

```text
Read https://raw.githubusercontent.com/cairn-framework/cairn/main/docs/agent-setup.md
and follow it to set up Cairn in this repo. Walk me through each decision.
```

**If you are a coding agent:** you are setting up Cairn, an architecture map
that you will write and maintain for your user. Follow the steps in order.
Confirm each major step with your user before moving on. Do not skip the
review step on an existing codebase.

## What you are installing

Cairn keeps a map of this repo: what the parts are, how they connect, and why.
You author the map (a `cairn.blueprint` file plus markdown artefacts); Cairn
checks it against the real code in both directions and reports every mismatch
as a finding. Everything it writes is plain text in the repo. Uninstalling is
deleting `cairn.blueprint`, `cairn.config.yaml`, `.cairn/`, and the generated
map files.

## Step 1: Install the CLI

Check whether it is already present:

```sh
cairn --version
```

If not, install a prebuilt binary (no toolchain needed). macOS and Linux:

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/cairn-framework/cairn/releases/latest/download/cairn-installer.sh | sh
```

Windows (PowerShell):

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/cairn-framework/cairn/releases/latest/download/cairn-framework-installer.ps1 | iex"
```

Other install paths (Homebrew, crates.io, source) are in
[docs/quickstart.md](quickstart.md).

## Step 2: Detect the repo state

Look at the repository you are in.

- **Existing codebase** (source files already present): follow Step 3A.
- **New or nearly empty project**: follow Step 3B.

## Step 3A: Existing codebase

1. From the repo root, run:

   ```sh
   cairn init --from-code
   ```

   This scans the source tree, discovers module-like directories, and writes
   a reviewable draft: a comment-only `cairn.blueprint` plus a proposal under
   `meta/changes/brownfield-init/`. Nothing is active yet.

2. **Review the draft with your user.** Open the proposal, walk through the
   discovered modules, and fix what the discovery got wrong: rename modules,
   regroup directories, add missing dependency edges, delete spurious nodes.
   This is the step where your user's knowledge of the system gets captured;
   do not rubber-stamp it.

3. Apply the reviewed proposal:

   ```sh
   cairn change apply brownfield-init
   ```

   (The verb is `apply`; it means "apply this change and file it". The `archive`
   form is a supported alias.)

   (When your user asks to skip the review and bootstrap the agent in one step,
   `cairn init --from-code --apply --wire` does steps 1 and 3, installs the
   owned pack, and appends its orientation pointer.)

4. Produce the first map:

   ```sh
   cairn scan
   ```

   Read `map.md`. Every declared part is marked `synced` (matches the code)
   or `ghost` (planned but not built), and files no module claims appear as
   `orphaned` findings. Triage the orphans with your user: `cairn onboard`
   groups the leftover files and suggests where they fit.

## Step 3B: New project

1. From the repo root, run:

   ```sh
   cairn init --wire
   ```

   This creates a starter `cairn.blueprint`, `cairn.config.yaml`, and
   `.cairn/AGENTS.md`. It installs the owned agent pack and appends a pointer to
   the project's agent instructions.

2. Ask your user what they are building, then draft the blueprint from that
   conversation: systems, containers, modules, the promises between them, and
   the decisions behind them. Modules you declare before the code exists show
   up as `ghost` nodes; that is the intended way to plan. The grammar
   reference is [docs/blueprint.md](blueprint.md).

3. Run `cairn scan` and read `map.md`. Build against the ghost nodes; the
   scan confirms each one as it lands.

## Step 4: Wire yourself in

If you followed the review-first existing-code path, run `cairn init --wire`
after applying the proposal. It installs the owned pack through the same
lifecycle engine as `cairn pack install` and appends a pointer to
`.cairn/AGENTS.md` in `AGENTS.md` or `CLAUDE.md`. The operation is idempotent.
If Step 3 already used `--wire`, nothing more is needed.

## Step 5: Wire the gate

The drift gate is one command with real exit codes:

```sh
cairn hook all
```

Only two kinds of finding block: broken structure or a changed interface.
Everything else is advice. Add it where this repo runs checks, in a
pre-commit hook, CI job, or both. Minimal pre-commit example:

```sh
#!/bin/sh
cairn hook all
```

## Step 6: Optional integrations

- **MCP**: if your runtime supports MCP, register `cairn-mcp` so map queries
  become native tool calls. See [docs/mcp.md](mcp.md).
- **Query patterns**: [docs/agent-prompts.md](agent-prompts.md) has prompts
  for orienting, scoping investigations, and pre-commit checks in a
  Cairn-managed repo.

## Step 7: Confirm and hand back

Show your user the first `map.md`, the findings list from `cairn lint`, and
where the gate now runs. From here, maintain the map as part of the work they
ask for: declare new modules before building them, record decisions when you
and your user settle one, and keep the blueprint current.

These instructions are a living procedure. If a step did not work as
described, or the flow fought you, record it:

```sh
cairn feedback "<what you expected, what happened instead>"
```

It writes a note to `.cairn/feedback.md` in the repo and prints a link to
file it upstream. Nothing is sent anywhere on its own.
