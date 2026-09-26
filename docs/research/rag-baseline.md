# RAG Tier 1 evidence matrix (#908)

Gate document required by #906. Updated 2026-09-26 at `a6ef0a21`.

Status: `implemented` | `partial` | `absent`

## Current matrix

| FM | Name | Status | Evidence on main |
| --- | --- | --- | --- |
| FM-1 | Fragmented chunks | partial | `gaia-ingest` `DocumentChunk` + markdown `from_path` |
| FM-2 | Weak embeddings | partial | `EmbeddingModel`, `HashingEmbedder` 384-dim. Not MiniLM |
| FM-3 | Irrelevant chunks | partial | `RankedHit::filter`, `mmr_select` |
| FM-4 | Metadata filtering | partial | ingest `ChunkMetadata` + retrieval filter |
| FM-5 | Outdated knowledge | partial | ingest `freshness.rs` timestamps; no live re-embed |
| FM-6 | Query understanding | partial | `expand_query` synonym table. Not HyDE |
| FM-7 | Conflicting sources | partial | `flag_hit_conflicts` |
| FM-8 | Content overload | partial | `budget_hits` |
| FM-9 | Missing citations | partial | live hex lookup + sentence markers |
| FM-10 | Unrestricted generation | partial | `GroundingClaim` + composite gate |
| FM-11 | Confidence threshold | partial | `retrieval_confidence`. Not calibrated |
| FM-12 | Document structure | partial | markdown ingest keeps headings as text |
| FM-13 | Poor preprocessing | partial | ingest dedup + pipeline |
| FM-14 | No RAG evolution | absent | no feedback loop |
| FM-15 | Single-stage retrieval | partial | hybrid RRF; not multi-hop |

## Honest refuse

Not Qdrant. Not MiniLM. Not an LLM judge. No sentient runtime.
