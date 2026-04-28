# Frequently Asked Questions

**Source:** https://www.getcairn.dev/docs/faq
**Captured:** 2026-04-28

## General

**What systems can I model with Cairn?**

"Any engineered system with decomposable structure. Vehicles, robots, satellites, consumer electronics, industrial equipment, IoT devices." The platform targets hardware-software products particularly well.

**Is Cairn a replacement for CAD?**

No. Cairn focuses on system architecture and component relationships, while CAD addresses geometry. These tools complement each other rather than compete.

**How does Cairn compare to Cameo, DOORS, or Rhapsody?**

"Those are enterprise MBSE tools with steep learning curves, high costs, and complex deployment." Cairn differentiates through lighter weight, AI-native design, and browser-based accessibility for individual engineers and small teams.

## AI and Data

**Does AI see my entire model?**

No. AI requests include only necessary context. Typically the selected node, nearby elements, and related entities. Your complete model remains local.

**Is my data used to train AI models?**

"No. Requests to Anthropic's API are not used for training." Data processing generates responses, then is discarded.

**Can I use Cairn offline?**

You can view and manually edit offline. AI features need network access. The system uses "local-data, cloud-compute" architecture.

## Workflow

**How do I collaborate with teammates?**

Export your project file and share it; teammates import into their instances. Real-time collaboration is planned.

**Can I undo AI changes?**

Yes. AI outputs undergo ChangeSet review before applying. You approve or reject proposed modifications.

**What if the AI suggests something wrong?**

Reject it through the ChangeSet pattern. AI proposes; you decide. Wrong suggestions are expected and normal.

**How do I export for documentation or reviews?**

Each lens offers export options. The Export tool creates full snapshots for assembly in your preferred documentation platform.
