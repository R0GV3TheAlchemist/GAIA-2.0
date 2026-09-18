# AI-code and MCP supply-chain controls (#348)

## MCP config lint

Reject: unreviewed publisher/server, `latest` tags, non-digest versions, shell wrappers (`sh -c`, bash), inline secret-bearing env.

## AI-generated code

Assistance provenance is not security assurance. Required later (ops, not claimed live here): protected branches, mandatory review, secret/dependency scanning, lockfiles, minimized CI permissions, pinned workflows, no autonomous merge/deploy/publish.

## Evaluation

`tests/control_plane.rs` is the local corpus. CI should run `cargo test -p gaia-acp`. No external targets.
