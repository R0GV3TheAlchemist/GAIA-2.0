//! Document freshness scoring and TTL-based staleness evaluation.
//!
//! Every [`DocumentChunk`] optionally carries a `ttl_seconds` field that
//! defines how long the chunk's content is considered authoritative.  This
//! module provides the logic to evaluate that TTL and compute a linear
//! freshness score.
//!
//! ## TTL defaults by document type
//!
//! | Document type      | Default TTL         |
//! |--------------------|---------------------|
//! | Canon tablets      | `None` (never stale)|
//! | Research documents | 90 days             |
//! | Memory summaries   | 30 days             |
//!
//! See `gaia-spec/rag/freshness.md` for the full rationale.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gaia_ingest::freshness::{evaluate, FreshnessVerdict};
//!
//! let verdict = evaluate(&chunk, ingested_at_unix, now_unix);
//! if verdict == FreshnessVerdict::Stale {
//!     // surface [STALE] flag to retrieval caller
//! }
//! ```

// ── TTL constants ─────────────────────────────────────────────────────────────

/// Canon tablets never expire — they are the authoritative source of truth.
pub const CANON_TTL_SECONDS: Option<u64> = None;

/// Research documents are considered fresh for 90 days.
pub const RESEARCH_TTL_SECONDS: u64 = 90 * 24 * 60 * 60;

/// Memory summaries are considered fresh for 30 days.
pub const MEMORY_TTL_SECONDS: u64 = 30 * 24 * 60 * 60;

// ── FreshnessVerdict ──────────────────────────────────────────────────────────

/// The staleness evaluation result for a single chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreshnessVerdict {
    /// The chunk is within its TTL window — safe to serve.
    Fresh,
    /// The chunk has exceeded its TTL — caller should surface `[STALE]`.
    Stale,
    /// No TTL was set on this chunk — treated as perpetually fresh.
    NoTtl,
}

// ── Core functions ────────────────────────────────────────────────────────────

/// Evaluate whether a chunk is stale given the Unix timestamps for when it
/// was ingested and the current wall-clock time.
///
/// - `ttl_seconds`: the chunk's TTL, taken from `DocumentChunk::ttl_seconds`.
/// - `ingested_at_unix`: when the chunk was first stored (seconds since epoch).
/// - `now_unix`: current time (seconds since epoch).
///
/// Returns [`FreshnessVerdict::NoTtl`] when `ttl_seconds` is `None`.
pub fn evaluate(
    ttl_seconds: Option<u64>,
    ingested_at_unix: u64,
    now_unix: u64,
) -> FreshnessVerdict {
    match ttl_seconds {
        None => FreshnessVerdict::NoTtl,
        Some(ttl) => {
            let age = now_unix.saturating_sub(ingested_at_unix);
            if age > ttl {
                FreshnessVerdict::Stale
            } else {
                FreshnessVerdict::Fresh
            }
        }
    }
}

/// Compute a freshness score in `[0.0, 1.0]`.
///
/// - `1.0` means just ingested (age = 0).
/// - `0.0` means TTL has expired (age ≥ TTL).
/// - When no TTL is set the score is always `1.0`.
///
/// The decay is linear: `score = 1.0 - (age / ttl).clamp(0.0, 1.0)`.
pub fn freshness_score(ttl_seconds: Option<u64>, ingested_at_unix: u64, now_unix: u64) -> f32 {
    match ttl_seconds {
        None => 1.0,
        Some(ttl) if ttl == 0 => 0.0,
        Some(ttl) => {
            let age = now_unix.saturating_sub(ingested_at_unix) as f32;
            let ttl_f = ttl as f32;
            (1.0 - age / ttl_f).clamp(0.0, 1.0)
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: u64 = 1_000_000;
    const TTL: u64 = 3_600; // 1 hour

    #[test]
    fn no_ttl_is_always_fresh() {
        assert_eq!(evaluate(None, 0, NOW), FreshnessVerdict::NoTtl);
    }

    #[test]
    fn within_ttl_is_fresh() {
        let ingested = NOW - TTL / 2; // half TTL ago
        assert_eq!(evaluate(Some(TTL), ingested, NOW), FreshnessVerdict::Fresh);
    }

    #[test]
    fn expired_ttl_is_stale() {
        let ingested = NOW - TTL - 1; // one second past TTL
        assert_eq!(evaluate(Some(TTL), ingested, NOW), FreshnessVerdict::Stale);
    }

    #[test]
    fn exactly_at_ttl_boundary_is_fresh() {
        let ingested = NOW - TTL;
        // age == ttl → not strictly greater, so Fresh
        assert_eq!(evaluate(Some(TTL), ingested, NOW), FreshnessVerdict::Fresh);
    }

    #[test]
    fn score_just_ingested_is_one() {
        let score = freshness_score(Some(TTL), NOW, NOW);
        assert!((score - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn score_expired_is_zero() {
        let ingested = NOW - TTL * 2;
        let score = freshness_score(Some(TTL), ingested, NOW);
        assert!((score - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn score_halfway_is_half() {
        let ingested = NOW - TTL / 2;
        let score = freshness_score(Some(TTL), ingested, NOW);
        assert!((score - 0.5).abs() < 1e-5);
    }

    #[test]
    fn score_no_ttl_is_one() {
        assert!((freshness_score(None, 0, NOW) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn research_ttl_constant_is_90_days() {
        assert_eq!(RESEARCH_TTL_SECONDS, 90 * 24 * 60 * 60);
    }

    #[test]
    fn memory_ttl_constant_is_30_days() {
        assert_eq!(MEMORY_TTL_SECONDS, 30 * 24 * 60 * 60);
    }
}
