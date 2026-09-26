# Source attribution format (#932)

Listed contract. Not a live citation renderer.

## Inline markers

Grounded prose embeds a marker at sentence or block end:

```text
The treaty was signed in 1992. [source: chunk_id]
```

`chunk_id` is the ingest document-chunk id / `gaia_runtime::ChunkId` string.

## Response contract

`Grounded` mode:

- `source_ids` is non-empty
- every factual sentence should map to at least one id
- `confidence` is declared by the caller; this file does not compute it

`Ungrounded` mode requires an explicit opt-in flag. Empty `source_ids` plus
`grounding_required: true` is `GroundingViolation`.

## Faithfulness stub

`gaia_runtime::score_faithfulness` is lexical overlap only. A fixture that
contradicts its chunks must score below `0.5`. It is not an NLI model.

## Refuse

- No claim that markers prove truth
- No LLM-as-judge in this slice
