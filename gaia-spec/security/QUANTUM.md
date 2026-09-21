# Quantum claims (#389)

Research note. Not a QPU. Not a consciousness theory. Not v1.0.
Claim classes follow `gaia-spec/CLAIM_CLASSES.md`.
Predecessor refuse: quantum/sentience lanes as runtime (`revival/LEDGER.md`).

## Separate the word

| Claim | Class | May bind code? |
| --- | --- | --- |
| Ed25519 is breakable by a large fault-tolerant quantum computer (Shor) | established | Yes, as a *warning* |
| SHA-256 preimage is Grover-weakened (bits, not an instant break) | established | Inventory only |
| Harvest-now-decrypt-later / harvest-now-forge-later on long-lived receipts | established as a threat model | Design |
| NIST FIPS 203 ML-KEM, 204 ML-DSA, 205 SLH-DSA exist | established | Design |
| This repo currently signs with Ed25519 | established | Yes |
| Cryptographically relevant quantum computer exists *today* and can forge our intents | prohibited (false) | Never |
| NISQ devices give GAIA a runtime advantage | experimental / currently false-for-us | No crate |
| Quantum consciousness, Orch-OR, superposition-as-experience, brain–QC interface | prohibited | Never |
| Multiverse or “Willow proves many-worlds” as an OS truth | prohibited | Never |
| “Quantum authority” grants tools or Autonomy 4–5 | prohibited | Never |

## What the field actually is (2026, condensed)

- Hardware is NISQ / early logical-qubit demos. Useful Shor on RSA-2048 is not a production fact.
- Resource estimates for breaking common public-key crypto have fallen versus older papers; that changes *planning*, not tonight’s invoke path.
- HNDL is the present tense danger: store ciphertext or signatures now, break or forge later.
- Agent identity that lives for years should not rest on ECC alone forever.
- Warm, wet neural tissue decoheres many orders of magnitude faster than spike times. “The brain is a quantum computer” is not an engineering premise for this repo.
- Vendor quantum-consciousness and multiverse talk is philosophy or marketing. It is not a syscall.

## Inventory on this repo (#391)

| Site | Algorithm | Quantum note |
| --- | --- | --- |
| `gaia-orchestrator` IntentSigner / MCP tagged signatures | Ed25519 (`ed25519-dalek` v1 in workspace lock) | Shor-vulnerable *eventually* |
| MCP `admit` unsigned / bad-sig codes | `GAIA_UNAUTHENTICATED`, `GAIA_SIGNATURE_INVALID` | Must survive any future dual-sign |
| `gaia-acp` audit chain | SHA-256 | Integrity today; not a signature |
| Live trace rows (#335) | local first; live off by default | Long-lived copies are HNDL-relevant |
| QPU / quantum SDK | none | correct |

## Dual-sign design (#392)

**Status:** Design-only. Zero new Cargo dependencies. No implementation until a bind issue with tests is opened.

`ed25519_required()` MUST return `true` at all times — Ed25519 is never optional.  
`ml_dsa_required_today()` MUST return `false` — ML-DSA is a reserved optional tag in this epoch.  
`quantum_rng_from_http_allowed()` MUST return `false`.

### 1. The HNDL problem and why design starts now

Harvest-now-forge-later means an adversary can record signed intents today and
forge them once a cryptographically relevant quantum computer exists. Agent
identity receipts that persist for years (audit chains, long-lived GAIAN keys)
are the exposure surface. The migration path must be designed before the threat
is live, not after.

### 2. Tag model

An intent signature envelope carries one or two algorithm tags.

```
SignatureEnvelope {
    ed25519_sig:  Bytes,        // required in all epochs
    ml_dsa_sig:   Option<Bytes>, // reserved; absent today
    algorithm:    AlgTag,
}

enum AlgTag {
    Ed25519Only,          // current epoch
    Ed25519AndMlDsa65,    // future epoch (design reserved)
}
```

| Tag | Meaning | Epoch |
|---|---|---|
| `Ed25519Only` | Ed25519 present and valid; no second tag | **Today (required)** |
| `Ed25519AndMlDsa65` | Both Ed25519 and ML-DSA-65 (FIPS 204) present; both must pass | Future — epoch artifact required |

### 3. Verification contract (`intent_signer.rs`)

```
enum VerifyResult {
    Ok,
    Unauthenticated,   // maps to GAIA_UNAUTHENTICATED
    SignatureInvalid,  // maps to GAIA_SIGNATURE_INVALID
    EpochMismatch,     // second tag required by epoch but absent
}
```

`verify(envelope: &SignatureEnvelope, epoch: &Epoch) -> VerifyResult`

| Condition | Result |
|---|---|
| Ed25519 absent | `Unauthenticated` |
| Ed25519 present, invalid | `SignatureInvalid` |
| Ed25519 valid; epoch = `Ed25519Only`; `ml_dsa_sig` absent | `Ok` |
| Ed25519 valid; epoch = `Ed25519Only`; `ml_dsa_sig` present | `Ok` (extra tag ignored — forward compatible) |
| Ed25519 valid; epoch = `Ed25519AndMlDsa65`; `ml_dsa_sig` absent | `EpochMismatch` |
| Ed25519 valid; epoch = `Ed25519AndMlDsa65`; `ml_dsa_sig` present and valid | `Ok` |
| Ed25519 valid; epoch = `Ed25519AndMlDsa65`; `ml_dsa_sig` present but invalid | `SignatureInvalid` |

**Fail-closed rule:** any `VerifyResult` other than `Ok` MUST cause `admit` to
reject the intent. `GAIA_UNAUTHENTICATED` and `GAIA_SIGNATURE_INVALID` MCP
error codes MUST be preserved through any algorithm change.

`mcp_error_codes_preserved()` MUST return `true`.

### 4. IntentSigner contract

`sign(payload: &[u8], key: &Ed25519Key, epoch: &Epoch) -> SignatureEnvelope`

| Rule | Constraint |
|---|---|
| Ed25519 always signed | `sign` MUST always produce a valid `ed25519_sig`; `Err(KeyMissing)` if key absent |
| ML-DSA today | `sign` MUST NOT produce `ml_dsa_sig` in `Ed25519Only` epoch; field is `None` |
| No silent downgrade | If epoch is `Ed25519AndMlDsa65` but ML-DSA key is absent, `sign` MUST return `Err(MlDsaKeyMissing)` rather than silently falling back to Ed25519-only |
| No new crate today | `ml_dsa` or equivalent crate MUST NOT be added to `Cargo.toml` until the bind issue is opened |

### 5. Epoch model

An epoch is a versioned, written artifact — not a runtime config flag.

| Epoch name | Requirement | Activation |
|---|---|---|
| `Ed25519Only` | Ed25519 required; ML-DSA absent or ignored | **Current epoch** |
| `Ed25519AndMlDsa65` | Both required; both must pass | Future — opened by a dedicated bind issue with tests |

Epoch upgrade rules:
- Epoch MUST NOT be changed by a runtime call or environment variable.
- Epoch change MUST be committed as a spec artifact in `gaia-spec/security/` before any code bind.
- `ml_dsa_required_today()` MUST return `false` until the bind issue is merged.

### 6. Entropy prohibition

`quantum_rng_from_http_allowed()` MUST return `false`.

| Prohibited source | Reason |
|---|---|
| Any `https://` URL returning random bytes | Untrusted; unauthenticated; network-dependent |
| A website labelled “quantum random” | Marketing; provides no verifiable entropy source attestation |
| Any non-local entropy for key generation | Key generation MUST use OS CSPRNG (`getrandom` / `/dev/urandom` on Linux) only |

Key generation MUST use the OS-provided CSPRNG. No external entropy API call.

### 7. What this design does not do

- Does not remove or weaken Ed25519 in any epoch.
- Does not add `ml_dsa`, `pqcrypto`, or any PQC crate today.
- Does not make ML-DSA the primary tag — Ed25519 remains required.
- Does not provide a runtime flag to skip the second tag when the dual-sign epoch is active.
- Does not treat a NISQ cloud job or quantum website RNG as a valid entropy source.
- Does not change `GAIA_UNAUTHENTICATED` or `GAIA_SIGNATURE_INVALID` error codes.

### 8. Prohibition surface

| Prohibition | Guard |
|---|---|
| Ed25519 not required | `ed25519_required()` → `true` |
| ML-DSA required today | `ml_dsa_required_today()` → `false` |
| HTTP quantum RNG | `quantum_rng_from_http_allowed()` → `false` |
| MCP error codes changed | `mcp_error_codes_preserved()` → `true` |
| Silent Ed25519-only downgrade in dual-sign epoch | `Err(MlDsaKeyMissing)` |
| New PQC crate before bind issue | `Err` — not in `Cargo.toml` |

### 9. Acceptance gate

- [ ] `ed25519_required()` → `true`
- [ ] `ml_dsa_required_today()` → `false`
- [ ] `quantum_rng_from_http_allowed()` → `false`
- [ ] `mcp_error_codes_preserved()` → `true`
- [ ] `verify` returns `Unauthenticated` when Ed25519 absent
- [ ] `verify` returns `Ok` when Ed25519 valid and epoch is `Ed25519Only` (regardless of `ml_dsa_sig`)
- [ ] `verify` returns `EpochMismatch` when epoch is `Ed25519AndMlDsa65` and `ml_dsa_sig` absent
- [ ] `sign` returns `Err(MlDsaKeyMissing)` in `Ed25519AndMlDsa65` epoch with no ML-DSA key
- [ ] No PQC crate in `Cargo.toml`
- [ ] `cargo test -p gaia-orchestrator` green

## Map to current issues (do not fork)

| Issue | Relation |
| --- | --- |
| #19 / trust | Signatures live here; agility is additive |
| #23 | Unsigned still rejected after any alg change |
| #341 | Policy is still the authorization boundary |
| #335 | Trace rows may outlive ECC |
| #367 | Consciousness claims stay prohibited |
| #383–#387 | Oath/titles/tablet; quantum-title is the same class |

## Prevented mistakes

- Do not add `gaia-quantum`.
- Do not call current Ed25519 post-quantum.
- Do not treat a NISQ cloud job as an allow.
- Do not weaken `refuse_live_supabase()` or MCP `live_wire() == false` because a paper mentioned qubits.
- Do not implement brain-computer or “entangle the operator.”
