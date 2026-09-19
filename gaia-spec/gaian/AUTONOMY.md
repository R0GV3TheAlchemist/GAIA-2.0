# Autonomy, confirm, envelope (#220)

Enforced in `gaia-acp` (`autonomy.rs`, `ControlPlane::invoke`). Default level is Suggest (1).
Levels 4–5 are named refuse in software.

| Level | Name | Software |
| --- | --- | --- |
| 0 | Observe | yes |
| 1 | Suggest (default) | yes |
| 2 | LocalAct | yes |
| 3 | BoundedRemediate | yes (max) |
| 4 | PlantAct | refuse |
| 5 | Unbounded | refuse |

At level 1, repo write / external write / egress / publish / destructive → `AutonomyCap`.

## Confirm domains (no receipt)

| Domain | Code |
| --- | --- |
| Money, legal, medical, irreversible, outbound | `ConfirmRequired` |
| Life-safety | `LifeSafetyDenied` |
| Vault | `VaultDumpDenied` |

## Envelope

Fields: `did`, `intent_id`, `purpose`, `raw_memory=false`, `specialist`.

- Raw memory or purpose `dump-vault` → `VaultDumpDenied`
- Specialist purpose containing `command` / `peer-direct` / `dump` → `CrossAgent`
- Specialist may `report-core` only

Schema: `envelope.schema.json`. Parent #216 stays open. #72 stays the mailbox protocol.
