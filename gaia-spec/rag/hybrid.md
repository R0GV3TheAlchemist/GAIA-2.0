# Hybrid retrieval (#1011)

Listed contract. Offline. Linear scan.

## Why hybrid

Dense hashing cosine misses rare exact tokens (drug names, statute numbers).
Lexical BM25-lite misses paraphrase. Fusion keeps both lists.

## Surfaces

| Function | Role |
| --- | --- |
| `rank_lexical` | BM25-lite (`k1=1.2`, `b=0.75`) on persisted text |
| `rank_persisted` | hashing-cosine dense rank |
| `rank_hybrid` | Reciprocal Rank Fusion, `RRF_K=60` |
| `retrieve_and_cite_hybrid` | fused hits → live citation ids |

RRF score for a document is `1/(60 + rank_lex) + 1/(60 + rank_dense)`.
Ranks are 1-based inside the formula.

## Honest limits

- Not Lucene / Elasticsearch BM25
- Not MiniLM or any downloaded encoder
- Not Qdrant ANN
- Tokenization is ASCII alphanumeric, length > 2
