//! Composite gate + sentence attribution (#1009).

use gaia_ingest::{embed::EmbeddingModel, FileChunkStore, HashingEmbedder};
use gaia_runtime::{attribute_sentences, score_faithfulness};

#[test]
fn low_composite_blocks_invoke() {
    let climate = "the earth twin observes climate";
    let path = std::env::temp_dir().join("gaia-integrity-gate.jsonl");
    let _ = std::fs::remove_file(&path);
    let embedder = HashingEmbedder::new();
    let vecs = embedder.embed(&[climate]).unwrap();
    let mut store = FileChunkStore::open(&path).unwrap();
    store.record(climate, embedder.model_id(), Some(&vecs[0])).unwrap();
    let cited = gaia_aikd::retrieve_and_cite("earth twin climate", &store, &embedder, 1, 0.0)
        .unwrap();
    let sources: Vec<&str> = cited.hits.iter().map(|h| h.text.as_str()).collect();
    let bad = score_faithfulness("The treaty was signed in 1848 on Mars and banned water.", &sources);
    assert!(bad.composite < 0.5);
    let mut plane = gaia_acp::ControlPlane::start(1_700_000_000, "agent-a").unwrap();
    let mut manifest = gaia_acp::CapabilityManifest::local_reader("agent-a", 1_700_000_000);
    let action = gaia_acp::ProposedAction {
        agent_id: "agent-a".into(),
        tool: "local_read".into(),
        method: "call".into(),
        target: "docs/a.md".into(),
        action_class: gaia_acp::ActionClass::LocalRead,
        payload: climate.into(),
        nonce: "nonce-agent-a".into(),
        gateway_id: "gateway-local".into(),
        server_id: "server-local".into(),
        resource_id: "repo-local".into(),
        wants_delegation: false,
    };
    let r = plane.invoke_grounded(
        &mut manifest,
        &action,
        None,
        None,
        gaia_acp::GroundingClaim::required(cited.citation_ids.clone())
            .with_faithfulness(bad.composite, 0.5),
    );
    assert!(!r.executed);
    assert_eq!(r.reason, gaia_acp::ReasonCode::GroundingViolation);
}

#[test]
fn sentence_marker_on_overlap_only() {
    let id = gaia_aikd::id_for_text("The treaty was signed in 1992 in Rio.");
    let sources = vec![(id.clone(), "The treaty was signed in 1992 in Rio.".into())];
    let out = attribute_sentences(
        "The treaty was signed in 1992. Purple pianos exist.",
        &sources,
    );
    assert!(out[0].marked.contains(&format!("[source: {id}]")));
    assert!(out[1].source_ids.is_empty());
}
