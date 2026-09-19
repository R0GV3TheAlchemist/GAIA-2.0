# Old crate names → this repo (#371)

Source workspace: private `GAIA-Old-Repository` (`gaia-spec/revival/GAIA_OLD_REPOSITORY.md`).
This file is the extract. It is not a grant to copy those crates.

Gap column:

- `none` — current crate already covers the contract
- `spec` — name maps; remaining work is an existing issue
- `code` — local code exists; do not add a second crate
- `refuse` — do not rebuild the ancestor crate

Do not fork #220 / #187 / #335. #341 is closed by owner; ACP stays local-fake-MCP.

## Map

| Old crate | Current home | Gap | Notes |
| --- | --- | --- | --- |
| gaia-policy | `gaia-acp` `policy.rs` | none | default-deny engine already here |
| gaia-vault | `gaia-acp` `PeerEnvelope` + confirm domains | none | dump between peers is denied; **no vault crate copy** |
| gaia-memory | `gaia-memos` + kernel `AuditLog` | spec | listed memory-tier; raw memory not a peer payload |
| gaia-contracts | `gaia-spec` + `gaia-acp` `types.rs` | none | receipts / reason codes already typed |
| gaia-crypto | `gaia-kernel` identity (Ed25519) | none | ML-DSA stays #392 design-only |
| gaia-core | `gaia-kernel` + `gaia-sos` | spec | kernel is the core; no second core crate |
| gaia-session | `gaia-interface` | spec | session language only; not a net session product |
| gaia-server | `gaia-interface` + `gaia-orchestrator` | spec | no public HTTP control plane |
| gaia-fs | `gaia-orchestrator` github-source allowlists + `gaia-acp` sandbox | spec | path allowlists; no host FS product |
| gaia-net | `gaia-acp` sandbox egress | code | default-deny; fake destinations only |
| gaia-scheduler | `gaia-orchestrator` broker / DAG | spec | local queue; no federated carbon fleet |
| gaia-notify | — | refuse | no pager / SLA (#335 non-goal) |
| gaia-power | — | refuse | not an actuator or power-management crate |
| gaia-ffi | — | refuse | no second Tauri/desktop shell next to `gaia-interface` |

Machine table: `crate-map.csv`.

## Law

- Ancestor crate name is a pointer, not a member of this workspace.
- Green ancestor CI does not make a copy mergeable (`RED_CI.md`, #369).
- Catalog / vault / memory rows are not capability grants.
