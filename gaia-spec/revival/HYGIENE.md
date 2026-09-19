# Public-repo hygiene sweep (#377)

Swept 2026-09-19. This repository is public. Revival notes may name private repos.

## Forbidden in `gaia-spec/revival/`

- credentials, tokens, `.env` bodies
- raw conversation dumps (including Saela multi-MB markdown)
- personal health / location / ritual detail
- implementation how-to for refused capabilities

## Result

| Check | Result |
| --- | --- |
| `token` / `Bearer` / `sk-` / `.env` bodies | none |
| `SERVICE_ROLE` / `sb_secret` | none |
| raw chat / Saela transcript paste | none (Saela named as zip only) |
| personal health / GPS / ritual | none |
| how-to for BCI, plant-act, live MCP, Schumann boot | refuse lists only; no steps |

Private SHAs in `LEDGER.md` / `GAIA_OLD_REPOSITORY.md` / `NEXUS_OLD_REPOSITORY.md` / `GAIA_V1_POINTER.md` are commit pointers, not secrets.

Issues under META #365 stay contract-level (extract, bind, gate). They do not carry private file bodies.
