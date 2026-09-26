# Source attribution format (#932)

Listed contract. Not a live citation renderer.

## Inline markers

Grounded prose embeds a marker at sentence or block end:

```text
The treaty was signed in 1992. [source: chunk_id]
```

`chunk_id` is the ingest document-chunk id / `ChunkId` hex from `gaia_ingest`.

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

## Live lookup (#997)

`gaia_aikd::lookup_citations` checks claimed hex ids against
`gaia_ingest::ChunkStore`. Unknown or malformed ids are `CitationError::UnknownChunk`.
`RetrievedChunk.chunk_id` is `ChunkId::from_text` hex, not a free-form label.
This is still an in-memory store, not Qdrant.

## Refuse

- No claim that markers prove truth
- No LLM-as-judge in this slice
