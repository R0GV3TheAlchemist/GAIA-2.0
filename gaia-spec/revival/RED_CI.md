# Red-CI rule (#369)

Green ancestor CI does not authorize a tree merge into GAIA-2.0.

| Red | Meaning | After it goes green |
| --- | --- | --- |
| Broken code | Import / borrow / schema fail | Repair *in the ancestor* if we need a module |
| Doctrine without tests | Values only | Still values; no new crate |
| False-when-green | Over-claim | Still refuse (see LEDGER.md) |

Promotion remains: Doctrine → Schema → Code → Tests → Audit → Release.
