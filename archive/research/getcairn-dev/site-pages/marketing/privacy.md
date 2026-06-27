# Privacy Policy

**Source:** https://www.getcairn.dev/privacy
**Captured:** 2026-04-28

Effective April 2026. Last updated April 2026.

Cairn is built by a solo developer who believes in straightforward data practices. This policy explains what data Cairn collects, why, where it goes, and how you control it.

## The Short Version

- Your engineering models live on your device by default. Cairn never sees them unless you enable cloud sync.
- When you use AI features, your prompts and model context are sent to third-party AI providers for processing.
- Your account email and usage metrics are stored in our database for authentication and billing.
- We don't sell your data. We don't run ads.

## What Cairn Collects

### Account Information

When you create an account, Cairn stores your email address (for authentication via magic link sign-in), a display name (derived from your email, editable by you), and your account tier and usage metrics. This data is stored in Supabase, hosted in the United States.

### Project Metadata

When you create a project, Cairn registers a lightweight record: project name, node count, and timestamps. Your actual model data stays in your browser's local storage (IndexedDB) unless you explicitly enable cloud sync.

### Model Data (Cloud Sync)

Cloud sync is **off by default** and enabled per project. When enabled, Cairn uploads project snapshots (JSON), binary assets, and ChangeSets to Supabase. If you never enable it, your model data never leaves your browser.

### AI Feature Usage

When you use AI features, context from your model is sent to **Anthropic (Claude)** for processing. For image generation, text prompts are sent to **Google (Gemini)**. Cairn logs token counts and estimated costs for billing.

**BYOK users:** Your prompts go directly from your browser to the AI provider. They never pass through Cairn's servers. Your API keys stay in your browser's local storage.

### Analytics

Cairn uses PostHog for analytics on public pages (page views, sign-in events). No model data or project content is sent to analytics. You can opt out via Do Not Track.

## Where Your Data Lives

| Data | Location | Access |
|------|----------|--------|
| Account email, profile | Supabase (US) | You, Cairn admin |
| Model data (local) | Your browser | Only you |
| Model data (cloud sync) | Supabase (US) | You, Cairn admin |
| AI prompts (managed) | Anthropic/Google | Transient processing |
| AI prompts (BYOK) | Direct to provider | You and provider |

## Data Ownership

**You own your models.** Your system models, requirements, interfaces, designs, and all project content belong to you. You can export everything at any time via the built-in .cairn.zip export.

## Data Retention and Deletion

Contact [greg@getcairn.dev](mailto:greg@getcairn.dev) to request account deletion. All associated data (profile, projects, cloud data, usage logs) will be removed. Local browser data is under your control.

## Third-Party Services

| Service | Purpose |
|---------|---------|
| Supabase | Authentication, database, storage |
| Anthropic | AI model processing (Claude) |
| Google | AI image generation (Gemini) |
| Vercel | Hosting, Edge Functions |
| Resend | Transactional email |
| PostHog | Product analytics |

## Security

All data in transit is encrypted via TLS. Database access is protected by row-level security. Server-side API keys are stored as environment variables. BYOK keys never leave your browser. If you discover a vulnerability, contact [greg@getcairn.dev](mailto:greg@getcairn.dev).

## Contact

For privacy questions, data requests, or account deletion: [greg@getcairn.dev](mailto:greg@getcairn.dev)

This privacy policy is written to be read by humans, not lawyers. If something is unclear, ask.
