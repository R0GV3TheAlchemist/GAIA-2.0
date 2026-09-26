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

## Faithfulness metrics (#1007)

`gaia_runtime::score_faithfulness` returns:

| Field | Definition |
| --- | --- |
| `precision` | response tokens found in sources |
| `recall` | source tokens found in the response |
| `jaccard` | token-set intersection / union |
| `composite` | mean of the three; gate value |

Tokens are ASCII alphanumeric, length > 2, lowercased. A fixture that
contradicts its chunks must have `composite < 0.5`. This is not NLI and
not an LLM-as-judge.

## retrieve_and_cite (#1007)

`gaia_aikd::retrieve_and_cite(query, store, embedder, k, min_score)` ranks
persisted rows, drops hits below `min_score`, and resolves hex ids via
`citations_from_hits`. Empty survivors are `CitationError::Empty`.

## Live lookup (#997)

`gaia_aikd::lookup_citations` checks claimed hex ids against
`gaia_ingest::ChunkStore`. Unknown or malformed ids are `CitationError::UnknownChunk`.
This is still a JSONL / in-memory store, not Qdrant.

## Refuse

- No claim that markers prove truth
- No LLM-as-judge in this slice
