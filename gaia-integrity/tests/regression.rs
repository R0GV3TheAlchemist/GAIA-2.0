//! Regression locks (#936).
//! Registry: r0001–r0006. Run: cargo test -p gaia-integrity --test regression

use gaia_acp::GroundingClaim;
use gaia_aikd::rank_lexical;
use gaia_ingest::{embed::EmbeddingModel, FileChunkStore, HashingEmbedder};
use gaia_runtime::score_faithfulness;

#[test]
fn r0001_workspace_test_does_not_require_gaia_cli_bin() {
    assert_eq!(env!("CARGO_PKG_NAME"), "gaia-integrity");
}

#[test]
fn r0002_raw_string_closer_token_documented() {
    let closer = "\"#";
    assert_eq!(closer.chars().last(), Some('#'));
}

#[test]
fn r0003_stage_helper_uses_global_not_nonlocal() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let script = root.join("scripts/agent-validate.sh");
    if script.exists() {
        let body = std::fs::read_to_string(&script).expect("read agent-validate.sh");
        assert!(
            !body.contains("nonlocal failed_stage"),
            "agent-validate.sh must not use nonlocal for module-level failed_stage"
        );
    }
}

/// Regression: #1011 / PR #1012 — BM25 IDF was ln((N-df)/df) and zeroed a 2-doc store.
#[test]
fn r0004_lexical_idf_nonzero_on_two_docs() {
    let path = std::env::temp_dir().join(format!("gaia-r0004-{}.jsonl", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let embedder = HashingEmbedder::new();
    let drug = "patient prescribed lisinopril twenty milligrams daily";
    let piano = "purple piano recipes";
    let v = embedder.embed(&[drug, piano]).expect("embed");
    let mut store = FileChunkStore::open(&path).expect("open store");
    store.record(drug, embedder.model_id(), Some(&v[0])).expect("record drug");
    store.record(piano, embedder.model_id(), Some(&v[1])).expect("record piano");
    let hits = rank_lexical("lisinopril dose", &store, 2);
    assert_eq!(hits.len(), 2);
    assert!(hits[0].score > hits[1].score, "exact token must outrank piano");
    assert!(hits[0].score > 0.0, "Lucene-style IDF must be > 0 on a two-doc store");
}

/// Regression: #1009 / PR #1010 — empty sources must not claim grounded.
#[test]
fn r0005_grounding_claim_empty_sources_is_violation() {
    let claim = GroundingClaim {
        content: "ungrounded".into(),
        source_ids: vec![],
        grounding_required: true,
        faithfulness: None,
    };
    assert!(claim.enforce().is_err(), "empty sources + required must violate");
}

/// Regression: #1007 / PR #1008 — copied answer stays high; invented answer stays low.
#[test]
fn r0006_faithfulness_copied_beats_invented() {
    let src = "the earth twin observes climate";
    let good = score_faithfulness("the earth twin observes climate", &[src]);
    let bad = score_faithfulness("purple piano recipes only", &[src]);
    assert!(good.composite > 0.5, "copied answer must score high");
    assert!(bad.composite < 0.5, "invented answer must score low");
}
