# EmbeddingModel — FM-2 trait contract and selection criteria

Issue: [#945](https://github.com/R0GV3TheAvatar/GAIA-2.0/issues/945)
Implementation child: [#946](https://github.com/R0GV3TheAvatar/GAIA-2.0/issues/946) (`gaia-ukd`)
Wiring child: [#947](https://github.com/R0GV3TheAvatar/GAIA-2.0/issues/947) (`gaia-ingest` / `gaia-aikd`)
Parent: [#906](https://github.com/R0GV3TheAvatar/GAIA-2.0/issues/906)

This file is the listed contract. It does not download a model. It does not
stand up a vector database. Offline sovereignty still applies.

## Trait contract (`gaia-ukd::embed`)

```text
EmbeddingModel: Send + Sync
  dimensions() -> usize
  embed(text) -> Result<Vec<f32>, EmbeddingError>   # unit-normalised
  embed_batch(texts) -> Result<Vec<Vec<f32>>, EmbeddingError>
  model_id() -> &str
```

Empty input is an error. Zero-dimension models are an error.
`MockEmbedder` is the test stand-in. It is not a production retrieval model.

## Selection criteria

| Criterion | Requirement |
| --- | --- |
| Dimensionality | 384–1536; 768 preferred |
| Licence | Apache-2.0 or MIT |
| Offline | Must run without network |
| Multilingual | Desirable, not blocking for Tier 1 |
| Benchmark floor | MTEB Retrieval average ≥ 50.0 |
| Runtime | GGUF / ONNX preferred |

Recommended baseline stand-in: `all-MiniLM-L6-v2` (384-dim, Apache-2.0).
The mock reports `model_id = mock-minilm-standin`.

## Wiring contract (not implemented in #946)

- chunk → embed lives in `gaia-ingest` after `DocumentChunk.embedding` (#948 field already exists)
- query → embed lives in `gaia-aikd` retrieval before vector comparison (#947)
- provenance must record `model_id` + dimensions

## Refuse

- No cloud embedding API in Tier 1
- No claim that mock vectors are semantic
- No sentient or planetary embedding space
