# gaia-acp

Local-first agent control plane for GAIA 2.0. Implements the #341 program and children #342–#348 with a fake MCP adapter only.

```text
Signed intent -> capability manifest -> policy engine
  -> exact human approval when required
  -> gateway (sole invoke path)
  -> fake adapter (no network)
  -> hash-chained action receipt
```

```bash
cargo test -p gaia-acp
```
