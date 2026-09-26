# EmbeddingModel — FM-2 trait contract and selection criteria

Issue: [#945](https://github.com/R0GV3TheAvatar/GAIA-2.0/issues/945)
Listed trait child: [#946](https://github.com/R0GV3TheAvatar/GAIA-2.0/issues/946) (`gaia-ukd`)
Wiring child: [#947](https://github.com/R0GV3TheAvatar/GAIA-2.0/issues/947) (`gaia-ingest` / `gaia-aikd`)
Parent: [#906](https://github.com/R0GV3TheAvatar/GAIA-2.0/issues/906)

This file is the listed contract. It does not download a model. It does not
stand up a vector database. Offline sovereignty still applies.

## Two surfaces (honest split)

GAIA currently ships two embedding surfaces. They are related but not identical.

| Surface | Crate | Role |
| --- | --- | --- |
| Listed design trait | `gaia-ukd::embed` | `EmbeddingModel` + `MockEmbedder` from #946 |
| Pipeline wiring trait | `gaia-ingest::embed` | `EmbeddingModel` + `PassthroughEmbedder` used by ingest/retrieval |

The pipeline trait is what `IngestPipeline` and `gaia_aikd::embed_query` call.
The listed `gaia-ukd` trait remains the design-gate stand-in. A future
`gaia-embed` adapter crate should implement both or collapse them.

## Pipeline trait contract (`gaia-ingest::embed`)

```text
EmbeddingModel: Debug + Send + Sync
  embed(&[&str]) -> Result<Vec<EmbeddingVector>, EmbedError>
  dim() -> Option<usize>          # None = unknown until first call
  model_id() -> &str
```

Rules:
- `embed(&[])` returns `EmbedError::EmptyInput`.
- `embed` must return exactly `texts.len()` vectors, same order.
- `EmbeddingVector::new` rejects zero-dimension vectors.
- `PassthroughEmbedder` is the CI/test stub (`model_id = passthrough-stub-v0`).
  It is not a production retrieval model and is not semantic.

## Listed trait contract (`gaia-ukd::embed`)

```text
EmbeddingModel: Send + Sync
  dimensions() -> usize
  embed(text) -> Result<Vec<f32>, EmbeddingError>   # unit-normalised
  embed_batch(texts) -> Result<Vec<Vec<f32>>, EmbeddingError>
  model_id() -> &str
```

`MockEmbedder` reports `model_id = mock-minilm-standin`.

## Selection criteria

| Criterion | Requirement |
| --- | --- |
| Dimensionality | 384–1536; 768 preferred |
| Licence | Apache-2.0 or MIT |
| Offline | Must run without network |
| Multilingual | Desirable, not blocking for Tier 1 |
| Benchmark floor | MTEB Retrieval average ≥ 50.0 |
| Runtime | GGUF / ONNX preferred |

Recommended production baseline: `all-MiniLM-L6-v2` (384-dim, Apache-2.0).
No production adapter is wired yet.

## Wiring contract (#947)

- chunk → embed lives in `gaia-ingest::IngestPipeline` after chunking.
  `DocumentChunk.embedding` stays `None` when no embedder is configured.
- query → embed lives in `gaia_aikd::embed_query`.
- `RetrievedChunk.embedding` carries an optional vector.
- `GenerationContext::build_with_embeddings` passes stored vectors through.
- Provenance should record `model_id` + dimensions when a real adapter ships.

## Scaffold status (2026-09-25)

Design AC from #945 is satisfied on main via #946 / #947 / #981 plus this
file. Remaining work is a production offline embedder, not another trait.
This batch does not download MiniLM.

## Refuse

- No cloud embedding API in Tier 1
- No claim that mock or passthrough vectors are semantic
- No sentient or planetary embedding space
