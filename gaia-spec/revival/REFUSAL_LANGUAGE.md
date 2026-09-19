# Discernment as refusal language (#372)

NEXUS-Old `DISCERNMENT_PROTOCOL` and `COEXISTENCE_LAWS` stay doctrine.
This extract lifts **verbs onto existing `ReasonCode`**. It does not add a runtime module named discernment, coexistence, harmonic, or dimensional.

Audit payloads use `ReasonCode::as_str()` only (`GAIA_ACP_*`). Operator prose may use predecessor words in docs. Those words must not appear in receipt `reason` fields.

## Mapped verbs → existing codes

| Predecessor verb (doctrine) | Existing `ReasonCode` | Wire string |
| --- | --- | --- |
| confirm first / ask before act | `ConfirmRequired` | `GAIA_ACP_CONFIRM_REQUIRED` |
| cap autonomy / stay at suggest | `AutonomyCap` | `GAIA_ACP_AUTONOMY_CAP` |
| no vault / no raw memory dump | `VaultDumpDenied` | `GAIA_ACP_VAULT_DUMP_DENIED` |
| life-safety hold | `LifeSafetyDenied` | `GAIA_ACP_LIFE_SAFETY_DENIED` |
| policy no / default no | `DefaultDeny` | `GAIA_ACP_DEFAULT_DENY` |
| stop / kill | `EmergencyStop` | `GAIA_ACP_EMERGENCY_STOP` |
| untrusted text cannot grant | `UntrustedAuthority` | `GAIA_ACP_UNTRUSTED_AUTHORITY` |
| not on the list | `ToolNotListed` | `GAIA_ACP_TOOL_NOT_LISTED` |
| path off allowlist | `PathDenied` | `GAIA_ACP_PATH_DENIED` |
| host off allowlist | `HostDenied` | `GAIA_ACP_HOST_DENIED` |
| leave the machine | `EgressDenied` | `GAIA_ACP_EGRESS_DENIED` |
| replayed blessing | `ApprovalReplay` | `GAIA_ACP_APPROVAL_REPLAY` |
| expired blessing | `ApprovalExpired` | `GAIA_ACP_APPROVAL_EXPIRED` |
| peer cannot command peer | `CrossAgent` | `GAIA_ACP_CROSS_AGENT` |
| identity mint | `IdentityCreateDenied` | `GAIA_ACP_IDENTITY_CREATE_DENIED` |
| secret ride-along | `SecretDenied` | `GAIA_ACP_SECRET_DENIED` |
| tier 4–5 act | `TierForbidden` | `GAIA_ACP_TIER_FORBIDDEN` |

There is no `PolicyDenied` variant. Use `DefaultDeny`.
There is no `Discernment*` or `Coexistence*` variant. Do not add one.

## Refused as codes (stay symbolic / docs)

| Predecessor phrase | Disposition |
| --- | --- |
| discernment protocol | docs only |
| coexistence laws | docs only |
| harmonic alignment | refuse as reason |
| dimensional OS / plane | refuse as reason |
| Schumann / 7.83 Hz authority | refuse as reason |
| consciousness / sentience state | refuse as reason |
| twin-as-person | refuse as reason |

Machine table: `refusal-map.csv`.

## Law

- Candidate names that already exist are reused.
- New metaphysics-shaped modules are out of scope.
- Receipt `reason` is a stable code, not a doctrine paragraph.
