# Confirm domains on invoke (#374)

`ControlPlane::invoke` calls `autonomy::gate` before policy. No live network.

| Domain | No receipt |
| --- | --- |
| Money / legal / medical / irreversible / outbound | `ConfirmRequired` |
| Life-safety | `LifeSafetyDenied` |
| Vault path or peer dump | `VaultDumpDenied` |

Evidence: `gaia-acp/tests/confirm_invoke.rs`. Sibling #220 stays open.
