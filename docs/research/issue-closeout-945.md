# Closeout map for #945 (FM-2 design)

Design ACs are met on main. Implementation children #946/#947 already merged.

| AC | Evidence |
| --- | --- |
| `gaia-spec/rag/embeddings.md` | selection criteria, two surfaces, refuse MiniLM claim |
| `EmbeddingModel` trait | `gaia-ukd::embed` listed + `gaia-ingest::embed` pipeline |
| `MockEmbedder` | `gaia-ukd` unit tests |
| `DocumentChunk.embedding` | `Option<EmbeddingVector>` populated by `IngestPipeline` |
| query embed | `gaia_aikd::embed_query` + `rank_persisted` |

Honest leftover: no downloaded MiniLM / GGUF runtime. `HashingEmbedder` is the offline stand-in (`model_id = hashing-384-offline-v0`).
