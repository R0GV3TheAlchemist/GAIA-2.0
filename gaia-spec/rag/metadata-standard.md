# GAIA RAG Metadata Standard

> **Status:** Stub — full content is the acceptance criterion for FM-4  
> **Implements:** #906 Tier 1, FM-4 (Metadata Incompleteness)  
> **Depends on:** `gaia-spec/rag/chunking-standard.md` (#909)  
> **Owner:** TBD (FM-4 implementation issue)  

This document will define the **required and optional metadata fields** that
every `DocumentChunk` must carry before it enters the vector index.

It is intentionally a stub at the time of #909 merge.  The FM-4 implementation
issue will replace this stub with the full specification.

---

## Placeholder: Required Fields

At merge time of #909, the minimum required fields are defined in
`gaia-spec/rag/chunking-standard.md` § 6 (Metadata Requirements).

The FM-4 issue will expand this into:

- Required fields taxonomy (structural, provenance, access, domain, quality)
- Field validation rules and allowed value sets
- Metadata completeness scoring algorithm
- Retrieval-time enforcement contract with `gaia-memos`
- Migration guide for chunks ingested before this standard was adopted

---

## Open Questions (to be resolved by FM-4)

- Should `heading_path` be a first-class field on `DocumentChunk` or remain
  in `attributes`?
- What is the minimum completeness score required before a chunk may be
  embedded?  (Proposed: 0.8 on a 0–1 scale.)
- How are multilingual chunks tagged when a document contains mixed languages?
- Should `confidence` be derived automatically from `DataSource` or always
  set explicitly by the ingest adapter?

---

*This stub was created by the #909 PR.  Do not add implementation content
here until the FM-4 issue is opened and assigned.*
