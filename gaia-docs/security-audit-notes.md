# Security audit notes (pre-1.0)

This is a summary of what the code enforces. It is not a third-party audit.

## Enforced in tests today

- Ed25519 signatures on kernel audit records and stored intents
- Unsigned MCP messages and unsigned agent packages are rejected by default
- WASI filesystem and network imports are denied without an explicit grant
- Cloud / egress paths default to denied (session cloud opt-in, sense policy,
  federation residency)

## Not audited

- No external pentest
- No formal verification of the hash chain
- No supply-chain attestation on crates.io downloads
- No signed GitHub Release artifacts
- In-process adapters are not a network TCB

## Until `v1.0.0`

Do not describe this tree as a certified sovereign OS. The primitives exist.
The release tag does not.
