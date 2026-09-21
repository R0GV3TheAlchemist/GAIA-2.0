# gaia-ui

React + Vite + TypeScript front-end for the GAIA sovereign runtime.
Connects to `gaia-gateway` (default `http://localhost:7700`) via a Vite
development proxy.

## Quick start

```bash
# Install dependencies
npm install

# Start gaia-gateway in another terminal
cargo run -p gaia-gateway

# Start the dev server (http://localhost:3000)
npm run dev
```

## Tabs

| Tab | Description |
|---|---|
| **Intent Console** | Submit intents and watch live SSE traces from `/intent/stream` |
| **Agents** | View all running agents; revoke any agent via `DELETE /agents/:id/revoke` |
| **Studio** | Compose agent manifests (TOML) and deploy to the runtime |

## Scripts

| Script | Description |
|---|---|
| `npm run dev` | Start Vite dev server on port 3000 |
| `npm run build` | TypeScript compile + Vite production build |
| `npm test` | Run Vitest unit tests |
| `npm run lint` | TypeScript type-check (`tsc --noEmit`) |

## Gateway API surface used

| Method | Path | Used by |
|---|---|---|
| `POST` | `/intent` | IntentConsole — submit |
| `GET` | `/intent/stream` | IntentConsole — SSE live trace |
| `GET` | `/agents` | AgentMatrix — list |
| `DELETE` | `/agents/:id/revoke` | AgentMatrix — revoke |
| `POST` | `/agents` | Studio — deploy |

## Accessibility

- Semantic HTML throughout (`<header>`, `<main>`, `<nav>`, `<section>`, `<table>`)
- All interactive elements keyboard-reachable; visible `:focus-visible` ring
- `aria-live="polite"` on the trace log for screen reader announcements
- `aria-label` on all icon-only and ambiguous buttons
- `prefers-reduced-motion` respected via CSS
- Colour contrast: all text tokens verified WCAG AA (4.5:1) on dark surfaces
- See [`../gaia-interface/ACCESSIBILITY.md`](../gaia-interface/ACCESSIBILITY.md) for full audit notes
