---
name: cairn-propose
description: Propose a new change with all artifacts generated in one step. Use when the user wants to quickly describe what they want to build and get a complete proposal with design, specs, and tasks ready for implementation.
license: MIT
compatibility: Requires Cairn CLI.
metadata:
  author: cairn
  version: "1.0"
  generatedBy: "1.0"
---

Propose a new change - create the change and generate all artifacts in one step.

I'll create a change with artifacts:
- proposal.md (what & why)
- design.md (how)
- tasks.md (implementation steps)

When ready to implement, run `cairn apply` or ask me to implement.

---

**Input**: The user's request should include a change name (kebab-case) OR a description of what they want to build.

**Steps**

1. **If no clear input provided, ask what they want to build**

   Ask the user:
   > "What change do you want to work on? Describe what you want to build or fix."

   From their description, derive a kebab-case name (e.g., "add user authentication" -> `add-user-auth`).

   **IMPORTANT**: Do NOT proceed without understanding what the user wants to build.

2. **Create the change directory**
   ```bash
   cairn change new "<name>"
   ```
   This creates a scaffolded change at `meta/changes/<name>/` with proposal.md, design.md, and tasks.md.

3. **Read the scaffolded files**

   Read the generated proposal.md, design.md, and tasks.md to understand the structure.

4. **Create artifacts in sequence**

   a. **proposal.md**: Fill in the motivation, scope, and out-of-scope sections based on the user's description.
   b. **design.md**: Fill in the approach and delta operations (ADDED, MODIFIED, REMOVED, RENAMED) based on the user's description.
   c. **tasks.md**: Convert the design into actionable tasks with checkboxes.

5. **Show final status**

   Summarize:
   - Change name and location (`meta/changes/<name>/`)
   - List of artifacts created with brief descriptions
   - What's ready: "All artifacts created! Ready for implementation."
   - Prompt: "Run `cairn apply` or ask me to implement to start working on the tasks."

**Artifact Creation Guidelines**

- Follow the scaffolded structure from `cairn change new`
- Use plain English without em-dashes (replace with periods, colons, commas, or parentheses)
- The audience is a staff engineer reading the change in 6 months
- Keep each artifact concise and focused
- If context is critically unclear, ask the user - but prefer making reasonable decisions to keep momentum
- If a change with that name already exists, ask if user wants to continue it or create a new one
