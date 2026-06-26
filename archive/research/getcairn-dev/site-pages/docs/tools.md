# Tools Documentation

**Source:** https://www.getcairn.dev/docs/tools
**Captured:** 2026-04-28

## Tools

Cairn offers eight sidebar tools that extend functionality beyond the core lens paradigm:

**Quality**
Automated linter with 16+ rules across nodes, requirements, interfaces, and trace links. Displays health score and actionable findings with source navigation.

**Simulation**
Monte Carlo analysis, state-based simulation, and parameter sweeps using Pyodide (in-browser Python). AI generates scripts from model data; Recharts renders results.

**History**
Full ChangeSet history with operation-level detail. Records every AI edit and manual change with timestamps, specialist identity, and undo capability.

**Usage**
AI cost tracking ledger showing total estimated spend, breakdowns by provider/category/model, recent API calls with token counts, and CSV export. Updates automatically on every API call.

**Traceability**
Trace link explorer displaying satisfies, verifies, derives, and depends_on relationships between requirements, verifications, and nodes.

**Types and Units**
Unit system management with SI base units, derived units, and custom type definitions for interface signals and properties.

**Assets**
Asset browser for all generated files. Images, meshes, scripts, simulation results. Stored locally in IndexedDB via Dexie.js.

**Settings**
API key configuration (Anthropic, Gemini), model preferences, project metadata, naming conventions, and export options.
