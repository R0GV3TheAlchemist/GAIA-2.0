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

## Dual-sign design only (#392)

Do **not** rip Ed25519 in one PR.

1. Keep Ed25519 as the current required tag.
2. Optional second tag reserved for ML-DSA-65 (FIPS 204) later.
3. Verify: if second tag present, both must pass; if absent, Ed25519 alone still admits (today).
4. Later epoch may require both. That epoch is a written artifact, not this page.
5. Refuse “quantum RNG from a public HTTP API” as entropy.
6. No new Cargo dependency until a bind issue with tests.

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
