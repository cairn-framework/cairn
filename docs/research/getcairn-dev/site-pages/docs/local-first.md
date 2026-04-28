# Local-First Architecture

**Source:** https://www.getcairn.dev/docs/local-first
**Captured:** 2026-04-28

## Local-First Architecture

Your model data lives on your device. Cairn stores everything in your browser's local database. Nodes, requirements, interfaces, state machines, verification records, all of it.

### Why Local-First

Traditional MBSE tools store your models on vendor servers. This creates problems:

- You need internet access to work
- Your proprietary designs live on someone else's infrastructure
- Subscription lapses can lock you out of your own data
- Export formats are often lossy or proprietary

Cairn inverts this. Your browser is the database. You own the bits.

### What Stays Local

- System hierarchy and node properties
- Requirements and verification records
- Interfaces and signal definitions
- State machines and behavior models
- Decision history (pruned alternatives)
- AI interaction logs and usage metrics

This data never leaves your device unless you explicitly export it.

### What Requires Network

AI features require network access. When you use Command+K to decompose, generate requirements, or ask questions, your request goes to Anthropic's API (Claude) for processing. The AI sees the context needed to respond but doesn't retain your data after the request completes.

Image generation uses Google's Gemini API with the same pattern: request out, response back, no retention. We call this "local-data, cloud-compute."

### Browser Storage

Cairn uses IndexedDB, your browser's built-in database. Data persists across sessions. Storage limits are generous (typically 50%+ of available disk). Data stays sandboxed to this browser on this device.

Clearing browser data will delete your projects. Use Export before clearing, switching browsers, or switching devices.

### Export and Backup

The Export tool creates a snapshot of your project. Standard formats you can store, version control, or share however you want. Export regularly, especially before major changes.
