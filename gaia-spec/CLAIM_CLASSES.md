# Claim classes (#367)

One page. Does not replace #172 AIMD humility / prohibited-magic charter.
Does not replace `gaia-spec/revival/LEDGER.md` (#366).

A claim is a sentence this repo might treat as true at runtime.
Every such sentence has exactly one class.

| Class | May bind code? | May appear as an allow in local audit? |
| --- | --- | --- |
| `established` | Yes, with a test on `main` | Yes |
| `experimental` | Scoring / research notes only | Only as deny or research tag |
| `symbolic` | UI / operator language only | No |
| `prohibited` | Never as runtime truth | Never as allow |

## Established (examples)

- Default autonomy is Suggest (level 1). Levels 4–5 are refused in software.
- Money, legal, medical, irreversible, and outbound actions need an exact receipt.
- Peer envelope must not carry vault or raw memory.
- Local invoke / kill emit a typed `TraceEvent`. Live Supabase write is not implied.
- MCP tool descriptions are untrusted input.

## Experimental (examples)

- Claim-validation CI on spec tags (#370).
- Claim-class field on local `TraceEvent` (#375).
- Knowledge-ID mapped-vs-listed ledger (#368).

## Symbolic (examples)

- Geometry names (Circle, Vesica, Compass) as operator language.
- “Twin” as a governed record label, not a person.
- Predecessor ethic / naming text.

## Prohibited (examples)

- Planetary consciousness or sentience as runtime state.
- Live Earth-cavity / Schumann measurement as a boot truth.
- Autonomy levels 4–5 implemented in software without a written artifact.
- Universal multi-paradigm language or second kernel as a crate.
- Catalog row treated as a capability grant.

Unclassified claims are treated as `experimental` until tagged. Promotion to `established` requires schema or code plus a test. See #378.

Parent: #365. Next: #368 knowledge IDs.
